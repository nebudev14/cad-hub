#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

extern crate crypto;

use tokio::io::{AsyncWriteExt, Result};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{StreamExt, SinkExt};

use crypto::{ symmetriccipher, buffer, aes, aes_gcm, blockmodes };
use crypto::buffer::{ ReadBuffer, WriteBuffer, BufferResult };

use std::env;
use dotenv;



// #[tauri::command]
// fn bundle(data: &[u8], size: u32) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
//   dotenv::dotenv.ok();


//   let mut cipher = aes_gcm::AesGcm::new();
// }

#[tauri::command]
async fn vars() {
  dotenv::dotenv().ok();
  let value = dotenv::var("AES_KEY").unwrap();

}

#[tauri::command]
async fn greet() {
  println!("Hello, tokio-tungstenite!");

  let url = url::Url::parse("ws://127.0.0.1:8080/updates").unwrap();

  let (ws_stream, _response) = connect_async(url).await.expect("Failed to connect");
  println!("WebSocket handshake has been successfully completed");

  let (mut write, read) = ws_stream.split();

  println!("sending");

  write.send(Message::Text(r#"hello"#.to_string())).await.unwrap();

  println!("sent");

  let read_future = read.for_each(|message| async {
      println!("receiving...");
       let data = message.unwrap().into_data();
       tokio::io::stdout().write(&data).await.unwrap();
       println!("received...");
       let s = String::from_utf8(data);
       if s.unwrap() == "Whats up" {
        println!("\nequal!");
       }
  });

  read_future.await;

}

#[tokio::main]
pub async fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet, vars])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
