use std::vec::IntoIter;

use crate::cyphers::simple::{CaesarCypherTransform, XorTransform};

pub trait Serialisable {
    fn to_crypto_iter(&self) -> IntoIter<u8>;
    fn from_byte_iter<I: Iterator<Item = u8> + ToOwned<Owned = I>>(b: &I) -> Self;
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
        String::from_byte_iter(self)
    }
}
