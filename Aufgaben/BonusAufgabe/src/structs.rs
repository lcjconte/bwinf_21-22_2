#![allow(non_snake_case)]
use std::ops::{BitOr, BitXor};
use std::collections::HashMap;
use ahash::RandomState;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct DParray<T: Copy>{
    underlying: Vec<T>,
    bounds: (usize, usize, usize),
}
impl<T: Copy> DParray<T> {
    pub fn new(initial_value: T, x_max: usize, y_max: usize, z_max: usize) -> DParray<T> {
        let vec = vec![initial_value; (x_max+1)*(y_max+1)*(z_max+1)];
        DParray { underlying: vec, bounds: (x_max+1, y_max+1, z_max+1) }
    }
    pub fn get2_mut(&mut self, x: usize, y: usize) -> &mut T {
        assert!(x < self.bounds.0 && y < self.bounds.1);
        &mut self.underlying[y*self.bounds.0+x]
    }
    pub fn get2(&self, x: usize, y: usize) -> T {
        assert!(x < self.bounds.0 && y < self.bounds.1);
        self.underlying[y*self.bounds.0+x]
    }
    pub fn get3_mut(&mut self, x: usize, y: usize, z: usize) -> &mut T{
        assert!(x < self.bounds.0 && y < self.bounds.1 && z < self.bounds.2);
        &mut self.underlying[z*self.bounds.1*self.bounds.0+y*self.bounds.0+x]
    }
    pub fn get3(&self, x: usize, y: usize, z: usize) -> T{
        assert!(x < self.bounds.0 && y < self.bounds.1 && z < self.bounds.2);
        self.underlying[z*self.bounds.1*self.bounds.0+y*self.bounds.0+x]
    }
}

/// A 256 unsigned int with limited functionality
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct u256(u128, u128);
impl u256 {
    /// Sets bit at idx to 1
    #[inline]
    pub fn set(&mut self, mut idx: usize) {
        debug_assert!(idx < 256);
        let ed: &mut u128 = if idx >= 128 {
            idx -= 128;
            &mut self.1
        }else {
            &mut self.0
        };
        *ed |= 1 << idx;
    }
    /// Toggles bit
    #[inline]
    pub fn toggle(&mut self, mut idx: usize) {
        debug_assert!(idx < 256);
        let ed: &mut u128 = if idx >= 128 {
            idx -= 128;
            &mut self.1
        }else {
            &mut self.0
        };
        *ed ^= 1 << idx;
    }
    /// Returns value of bit at idx
    #[inline]
    pub fn get(&self, mut idx: usize) -> bool {
        debug_assert!(idx < 256);
        let ed: &u128 = if idx >= 128 {
            idx -= 128;
            &self.1
        }else {
            &self.0
        };
        (*ed & (1 << idx)) > 0
    }
    pub fn zero() -> Self {
        Self::default()
    }
}
impl From<u128> for u256 {
    fn from(v: u128) -> Self {
        u256(v, 0)
    }
}
impl BitOr for u256 {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0, self.1 | rhs.1)
    }
}
impl BitXor for u256 {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0, self.1 ^ rhs.1)
    }
}

/// Trait for fast storage and retrieval of combinations
pub trait CombStore : Clone {
    fn new(size: usize) -> Self;
    fn insert(&mut self, k: u128, v: u256);
    fn get(&mut self, k: u128) -> Option<u256>;
    fn clear(&mut self);
}

#[derive(Clone)]
pub struct HashMapStore(HashMap<u128, u256, RandomState>);
impl CombStore for HashMapStore {
    fn new(size: usize) -> Self{
        let mut hmap = HashMap::<u128, u256, RandomState>::default();
        hmap.reserve(size);
        HashMapStore(hmap)
    }
    #[inline]
    fn insert(&mut self, k: u128, v: u256) {
        self.0.insert(k, v);
    }
    #[inline]
    fn get(&mut self, k: u128) -> Option<u256> {
        self.0.get(&k).cloned()
    }
    #[inline]
    fn clear(&mut self) {
        self.0.clear();
    }
}
/// (xor, combination)
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Combination(pub u128, pub u256);
impl Combination {
    /// Add number and its index to new combination
    #[inline]
    pub fn add(&self, b: u128, idx: usize) -> Combination {
        let mut c = Combination(self.0 ^ b, self.1);
        c.1.set(idx);
        c
    }
    #[inline]
    pub fn apply(&self, b: u128, idx: usize) -> Combination {
        let mut c = Combination(self.0 ^ b, self.1);
        c.1.toggle(idx);
        c
    }
    #[inline]
    pub fn combine(&self, b: &Combination) -> Combination {
        let mut c = Combination(self.0 ^ b.0, self.1);
        c.1 = c.1 | b.1;
        c
    }
    #[inline]
    pub fn toggle_inplace(&mut self, b: &Combination) {
        self.0 ^= b.0;
        self.1 = self.1 ^ b.1;
    }
}
pub type SearchRes = Option<Combination>;