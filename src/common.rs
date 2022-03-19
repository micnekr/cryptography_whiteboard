use std::vec::IntoIter;

use crate::traits::Serialisable;

impl Serialisable for String {
    type CryptoIter = IntoIter<u8>;
    fn serialise(&self) -> Self::CryptoIter {
        self.as_bytes().to_vec().into_iter()
    }
    fn deserialise<I: Iterator<Item = u8>>(b: I) -> Option<Self> {
        let b: Vec<_> = b.collect();
        Some(String::from_utf8(b).expect("Expected a valid UTF-8 string"))
    }
}
