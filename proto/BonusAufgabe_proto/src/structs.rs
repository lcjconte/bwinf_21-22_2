#![allow(non_snake_case)]
use std::ops::BitOr;
use std::collections::HashMap;
use ahash::RandomState;

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
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
#[allow(non_camel_case_types)]
pub struct u256(pub u128, pub u128);
impl u256 {
    /// Sets bit at idx to 1
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
    /// Returns value of bit at idx
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
impl BitOr for u256 {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0, self.1 | rhs.1)
    }
}

/// Returns xor of all numbers whose index is set in b
pub fn get_xor(nums: &[u128], b: &u256) -> u128 {
    let mut d = 0;
    for (i, num) in nums.iter().enumerate() {
        if b.get(i) {
            d ^= num;
        }
    }
    d
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
    fn insert(&mut self, k: u128, v: u256) {
        self.0.insert(k, v);
    }
    fn get(&mut self, k: u128) -> Option<u256> {
        self.0.get(&k).cloned()
    }
    fn clear(&mut self) {
        self.0.clear();
    }
}
/// (xor, combination)
#[derive(Clone, Default)]
pub struct Combination(pub u128, pub u256);
impl Combination {
    /// Add number and its index to new combination
    pub fn add(&self, b: u128, idx: usize) -> Combination {
        let mut c = Combination(self.0 ^ b, self.1.clone());
        c.1.set(idx);
        c
    }
    pub fn combine(&self, b: &Combination) -> Combination {
        let mut c = Combination(self.0 ^ b.0, self.1.clone());
        c.1 = c.1 | b.1.clone();
        c
    }
}
pub type SearchRes = Option<Combination>;