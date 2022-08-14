#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

extern crate crypto;

use crypto::aead::AeadEncryptor;
use futures_util::{SinkExt, StreamExt};
use tokio::io::AsyncWriteExt;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
use crypto::{aes, aes_gcm, blockmodes, buffer, symmetriccipher};

use dotenv;
use std::env;
use std::fs;
use std::iter::repeat;
use std::iter::Iterator;

fn bundle(data: Vec<u8>) -> (Vec<u8>, Vec<u8>) {
    dotenv::dotenv().ok();
    let aes_key = dotenv::var("AES_KEY").unwrap();
    let iv = dotenv::var("IV").unwrap();
    let aad = dotenv::var("AAD").unwrap();

    let mut cipher = aes_gcm::AesGcm::new(
        aes::KeySize::KeySize128,
        &aes_key[..].as_bytes(),
        &iv[..].as_bytes(),
        &aad[..].as_bytes(),
    );
    let mut output: Vec<u8> = repeat(0).take(data.len()).collect();
    let mut tag: Vec<u8> = repeat(0).take(16).collect();

    cipher.encrypt(&data as &[u8], &mut output[..], &mut tag[..]);

    return (output, tag);
}

#[tauri::command]
fn test_bundle(file_path: &str) {
    let data = fs::read(file_path);
    let (output, tag) = bundle(data.unwrap());
    println!("{:?}", tag);
    println!("{:?}", output);
}

#[tauri::command]
async fn send_file(file_path: &str) -> Result<(), ()> {
    println!("Encrypting data...");
    let data = fs::read(file_path);
    let (output, tag) = bundle(data.unwrap());

    let url = url::Url::parse("ws://127.0.0.1:8080/updates").unwrap();

    let (ws_stream, _response) = connect_async(url).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");

    let (mut write, read) = ws_stream.split();

    println!("sending size");
    println!("{}", output.len());

    write
        .send(Message::Text(output.len().to_string()))
        .await
        .unwrap();

    println!("sending data");

    let mut counter: usize = 0;
    while counter < output.len() {
        if output.len() - counter < 16 {
            let left_over = output.len() - counter;
            write
                .send(Message::Binary(
                    output[counter..counter + left_over]
                        .iter()
                        .cloned()
                        .collect(),
                ))
                .await
                .unwrap();
        } else {
            write
                .send(Message::Binary(
                    output[counter..counter + 16].iter().cloned().collect(),
                ))
                .await
                .unwrap();
        }

        counter += 16
    }

    println!("Sending auth tag");
    write.send(Message::Binary(tag)).await.unwrap();

    println!("Finished");

    Ok(())
}

#[tokio::main]
pub async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![send_file, test_bundle])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
