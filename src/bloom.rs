use crate::bitset::Bitset;
use std::{hash::Hasher, marker::PhantomData};

pub struct Bloom<H>
where
    H: Hasher + Default,
{
    bits: Bitset,
    size: usize,
    k: usize,
    _marker: PhantomData<H>,
}

impl<H> Bloom<H>
where
    H: Hasher + Default,
{
    pub fn new() -> Self {
        Self::with_params(1000, 2, 0.001)
    }

    pub fn with_params(n: usize, k: usize, error: f64) -> Self {
        if error == 0.0 {
            panic!("error cannot be zero");
        }

        let bits = m(k, n, error);
        let mut bytes = bits / 8;
        let rem = bits % 8;

        if rem > 0 {
            let n = rem % 8;
            bytes += (n + 8 - rem) / 8;
        }

        Self {
            bits: Bitset::with_capacity(bytes),
            size: bits,
            k,
            _marker: PhantomData,
        }
    }

    pub fn contains<K>(&self, key: &K) -> bool
    where
        K: std::hash::Hash,
    {
        let (h1, h2) = self.hash(key);

        for i in 0..self.k {
            let i = i as u64;
            let h = (h1 + (i * h2)) as usize;
            if !self.bits.get(h % self.size) {
                return false;
            }
        }

        true
    }

    pub fn insert<K>(&mut self, key: &K)
    where
        K: std::hash::Hash,
    {
        let (h1, h2) = self.hash(key);

        for i in 0..self.k {
            let i = i as u64;
            let h = (h1 + (i * h2)) as usize;
            self.bits.set(h % self.size);
        }
    }

    #[inline]
    fn hash<K>(&self, key: K) -> (u64, u64)
    where
        K: std::hash::Hash,
    {
        let mut hasher = H::default();
        key.hash(&mut hasher);
        let h = hasher.finish();

        let h1 = (h >> 32) as u32 as u64;
        let h2 = (h & 0xFFFFFFFF) as u32 as u64;

        (h1, h2)
    }
}

fn m(k: usize, n: usize, f: f64) -> usize {
    let n = n as f64;
    let k = k as f64;
    f64::ceil(-(k * n) / f64::ln(1.0 - f.powf(1.0 / k))) as usize
}
