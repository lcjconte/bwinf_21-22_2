#![allow(non_snake_case)]
use std::error::Error;
use std::io::{self, BufRead};
use std::fs::File;
use std::ops::BitOr;
use std::path::Path;
use std::collections::HashMap;
use ahash::RandomState;

pub const MAXN: usize = 256;
pub const MAXK: usize = 20;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Clone)]
pub struct TInput {
    pub n: i32,
    pub k: i32,
    pub m: i32,
    pub nums: Vec<u128>,
}
impl TInput {
    fn new() -> TInput {
        TInput {n: 0, k: 0, m: 0, nums: vec![]}
    }
    pub fn read_from(file_name: &str) -> Result<TInput, Box<dyn Error>> {
        let mut input = TInput::new();
        for (idx, line) in read_lines(file_name)?.enumerate() {
            match idx {
                0 => {
                    let parts: Vec<String> = line?.split(" ").map(|s| s.to_string()).collect();
                    input.n = parts[0].trim().parse()?;
                    input.k = parts[1].trim().parse()?;
                    input.m = parts[2].trim().parse()?;
                    input.nums.resize(input.n as usize, 0);
                },
                _ => {
                    input.nums[idx-1] = u128::from_str_radix(&line?, 2)?
                }
            }
        }
        Ok(input)
    }
}

pub struct TOutput {
    pub nums: Vec<u128>,
}
impl TOutput {
    pub fn verify(&self) -> bool{
        let mut a = 0;
        for i in &self.nums {
            a ^= i;
        }
        a == 0
    }
}
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
        &mut self.underlying[y*self.bounds.0+x]
    }
    pub fn get2(&self, x: usize, y: usize) -> T {
        self.underlying[y*self.bounds.0+x]
    }
    pub fn get3_mut(&mut self, x: usize, y: usize, z: usize) -> &mut T{
        &mut self.underlying[z*self.bounds.1*self.bounds.0+y*self.bounds.0+x]
    }
    pub fn get3(&self, x: usize, y: usize, z: usize) -> T{
        self.underlying[z*self.bounds.1*self.bounds.0+y*self.bounds.0+x]
    }
}

/// A 256 unsigned int with limited functionality
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
pub struct u256(u128, u128);
impl u256 {
    pub fn set(&mut self, mut idx: usize, v: bool) {
        let ed: &mut u128 = if idx >= 128 {
            idx -= 128;
            &mut self.1
        }else {
            &mut self.0
        };
        *ed |= 1 << idx;
    }
    pub fn get(&self, mut idx: usize) -> Option<bool>{
        let ed: &u128 = if idx >= 128 {
            idx -= 128;
            &self.1
        }else {
            &self.0
        };
        Some((*ed & (1 << idx))>0)
    }
    pub fn zero() -> Self {
        u256(0, 0)
    }
}
impl BitOr for u256 {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0, self.1 | rhs.1)
    }
}

pub fn get_xor(nums: &[u128], b: &u256) -> u128 {
    let mut d = 0;
    for i in 0..nums.len() {
        if b.get(i).unwrap() {
            d ^= nums[i];
        }
    }
    d
}

pub trait ISolver<'a> {
    fn new() -> Self;
    fn process(&mut self, t_input: &'a TInput) -> Option<TOutput>;
}

pub trait CombStore {
    fn new(size: usize) -> Self;
    fn insert(&mut self, k: u128, v: u256) -> ();
    fn get(&mut self, k: u128) -> Option<u256>;
    fn clear(&mut self) -> ();
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
        self.0.get(&k).map(|x| {x.clone()})
    }
    fn clear(&mut self) {
        self.0.clear();
    }
}

#[derive(Debug, Clone)]
pub struct Combination(pub u128, pub u256);
impl Combination {
    pub fn new() -> Self{
        Combination(0, u256::zero())
    }
    pub fn add(&self, b: u128, idx: usize) -> Combination {
        let mut c = Combination(self.0 ^ b, self.1.clone());
        c.1.set(idx, true);
        c
    }
    pub fn combine(&self, b: &Combination) -> Combination {
        let mut c = Combination(self.0 ^ b.0, self.1.clone());
        c.1 = c.1 | b.1.clone();
        c
    }
}