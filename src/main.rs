use crate::traits::InspectableState;
use traits::Serialisable;

use crate::compression::huffman::{self, HuffmanEncoding};

pub mod bitstream;
pub mod common;
pub mod compression;
pub mod cyphers;
pub mod traits;

fn main() {
    // let original = String::from("AABAC");
    // let original = String::new();
    let original = String::from(
        r#"Give a man a fish and you feed him for a day.
    Teach a man to fish and you feed him for a lifetime."#,
    );

    println!(
        "Original: {}, bit length: {}",
        original,
        original.as_bytes().len() * 8
    );

    let huffman = huffman::HuffmanEncoding::from_crypto_iter(original.to_crypto_iter());

    println!("Dictionary: {:?}", &huffman.dictionary);
    println!("Stream: {:?}", &huffman.data);
    println!("Bit length: {:?}", huffman.clone().data.into_iter().count());

    let decompressed = HuffmanEncoding::decode(huffman);
    if let Some(decompressed) = decompressed {
        println!("{:?}", decompressed.inspect_state());
    } else {
        println!("Invalid encoding");
    }

    // let plaintext = String::from("test");
    // let xor_key = String::from("Second test value");
    // // let xor_key = String::from("abc");

    // let ciphertext = plaintext.to_crypto_iter().caesar_shift(3);
    // // let xor_key = xor_key.to_byte_iter();
    // let short_xor_key = xor_key.to_crypto_iter();

    // String::from_byte_iter(&ciphertext);
    // println!("Caesar cypher encrypted: {}", ciphertext.inspect_state());

    // let plaintext = ciphertext.caesar_unshift(3);
    // println!("Caesar cypher decrypted: {}", plaintext.inspect_state());

    // let plaintext = plaintext.xor(short_xor_key.clone());
    // println!(
    //     "Vernam cypher encrypted: {}, len: {}",
    //     plaintext.inspect_state(),
    //     plaintext.inspect_state().len()
    // );
    // let plaintext = plaintext.xor(short_xor_key);
    // println!("Vernam cypher decrypted: {}", plaintext.inspect_state());
}
