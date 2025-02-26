use std::ops::BitXorAssign;

#[derive(Debug, Clone)]
pub struct Symbol {
    esi: u16,
    data: Vec<u8>,
    repair: bool
}

impl Symbol {
    pub fn new(esi: u16, data: Vec<u8>, repair: bool) -> Self {
        Symbol { esi, data, repair }
    }
}

impl BitXorAssign for Symbol {
    fn bitxor_assign(&mut self, other: Self) {
        assert_eq!(self.data.len(), other.data.len(), "Symbol XOR: data length mismatch");
        self.data.iter_mut().zip(other.data.iter()).for_each(|(a, b)| *a ^= b);
    }
}