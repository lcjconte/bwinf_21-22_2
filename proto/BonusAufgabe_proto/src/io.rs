#![allow(non_snake_case)]
use std::error::Error;
use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

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
                    let parts: Vec<String> = line?.split(' ').map(|s| s.to_string()).collect();
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
    pub fn verify(&self) -> bool {
        let mut a = 0;
        for i in &self.nums {
            a ^= i;
        }
        a == 0
    }
}