use std::{
    collections::{BTreeMap, BinaryHeap},
    iter::Chain,
    vec::IntoIter,
};

use crate::{
    bitstream::BitVec,
    traits::{CryptographicIter, Serialisable},
};

// TODO: differentiate between methods that collect the stream and the ones that do not
#[derive(Clone)]
pub struct HuffmanEncoding {
    pub dictionary: BTreeMap<u8, BitVec>,
    pub data: BitVec,
}

impl Serialisable for BTreeMap<u8, BitVec> {
    type CryptoIter = Chain<IntoIter<u8>, <BitVec as Serialisable>::CryptoIter>;

    fn serialise(&self) -> Self::CryptoIter {
        let mut ret: Vec<u8> = Vec::with_capacity(2 * self.len() + 1);

        // We need a u8 to store a length of at most u8's worth of unique keys
        ret.push(
            self.len()
                .try_into()
                .expect("Could not store the length in a u8"),
        );

        // Store the key and the length of the value in the bitstream
        self.iter().for_each(|(k, v)| {
            ret.push(*k);
            ret.push(
                v.len()
                    .try_into()
                    .expect("Could not store the bit length of the token in a u8"),
            );
        });

        // concatenate all the BitVecs
        let mut bit_vec = BitVec::new();
        self.values().for_each(|v| bit_vec += v.to_owned());

        ret.into_iter().chain(bit_vec.serialise())
    }

    fn deserialise<I: Iterator<Item = u8>>(b: I) -> Option<Self>
    where
        Self: Sized,
    {
        todo!()
    }
}

// TODO: maybe use an array representation instead
enum ByteFrequencyTreeNode {
    LEAF {
        byte: u8,
    },
    NODE {
        left: Box<ByteFrequencyEntry>,
        right: Box<ByteFrequencyEntry>,
    },
}
struct ByteFrequencyEntry {
    f: usize,
    data: ByteFrequencyTreeNode,
}

impl ByteFrequencyEntry {
    fn fill_in_map(&self, m: &mut BTreeMap<u8, BitVec>, current_val: BitVec) {
        match &self.data {
            ByteFrequencyTreeNode::NODE { left, right } => {
                let mut l = current_val.clone();
                let mut r = current_val.clone();
                l += false;
                r += true;
                left.fill_in_map(m, l);
                right.fill_in_map(m, r);
            }
            ByteFrequencyTreeNode::LEAF { byte } => {
                m.insert(*byte, current_val);
            }
        }
    }
}

impl PartialEq for ByteFrequencyEntry {
    fn eq(&self, other: &Self) -> bool {
        self.f == other.f
    }
}
impl Eq for ByteFrequencyEntry {}

impl PartialOrd for ByteFrequencyEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // NOTE: `other` and `self` are intentionally the other way round than expected, to make sure it is a min heap
        other.f.partial_cmp(&self.f)
    }
}
impl Ord for ByteFrequencyEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // NOTE: `other` and `self` are intentionally the other way round than expected, to make sure it is a min heap
        other.f.cmp(&self.f)
    }
}

// TODO: idea for an improved compression: find all repeats of length 1, 2, 3, etc, searching for repeats only starting on the previously found repeats (e.g. "abcdab" finds "ab" twice and then looks to see if the characters following the two "ab"s are the same). Then encode imilarly to Huffman
impl HuffmanEncoding {
    pub fn new(dictionary: BTreeMap<u8, BitVec>, data: BitVec) -> Self {
        Self { dictionary, data }
    }

    pub fn from_crypto_iter<I: CryptographicIter>(iter: I) -> Self {
        let data: Vec<_> = iter.collect();
        if data.len() == 0 {
            return Self {
                dictionary: BTreeMap::new(),
                data: BitVec::new(),
            };
        }

        let mut frequencies: BTreeMap<u8, usize> = BTreeMap::new();

        // record the frequencies
        data.iter().for_each(|b| {
            frequencies.entry(*b).and_modify(|f| *f += 1).or_insert(1);
        });

        let mut frequency_heap = BinaryHeap::new();
        frequencies.iter().for_each(|(b, f)| {
            frequency_heap.push(ByteFrequencyEntry {
                f: *f,
                data: ByteFrequencyTreeNode::LEAF { byte: *b },
            })
        });

        // create the dictionary
        while frequency_heap.len() > 1 {
            // TODO: is there a more efficient way around this without reshuffling the heap twice?

            // NOTE: the unwrap is safe because we know that the length is greater than two
            let a = frequency_heap.pop().unwrap();
            let b = frequency_heap.pop().unwrap();

            frequency_heap.push(ByteFrequencyEntry {
                f: a.f + b.f,
                data: ByteFrequencyTreeNode::NODE {
                    left: Box::new(a),
                    right: Box::new(b),
                },
            });
        }

        let frequency_tree = frequency_heap.pop().unwrap(); // SAFETY: we know that there is at least one type of characters as the byte stream is not empty
        drop(frequency_heap);

        let mut code_to_node_mapping = BTreeMap::new();
        frequency_tree.fill_in_map(&mut code_to_node_mapping, BitVec::new());

        // TODO: maybe a more efficient way of manipulating bits, e.g. by having two bytes forming a circular queue and using shifts?

        // Initialise the vector with roughly the expected capacity to reduce reallocations
        let mut stream = BitVec::with_capacity((data.len() as f64 * 0.9) as usize);

        // the first byte represents how many bits at the end are padding
        data.into_iter().for_each(|c| {
            let replacement = code_to_node_mapping.get(&c).unwrap(); // SAFETY: we know that all the characters are in the mapping, so we can unwrap here
            stream += replacement.clone();
        });

        Self {
            dictionary: code_to_node_mapping,
            data: stream,
        }
    }

    pub fn decode(self) -> Option<std::vec::IntoIter<u8>> {
        // initialise it slightly over capacity to reduce reallocations
        let mut v = Vec::with_capacity((self.data.len() as f64 * 1.4) as usize / 8);

        let HuffmanEncoding { dictionary, data } = self;
        let mut reverse_dictionary = BTreeMap::new();
        dictionary.into_iter().for_each(|(k, v)| {
            reverse_dictionary.insert(v, k);
        });

        let mut buffer = BitVec::new();

        // TODO: is there a better way to read out of this?
        for bit in data.into_iter() {
            buffer += bit;
            // TODO: use a tree for improved effeciency, so that we do not need to search each thing multiple times
            if let Some(byte) = reverse_dictionary.get(&buffer) {
                buffer.clear();
                v.push(*byte);
            }
        }

        if buffer.len() != 0 {
            return None;
        }

        Some(v.into_iter())
    }
}
