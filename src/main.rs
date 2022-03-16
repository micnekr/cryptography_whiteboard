use traits::Serialisable;

use crate::traits::{CryptographicIter, InspectableState};

mod common;
mod cyphers;
mod traits;

fn main() {
    let plaintext = String::from("test");
    let xor_key = String::from("Second test value");
    // let xor_key = String::from("abc");

    let ciphertext = plaintext.to_byte_iter().caesar_shift(3);
    // let xor_key = xor_key.to_byte_iter();
    let short_xor_key = xor_key.to_byte_iter();

    String::from_byte_iter(&ciphertext);
    println!("Caesar cypher encrypted: {}", ciphertext.inspect_state());

    let plaintext = ciphertext.caesar_unshift(3);
    println!("Caesar cypher decrypted: {}", plaintext.inspect_state());

    let plaintext = plaintext.xor(short_xor_key.clone());
    println!(
        "Vernam cypher encrypted: {}, len: {}",
        plaintext.inspect_state(),
        plaintext.inspect_state().len()
    );
    let plaintext = plaintext.xor(short_xor_key);
    println!("Vernam cypher decrypted: {}", plaintext.inspect_state());
}
