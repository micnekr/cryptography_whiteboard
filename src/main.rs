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
    // let original = String::from(
    //     "Give a man a fish and you feed him for a day.
    // Teach a man to fish and you feed him for a lifetime.",
    // );

    let original = String::from(
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nulla dignissim turpis sit amet turpis mattis consectetur. In venenatis nunc tortor, quis laoreet enim hendrerit id. Donec in tortor at tortor ullamcorper hendrerit sit amet non quam. Nunc faucibus neque iaculis felis commodo, eu aliquet ex dignissim. Nullam feugiat luctus libero id mollis. Proin vitae efficitur massa. Mauris in accumsan sem. Maecenas tristique, libero nec mattis tincidunt, ligula orci dignissim felis, vel malesuada est lectus sollicitudin elit.

    Vestibulum purus nisl, ornare nec odio sit amet, porta volutpat arcu. Morbi risus massa, mattis eget metus et, cursus suscipit ante. Ut accumsan, risus sed ultrices faucibus, nulla orci cursus est, vel viverra lacus augue ac libero. Aliquam erat volutpat. Aliquam sit amet nisi commodo, tincidunt ex eu, tempor sapien. Maecenas eget nisl eget eros ullamcorper tristique. Aenean facilisis posuere mauris. Pellentesque quis feugiat massa, at elementum nulla. Mauris lobortis posuere libero sit amet gravida.

    Integer interdum justo vitae elit laoreet, nec efficitur magna commodo. Nam semper vitae tortor id consectetur. Aenean ac rutrum leo. In mauris sem, aliquet et dapibus nec, ultrices ac libero. Integer sit amet orci eget est condimentum dapibus. Praesent vel ultrices quam. Nam consequat ligula eu molestie hendrerit. Duis tincidunt, nisi efficitur hendrerit facilisis, erat lacus ultricies ligula, et tempor elit nibh nec metus. Vivamus suscipit libero arcu, non iaculis nisi maximus vitae. Proin tincidunt vestibulum justo ac ultrices.",
    );

    let original_len = original.as_bytes().len() * 8;
    println!("Original: {},\n bit length: {}", original, original_len);

    let huffman = huffman::HuffmanEncoding::from_crypto_iter(original.serialise());

    let dictionary_bit_length = &huffman.dictionary.serialise().count() * 8;
    let message_bit_length = huffman.clone().data.into_iter().count();
    println!("Stream: {:?}", &huffman.data);
    println!("Dictionary: {:?}", &huffman.dictionary);
    println!("Dictionary bit length: {}", dictionary_bit_length);
    println!("Message bit length: {:?}", message_bit_length);
    println!(
        "Total bit length: {:?}",
        dictionary_bit_length + message_bit_length
    );
    println!(
        "Compression: {:.3}%",
        (100 * (original_len as isize
            - dictionary_bit_length as isize
            - message_bit_length as isize)) as f64
            / original_len as f64
    );

    let decompressed = HuffmanEncoding::decode(huffman);
    if let Some(decompressed) = decompressed {
        println!("{:?}", decompressed.inspect_state());
        assert_eq!(
            decompressed.inspect_state(),
            original,
            "The decompressed text differs from the original"
        );
    } else {
        panic!("Invalid encoding");
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
