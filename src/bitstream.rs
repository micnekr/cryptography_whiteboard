use std::{
    fmt::Debug,
    iter::once,
    ops::{Add, AddAssign},
};

use crate::traits::Serialisable;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SubByteValue {
    val: u8,
    len: u8,
}

impl SubByteValue {
    pub fn new() -> Self {
        Self { len: 0, val: 0 }
    }
    pub fn add_bit(&mut self, bit: bool) {
        self.len += 1;
        assert!(
            self.len <= 8,
            "The length of a SubByteValue can not be longer than a byte"
        );

        self.val |= (bit as u8) << (8 - self.len);
    }

    #[inline]
    pub fn clear(&mut self) {
        self.val = 0;
        self.len = 0;
    }

    #[inline]
    pub fn raw_value(&self) -> u8 {
        self.val
    }
    #[inline]
    pub fn len(&self) -> u8 {
        self.len
    }
}

impl TryFrom<&str> for SubByteValue {
    type Error = std::io::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut ret = SubByteValue::new();
        for c in value.chars() {
            match c {
                '0' => {
                    ret.add_bit(false);
                }
                '1' => {
                    ret.add_bit(true);
                }
                _ => {
                    return Err(Self::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "The string has to contain only '0' and '1' characters",
                    ));
                }
            };
        }
        Ok(ret)
    }
}

impl From<SubByteValue> for String {
    fn from(v: SubByteValue) -> Self {
        // get binary repr
        let s = format!("{:08b}", v.val).to_owned();

        let s = s.chars().into_iter();

        // remove all the unneeded bits
        s.take(v.len as usize).collect()
    }
}

impl Debug for SubByteValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SubByteValue")
            .field("v", &<String as From<SubByteValue>>::from(self.to_owned()))
            .field("len", &self.len)
            .finish()
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BitVec {
    bytes: Vec<u8>,
    last_sub_byte: SubByteValue,
}

impl BitVec {
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            bytes: Vec::with_capacity(cap),
            last_sub_byte: SubByteValue::new(),
        }
    }
    pub fn new() -> Self {
        Self {
            bytes: vec![],
            last_sub_byte: SubByteValue::new(),
        }
    }

    // TODO: make sure this does not overflow
    pub fn len(&self) -> usize {
        self.bytes.len() * 8 + self.last_sub_byte.len as usize
    }
    pub fn clear(&mut self) {
        self.bytes.clear();
        self.last_sub_byte = SubByteValue::new();
    }

    #[inline]
    pub fn bytes(&self) -> &Vec<u8> {
        &self.bytes
    }

    #[inline]
    pub fn last_sub_byte(&self) -> &SubByteValue {
        &self.last_sub_byte
    }
}

impl AddAssign<bool> for BitVec {
    fn add_assign(&mut self, v: bool) {
        self.last_sub_byte.add_bit(v);

        if self.last_sub_byte.len == 8 {
            self.bytes.push(self.last_sub_byte.val);
            self.last_sub_byte = SubByteValue::new();
        }
    }
}

impl Add<bool> for BitVec {
    type Output = BitVec;

    fn add(mut self, v: bool) -> Self::Output {
        self += v;
        self
    }
}

impl AddAssign<SubByteValue> for BitVec {
    fn add_assign(&mut self, v: SubByteValue) {
        // append to the buffer and record the buffer if needed
        let last_byte_len = self.last_sub_byte.len;
        let v_len = v.len;

        let len_sum = last_byte_len + v_len;

        // fill in the byte with the new values
        self.last_sub_byte.val |= v.val >> last_byte_len;

        if len_sum < 8 {
            self.last_sub_byte.len = len_sum;
        } else {
            self.bytes.push(self.last_sub_byte.val);

            self.last_sub_byte.len = len_sum - 8;
            // TODO: find a way to do this without the checks
            // NOTE: it would panic if the shift exceeds the bit width of the type
            if last_byte_len == 0 {
                self.last_sub_byte.clear();
            } else {
                self.last_sub_byte.val = v.val << (8 - last_byte_len);
            }
        }
    }
}

impl Add<SubByteValue> for BitVec {
    type Output = BitVec;

    fn add(mut self, v: SubByteValue) -> Self::Output {
        self += v;
        self
    }
}

impl AddAssign<BitVec> for BitVec {
    fn add_assign(&mut self, v: BitVec) {
        // TODO: do those extra allocations of the len byte matter? Should we optimise them?
        v.bytes.into_iter().for_each(|b| {
            *self += SubByteValue { len: 8, val: b };
        });
        *self += v.last_sub_byte;
    }
}

impl Add<BitVec> for BitVec {
    type Output = BitVec;

    fn add(mut self, v: BitVec) -> Self::Output {
        self += v;
        self
    }
}

pub struct IntoIter {
    stream: BitVec,
    index: usize, // TODO: make sure it does not overflow
}

impl IntoIterator for BitVec {
    type Item = bool;
    type IntoIter = IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            stream: self,
            index: 0,
        }
    }
}

impl Iterator for IntoIter {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let vec_index = self.index / 8;
        let bit_index = (self.index % 8) as u8;

        let vec_len = self.stream.bytes.len();
        let byte = if vec_index > vec_len {
            return None;
        } else if vec_len == vec_index {
            if bit_index >= self.stream.last_sub_byte.len {
                return None;
            } else {
                self.stream.last_sub_byte.val
            }
        } else {
            // SAFETY: we have checked the length already - it is safe to get the value
            unsafe { *self.stream.bytes.get_unchecked(vec_index) }
        };

        let ret = byte & (1 << 7 - bit_index) != 0;

        self.index += 1;

        Some(ret)
    }
}

impl Debug for BitVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.bytes.iter().map(|b| format!("{:08b}", b));

        let s: String = if self.last_sub_byte.len == 0 {
            s.collect()
        } else {
            s.chain(once(self.last_sub_byte.clone().into())).collect()
        };

        f.debug_struct("BitVec").field("data", &s).finish()
    }
}

impl Serialisable for BitVec {
    type CryptoIter = std::iter::Chain<
        std::iter::Chain<std::iter::Once<u8>, std::iter::Once<u8>>,
        std::vec::IntoIter<u8>,
    >;

    fn serialise(&self) -> Self::CryptoIter {
        once(self.last_sub_byte.len)
            .chain(once(self.last_sub_byte.val))
            .chain(self.bytes.clone().into_iter())
    }

    fn deserialise<I: Iterator<Item = u8>>(mut b: I) -> Option<Self>
    where
        Self: Sized,
    {
        // the first two bytes are the length of the "leftover" bits and their values.
        if let (Some(len), Some(val)) = (b.next(), b.next()) {
            let sub_byte_value = SubByteValue { val, len };
            Some(BitVec {
                last_sub_byte: sub_byte_value,
                bytes: b.collect(),
            })
        } else {
            None
        }
    }
}
