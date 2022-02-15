#[allow(non_snake_case)]
use std::error::Error;
use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

pub const MAXN: usize = 200;
pub const MAXK: usize = 20;
pub const MAXM: usize = 128;

#[derive(Clone)]
pub struct TInput {
    pub n: i32,
    pub k: i32,
    pub m: i32,
    pub nums: Vec<u128>,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
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

pub enum Var<'a, T> {
    Mut(&'a mut T),
    Const(&'a T),
}