use std::collections::HashMap;
use std::fs::File;
use std::error::Error;
use std::io::{BufReader, Read, BufRead};
use std::path::{Path, PathBuf};
use serde_json::Value;

/// Returns manifest path plus argument
pub fn manifest_plus<P: AsRef<Path>>(plus: P) -> PathBuf {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push(plus);
    d
}

#[derive(Clone)]
pub struct TInput {
    pub m: u64,
    pub s: String
}

impl TInput {
    pub fn read_from(file_name: &Path) -> Result<Self, Box<dyn Error>>{
        let mut obj = TInput { m: 0, s: String::new() };
        let file = File::open(file_name)?;
        let mut buf = String::new();
        let mut reader = BufReader::new(file);
        reader.read_line(&mut buf)?;
        obj.s = buf.trim().to_string();
        buf.clear();
        reader.read_line(&mut buf)?;
        obj.m = buf.trim().parse()?;
        Ok(obj)
    }
}
pub struct Step {
    pub from: (usize, usize),
    pub to: (usize, usize),
    pub result: Vec<u32>,
}
pub struct TOutput {
    pub input: TInput,
    pub s: String,
    pub steps: Option<Vec<Step>>,
    /// Processing runtime in ms
    pub runtime: u128,
}
#[derive(Clone)]
pub struct Character {
    pub bits: u32,
    pub display: String,
}
pub struct Characters {
    pub positions: i64,
    pub chars: Vec<Character>,
    pub from_disp: HashMap<String, Character>,
    pub from_bits: HashMap<u32, Character>
}

/// Converts bitstring to u32
pub fn to_u32(s: &str) -> u32 {
    let s = s.as_bytes();
    let mut res = 0;
    for i in 0..s.len() {
        res += (s[i] == b'1') as u32 *(1 << i);
    }
    res
}

impl Characters {
    pub fn read_from(file_name: &Path) -> Result<Self, Box<dyn Error>> {
        let file = File::open(file_name)?;
        let mut s = String::new();
        BufReader::new(file).read_to_string(&mut s)?;
        let json: Value = serde_json::from_str(&s)?;
        let mut chars = Characters {
            positions: 0, 
            chars: vec![], 
            from_disp: HashMap::new(),
            from_bits: HashMap::new(),
        };
        let em = "Unexpected format";
        chars.positions = json["positions"].as_i64().ok_or(em)?;
        let combs = json["combinations"].as_object().ok_or(em)?;
        for pair in combs.into_iter() {
            let c = Character {bits: to_u32(pair.1.as_str().ok_or(em)?), display: pair.0.to_string()};
            chars.chars.push(c);
            let c = chars.chars.last().unwrap();
            chars.from_bits.insert(c.bits, c.clone());
            chars.from_disp.insert(c.display.clone(), c.clone());
        }
        Ok(chars)
    }
}