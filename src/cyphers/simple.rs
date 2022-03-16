use crate::traits::CryptographicIter;

#[derive(Clone)]
pub struct CaesarCypherTransform<I: CryptographicIter> {
    iter: I,
    shift: u8,
}

impl<I: CryptographicIter> CaesarCypherTransform<I> {
    #[inline]
    pub fn new(iter: I, shift: u8) -> Self {
        CaesarCypherTransform { iter, shift }
    }
}

impl<I: CryptographicIter> Iterator for CaesarCypherTransform<I> {
    type Item = u8;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|v| v.wrapping_add(self.shift))
    }
}
impl<I: CryptographicIter> CryptographicIter for CaesarCypherTransform<I> {}

#[derive(Clone)]
pub struct XorTransform<I1: CryptographicIter, I2: CryptographicIter> {
    iter: I1,
    key: I2,
}

impl<I1: CryptographicIter, I2: CryptographicIter> XorTransform<I1, I2> {
    #[inline]
    pub fn new(iter: I1, key: I2) -> Self {
        XorTransform { iter, key }
    }
}

impl<I1: CryptographicIter, I2: CryptographicIter> Iterator for XorTransform<I1, I2> {
    type Item = u8;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let (Some(ch), Some(k)) = (self.iter.next(), self.key.next()) {
            Some(ch ^ k)
        } else {
            None
        }
    }
}
impl<I1: CryptographicIter, I2: CryptographicIter> CryptographicIter for XorTransform<I1, I2> {}
