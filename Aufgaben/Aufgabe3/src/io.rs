use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::error::Error;
use std::io::{BufReader, Read, BufRead};
use std::path::Path;
use serde_json::Value;

#[derive(Clone)]
pub struct TInput {
    pub m: u64,
    pub s: String
}

impl TInput {
    pub fn read_from<T: AsRef<Path>>(file_name: T) -> Result<Self, Box<dyn Error>>{
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
impl fmt::Display for TInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.s)?;
        write!(f, "{}", self.m)?;
        Ok(())
    }
}
/// Single segment movement
pub struct Step {
    /// (character idx, segment idx) Set character idx to u32::MAX to -1
    pub from: (usize, usize), 
    /// (character idx, segment idx)
    pub to: (usize, usize),
    pub result: Vec<u32>,
}
pub struct TOutput {
    pub input: TInput,
    pub s: String,
    /// Initial segment config for convenience
    pub initial: Vec<u32>,
    pub steps: Option<Vec<Step>>,
    /// Processing runtime in ms
    pub runtime: u128,
}

impl fmt::Display for TOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.s)?;
        writeln!(f, "{}", self.runtime)?;
        if let Some(ref v) = self.steps {
            writeln!(f, "{:?} {:?} {}", (-1, -1), (-1, -1), self.initial.iter().map(|x| {x.to_string()}).collect::<Vec<String>>().join(" "))?;
            for step in v {
                writeln!(f, "{:?} {:?} {}", step.from, step.to, step.result.iter().map(|x| {x.to_string()}).collect::<Vec<String>>().join(" "))?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Character {
    pub bits: u32,
    pub display: String,
}
pub struct Characters {
    pub positions: i64,
    pub chars: Vec<Character>,
    /// ascii char to object
    pub from_disp: HashMap<String, Character>,
    /// segment representation to object
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
    pub fn read_from<T: AsRef<Path>>(file_name: T) -> Result<Self, Box<dyn Error>> {
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