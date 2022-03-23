use cryptography_whiteboard::{
    compression::huffman::HuffmanEncoding,
    traits::{InspectableState, Serialisable},
};

#[cfg(test)]
mod tests {
    use crate::test_huffman_raw;

    pub const TEST_MESSAGES: [&str; 4] = [
        "AABAC",
        "",
        "Give a man a fish and you feed him for a day.
        Teach a man to fish and you feed him for a lifetime.",
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nulla dignissim turpis sit amet turpis mattis consectetur. In venenatis nunc tortor, quis laoreet enim hendrerit id. Donec in tortor at tortor ullamcorper hendrerit sit amet non quam. Nunc faucibus neque iaculis felis commodo, eu aliquet ex dignissim. Nullam feugiat luctus libero id mollis. Proin vitae efficitur massa. Mauris in accumsan sem. Maecenas tristique, libero nec mattis tincidunt, ligula orci dignissim felis, vel malesuada est lectus sollicitudin elit.

        Vestibulum purus nisl, ornare nec odio sit amet, porta volutpat arcu. Morbi risus massa, mattis eget metus et, cursus suscipit ante. Ut accumsan, risus sed ultrices faucibus, nulla orci cursus est, vel viverra lacus augue ac libero. Aliquam erat volutpat. Aliquam sit amet nisi commodo, tincidunt ex eu, tempor sapien. Maecenas eget nisl eget eros ullamcorper tristique. Aenean facilisis posuere mauris. Pellentesque quis feugiat massa, at elementum nulla. Mauris lobortis posuere libero sit amet gravida.

        Integer interdum justo vitae elit laoreet, nec efficitur magna commodo. Nam semper vitae tortor id consectetur. Aenean ac rutrum leo. In mauris sem, aliquet et dapibus nec, ultrices ac libero. Integer sit amet orci eget est condimentum dapibus. Praesent vel ultrices quam. Nam consequat ligula eu molestie hendrerit. Duis tincidunt, nisi efficitur hendrerit facilisis, erat lacus ultricies ligula, et tempor elit nibh nec metus. Vivamus suscipit libero arcu, non iaculis nisi maximus vitae. Proin tincidunt vestibulum justo ac ultrices.",
    ];

    #[test]
    fn test_huffman() {
        for message in TEST_MESSAGES {
            test_huffman_raw(message.to_owned());
        }
    }
}

fn test_huffman_raw(original: String) {
    let original_len = original.as_bytes().len() * 8;
    // println!("Original: {},\n bit length: {}", original, original_len);

    let huffman = HuffmanEncoding::from_crypto_iter(original.serialise());

    let dictionary_bit_length = &huffman.dictionary.serialise().count() * 8;
    let message_bit_length = huffman.clone().data.into_iter().count();
    // println!("Stream: {:?}", &huffman.data);
    // println!("Dictionary: {:?}", &huffman.dictionary);
    // println!("Dictionary bit length: {}", dictionary_bit_length);
    // println!("Message bit length: {:?}", message_bit_length);
    // println!(
    //     "Total bit length: {:?}",
    //     dictionary_bit_length + message_bit_length
    // );
    // println!(
    //     "Compression: {:.3}%",
    //     (100 * (original_len as isize
    //         - dictionary_bit_length as isize
    //         - message_bit_length as isize)) as f64
    //         / original_len as f64
    // );

    let decompressed = HuffmanEncoding::decode(huffman);
    if let Some(decompressed) = decompressed {
        // println!("{:?}", decompressed.inspect_state());
        assert_eq!(
            decompressed.inspect_state(),
            original,
            "The decompressed text differs from the original"
        );
    } else {
        panic!("Invalid encoding");
    }
}
