use std::{fmt::Debug, iter::once};

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
        assert!(self.len <= 8, "The length can not be a byte or longer");

        self.val |= (bit as u8) << (8 - self.len);
    }
    pub fn clear(&mut self) {
        self.val = 0;
        self.len = 0;
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

#[derive(Clone)]
pub struct BitStream {
    bytes: Vec<u8>,
    last_sub_byte: SubByteValue,
}

impl BitStream {
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

    pub fn push(&mut self, v: SubByteValue) {
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
            self.last_sub_byte.val = v.val << (8 - last_byte_len);
        }
    }

    // TODO: make sure this does not overflow
    pub fn len(&self) -> usize {
        self.bytes.len() * 8 + self.last_sub_byte.len as usize
    }
}

pub struct IntoIter {
    stream: BitStream,
    index: usize, // TODO: make sure it does not overflow
}

impl IntoIterator for BitStream {
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

impl Debug for BitStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.bytes.iter().map(|b| format!("{:08b}", b));

        let s: String = if self.last_sub_byte.len == 0 {
            s.collect()
        } else {
            s.chain(once(self.last_sub_byte.clone().into())).collect()
        };

        f.debug_struct("BitStream").field("data", &s).finish()
    }
}
