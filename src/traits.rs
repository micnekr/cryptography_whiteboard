use std::vec::IntoIter;

use crate::cyphers::simple::{CaesarCypherTransform, XorTransform};

pub trait Serialisable {
    fn to_byte_iter(&self) -> IntoIter<u8>;
    fn from_byte_iter<I: Iterator<Item = u8> + ToOwned<Owned = I>>(b: &I) -> Self;
}

pub struct U64Iterator<I: CryptographicIter> {
    iter: I,
    is_over: bool,
}

impl<I: CryptographicIter> U64Iterator<I> {
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            is_over: false,
        }
    }

    // // TODO: somehow restructure how the data is stored
    // pub fn into_cryptographic_iter(self) -> IntoIter<u8> {
    //     let v: Vec<_> = self.collect();
    //     v.into_iter().map(|| )
    // }
}

// TODO: also override the length
impl<I: CryptographicIter> Iterator for U64Iterator<I> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_over {
            return None;
        }

        // TODO: improve performance of this, including unrolling the loop
        let mut val: u64 = 0;
        for i in 0..8 {
            let current = self.iter.next();

            if let Some(current) = current {
                val <<= 8; // NOTE: this shifts the 0 - may be not as performant
                val += current as u64;
            } else {
                // shift it so that the most significant byte is the first byte
                val <<= 64 - 8 * i;

                self.is_over = true;

                return Some(val);
            }
        }
        // last u64 is to show how many chunks of the previous u64 were actual data
        Some(val)
    }
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

    fn into_u64_iter(self) -> U64Iterator<Self>
    where
        Self: Sized,
    {
        U64Iterator::new(self)
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
