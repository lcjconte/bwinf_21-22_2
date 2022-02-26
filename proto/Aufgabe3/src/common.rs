use std::cmp::min;
use std::collections::HashMap;
use std::fs::File;
use std::error::Error;
use std::io::{BufReader, Read, BufRead};
use std::ops::Add;
use serde_json::Value;

pub struct Characters {
    pub positions: i64,
    pub chars: Vec<String>,
    pub char_rep: HashMap<String, String>
}
pub struct TEffect {
    pub req: i64,
    pub cost: f64,
}

#[derive(Copy, Clone)]
pub struct BVec(pub i64, pub i64); //Balance vector

impl Add for BVec {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            0: self.0 + other.0,
            1: self.1 + other.1,
        }
    }
}

impl Characters {
    pub fn read_from(file_name: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(file_name)?;
        let mut s = String::new();
        BufReader::new(file).read_to_string(&mut s)?;
        let json: Value = serde_json::from_str(&s)?;
        let mut chars = Characters {
            positions: 0, 
            chars: vec![], 
            char_rep: HashMap::new()
        };
        let em = "Unexpected format";
        chars.positions = json["positions"].as_i64().ok_or(em)?;
        let combs = json["combinations"].as_object().ok_or(em)?;
        for pair in combs.into_iter() {
            chars.chars.push(pair.0.to_string());
            chars.char_rep.insert(chars.chars.last().unwrap().to_string(), pair.1.as_str().ok_or(em)?.to_string());
        }
        Ok(chars)
    }
    pub fn char_transform(&self, a: &str, b: &str) -> TEffect{
        let balance = self.char_balance(a, b);
        TEffect { req: balance.1-balance.0, cost: min(balance.0, balance.1) as f64+((balance.1-balance.0) as f64/2.0).abs() }
    }
    pub fn char_balance(&self, a: &str, b: &str) -> BVec {
        let mut balance = BVec(0, 0);
        let (r1, r2) = (self.char_rep[a].as_str(), self.char_rep[b].as_str());
        for i in 0..r1.len() {
            if (r1.as_bytes()[i] as char, r2.as_bytes()[i] as char) == ('1', '0') {
                balance.0 += 1;
            }
            else if (r1.as_bytes()[i] as char, r2.as_bytes()[i] as char) == ('0', '1') {
                balance.1 += 1;
            }
        }
        balance
    }
    /*pub fn string_balance(&self, a: &str, b: &str) -> BVec {
        assert_eq!(a.len(), b.len());
        let s1 = a.as_bytes();
        let s2 = b.as_bytes();
        let mut balance = BVec(0, 0);
        for i in 0..a.len() {
            balance += self.char_balance(s1[i] as char, b)
        }
        unimplemented!()
        
    }*/
}

pub struct TInput {
    pub changes: i64,
    pub s: String
}

impl TInput {
    pub fn read_from(file_name: &str) -> Result<Self, Box<dyn Error>>{
        let mut obj = TInput { changes: 0, s: String::new() };
        let file = File::open(file_name)?;
        let mut buf = String::new();
        let mut reader = BufReader::new(file);
        reader.read_line(&mut buf)?;
        obj.s = buf.trim().to_lowercase();
        buf.clear();
        reader.read_line(&mut buf)?;
        obj.changes = buf.trim().parse()?;
        Ok(obj)
    }
}

