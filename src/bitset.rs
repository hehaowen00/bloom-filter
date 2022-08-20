pub struct Bitset {
    data: Vec<u8>,
}

impl Bitset {
    pub fn with_capacity(bytes: usize) -> Self {
        Self {
            data: vec![0; bytes],
        }
    }

    pub fn get(&self, idx: usize) -> bool {
        self.data[idx / 8] & (1 << idx % 8) > 0
    }

    pub fn set(&mut self, idx: usize) {
        self.data[idx / 8] |= 1 << (idx % 8);
    }
}
