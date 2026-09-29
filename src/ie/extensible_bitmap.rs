/// Byte-backed PFCP bitmap which retains its encoded length and unknown bits.
///
/// Bit `n` lives in octet `n / 8` at position `n % 8` (LSB first).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub(crate) struct ExtensibleBitmap {
    octets: Vec<u8>,
}

impl ExtensibleBitmap {
    pub(crate) fn with_min_octets(octets: usize) -> Self {
        Self {
            octets: vec![0; octets],
        }
    }

    pub(crate) fn from_octets(octets: impl Into<Vec<u8>>) -> Self {
        Self {
            octets: octets.into(),
        }
    }

    pub(crate) fn contains(&self, bit: usize) -> bool {
        self.octets
            .get(bit / 8)
            .is_some_and(|octet| octet & (1 << (bit % 8)) != 0)
    }

    pub(crate) fn insert(&mut self, bit: usize) {
        if self.octets.len() <= bit / 8 {
            self.octets.resize(bit / 8 + 1, 0);
        }
        self.octets[bit / 8] |= 1 << (bit % 8);
    }

    pub(crate) fn remove(&mut self, bit: usize) {
        if let Some(octet) = self.octets.get_mut(bit / 8) {
            *octet &= !(1 << (bit % 8));
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.octets.iter().all(|octet| *octet == 0)
    }

    pub(crate) fn count_ones(&self) -> usize {
        self.octets
            .iter()
            .map(|octet| octet.count_ones() as usize)
            .sum()
    }

    pub(crate) fn octets(&self) -> &[u8] {
        &self.octets
    }

    pub(crate) fn union(&mut self, other: &Self) {
        if self.octets.len() < other.octets.len() {
            self.octets.resize(other.octets.len(), 0);
        }
        for (dst, src) in self.octets.iter_mut().zip(&other.octets) {
            *dst |= src;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_contains_remove() {
        let mut b = ExtensibleBitmap::default();
        assert!(b.is_empty());
        b.insert(0);
        b.insert(9);
        assert!(b.contains(0) && b.contains(9) && !b.contains(1));
        assert_eq!(b.octets(), &[0b1, 0b10]);
        assert_eq!(b.count_ones(), 2);
        b.remove(9);
        b.remove(100);
        assert!(!b.contains(9));
        assert_eq!(b.octets().len(), 2, "encoded length is retained");
    }

    #[test]
    fn test_union_and_min_octets() {
        let mut a = ExtensibleBitmap::with_min_octets(3);
        let b = ExtensibleBitmap::from_octets(vec![0x80, 0, 0, 0x01]);
        a.insert(1);
        a.union(&b);
        assert_eq!(a.octets(), &[0x82, 0, 0, 0x01]);
        assert!(a.contains(7) && a.contains(24));
    }
}
