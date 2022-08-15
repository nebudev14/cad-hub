extern crate crypto;

use crypto::aead::AeadEncryptor;
use crypto::{aes, aes_gcm};
use dotenv;
use std::iter::repeat;
use std::iter::Iterator;

pub fn bundle(data: Vec<u8>) -> (Vec<u8>, Vec<u8>) {
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
