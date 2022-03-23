use std::{borrow::Borrow, ops::Deref};

use cryptography_whiteboard::traits::{CryptographicIter, InspectableState, Serialisable};

#[cfg(test)]
mod tests {
    use crate::{test_caesar_raw, test_vernam_raw};

    pub const TEST_KEYS: [&str; 3] = [
        "a bc", "", "a
b
c",
    ];

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
    fn test_caesar() {
        for message in TEST_MESSAGES {
            test_caesar_raw(message.to_owned());
        }
    }

    #[test]
    fn test_vernam() {
        for message in TEST_MESSAGES {
            for key in TEST_KEYS {
                test_vernam_raw(message.to_owned(), key.to_owned());
            }
        }
    }
}

fn test_caesar_raw(plaintext: String) {
    let ciphertext = plaintext.serialise().caesar_shift(3);

    assert_eq!(
        plaintext.serialise().inspect_state(),
        ciphertext.caesar_unshift(3).inspect_state(),
        "The decrypted text does not match the original"
    );
}

fn test_vernam_raw(plaintext: String, key: String) {
    let plaintext = plaintext.serialise();
    let key = key.serialise();

    let expected = plaintext.inspect_state();
    // NOTE: it is truncated to the length of the shortest of the streams
    let expected = &expected.as_bytes()[..std::cmp::min(key.len(), expected.len())];
    let expected = String::from_utf8_lossy(expected);
    let expected = expected.deref();

    assert_eq!(
        expected,
        plaintext.xor(key.clone()).xor(key).inspect_state(),
        "The decrypted text does not match the original"
    );
}
