#![allow(non_snake_case)]
use std::error::Error;
use std::fmt::{self, Display};
use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

pub const MAXN: usize = 255;
pub const MAXK: usize = 20;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Clone, Default)]
pub struct TInput {
    pub n: i32,
    pub k: i32,
    pub m: i32,
    pub nums: Vec<u128>,
}
impl TInput {
    pub fn read_from(file_name: &str) -> Result<TInput, Box<dyn Error>> {
        let mut input = TInput::default();
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
    /// Runtime in ms
    pub runtime: u128,
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
impl Display for TOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //self.nums.sort();
        for (idx, el) in self.nums.iter().enumerate() {
            write!(f, "{}{}", el, if idx==self.nums.len()-1 {""} else {"\n"})?;
        }
        Ok(())
    }
}