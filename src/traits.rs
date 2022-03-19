use std::{collections::BTreeMap, vec::IntoIter};

use crate::cyphers::simple::{CaesarCypherTransform, XorTransform};

pub trait Serialisable {
    type CryptoIter;

    fn serialise(&self) -> Self::CryptoIter;
    fn deserialise<I: Iterator<Item = u8>>(b: I) -> Option<Self>
    where
        Self: Sized;
}

pub trait CryptographicIter: Iterator<Item = u8> {
    #[inline]
    fn caesar_shift(self, shift: u8) -> CaesarCypherTransform<Self>
    where
        Self: Sized,
    {
        CaesarCypherTransform::new(self, shift)
    }

    #[inline]
    fn caesar_unshift(self, shift: u8) -> CaesarCypherTransform<Self>
    where
        Self: Sized,
    {
        CaesarCypherTransform::new(self, 0u8.wrapping_sub(shift))
    }

    #[inline]
    fn xor<I2: CryptographicIter>(self, key: I2) -> XorTransform<Self, I2>
    where
        Self: Sized,
    {
        XorTransform::new(self, key)
    }
}

impl CryptographicIter for IntoIter<u8> {}

pub trait InspectableState {
    fn inspect_state(&self) -> String;
}

impl<I: Iterator<Item = u8> + ToOwned<Owned = I>> InspectableState for I {
    #[inline]
    fn inspect_state(&self) -> String {
        String::deserialise(self.to_owned()).unwrap_or(String::from("<Ccount not parse>"))
    }
}
