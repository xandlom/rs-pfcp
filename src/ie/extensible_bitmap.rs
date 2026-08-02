use bitvec::prelude::{BitVec, Lsb0};

/// Byte-backed PFCP bitmap which retains its encoded length and unknown bits.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub(crate) struct ExtensibleBitmap {
    bits: BitVec<u8, Lsb0>,
}

impl ExtensibleBitmap {
    pub(crate) fn with_min_octets(octets: usize) -> Self {
        Self {
            bits: BitVec::repeat(false, octets * 8),
        }
    }

    pub(crate) fn from_octets(octets: impl Into<Vec<u8>>) -> Self {
        Self {
            bits: BitVec::from_vec(octets.into()),
        }
    }

    pub(crate) fn contains(&self, bit: usize) -> bool {
        self.bits.get(bit).is_some_and(|value| *value)
    }

    pub(crate) fn insert(&mut self, bit: usize) {
        if self.bits.len() <= bit {
            self.bits.resize((bit / 8 + 1) * 8, false);
        }
        self.bits.set(bit, true);
    }

    pub(crate) fn remove(&mut self, bit: usize) {
        if bit < self.bits.len() {
            self.bits.set(bit, false);
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.bits.not_any()
    }

    pub(crate) fn count_ones(&self) -> usize {
        self.bits.count_ones()
    }

    pub(crate) fn octets(&self) -> &[u8] {
        self.bits.as_raw_slice()
    }

    pub(crate) fn union(&mut self, other: &Self) {
        for bit in other.bits.iter_ones() {
            self.insert(bit);
        }
    }
}
