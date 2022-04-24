#![allow(non_snake_case)]
use std::error::Error;
use std::fmt::{self, Display};
use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;
use serde::{Serialize, Deserialize};

pub const MAXN: usize = 256;
/// k = N-MAXK also possible!
pub const MAXK: usize = 20;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TInput {
    pub n: usize,
    pub k: usize,
    pub m: usize,
    pub nums: Vec<u128>,
}
impl TInput {
    pub fn read_from<P: AsRef<Path>>(file_name: P) -> Result<TInput, Box<dyn Error>> {
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

#[derive(Clone, Serialize, Deserialize)]
pub struct TOutput {
    pub input: TInput,
    pub nums: Vec<u128>,
    /// Runtime in ms
    pub runtime: u128,
}

impl Display for TOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.runtime)?;
        for (idx, el) in self.nums.iter().enumerate() {
            if self.input.m <= 32 {
                write!(f, "{:0>32b}{}", el, if idx==self.nums.len()-1 {""} else {"\n"})?;
            }
            else if self.input.m <= 64 {
                write!(f, "{:0>64b}{}", el, if idx==self.nums.len()-1 {""} else {"\n"})?;
            }
            else {
                write!(f, "{:0>128b}{}", el, if idx==self.nums.len()-1 {""} else {"\n"})?;
            }
        }
        Ok(())
    }
}

// Additional code for client/server extension
use crate::structs::SearchRes;
use hyper::{Body, body::HttpBody};

/// Assignment to process shift
#[derive(Serialize, Deserialize)]
pub struct ShiftAssignment(pub u32);

/// Result of single shift processing (uuid, result, shift)
#[derive(Serialize, Deserialize)]
pub struct ShiftResult(pub SearchRes, pub u32);

pub async fn get_json<T: serde::de::DeserializeOwned>(mut body: Body) -> Result<T, Box<dyn std::error::Error + Send + Sync>> {
    let mut data = Vec::with_capacity(body.size_hint().lower() as usize);
    while let Some(chunk) = body.data().await {
        data.extend(&chunk?);
    }
    let re = serde_json::from_slice(&data);
    Ok(re?)
}

/// Shorthand for .try_into().unwrap()
#[macro_export]
macro_rules! conv {
    ($a:expr) => {
        $a.try_into().unwrap()
    };
}