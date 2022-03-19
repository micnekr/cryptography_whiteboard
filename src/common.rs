use std::vec::IntoIter;

use crate::traits::Serialisable;

impl Serialisable for String {
    fn to_crypto_iter(&self) -> IntoIter<u8> {
        self.as_bytes().to_vec().into_iter()
    }
    fn from_byte_iter<I: Iterator<Item = u8> + ToOwned<Owned = I>>(b: &I) -> Self {
        let b: Vec<_> = b.to_owned().collect();
        String::from_utf8(b).expect("Expected a valid UTF-8 string")
    }
}
