mod protect

extern crate crypto;

use crypto::{ symmetriccipher, buffer, aes, blockmodes };
use crypto::buffer::{ ReadBuffer, WriteBuffer, BufferResult };

pub fun bundle(data: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {

}