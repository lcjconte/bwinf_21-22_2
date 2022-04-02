use std::cmp::{min, max};
use std::collections::HashMap;
use std::fs::File;
use std::error::Error;
use std::io::{BufReader, Read, BufRead};
use serde_json::Value;

pub struct Characters {
    pub positions: i64,
    pub chars: Vec<Character>,
    pub from_disp: HashMap<String, Character>,
    pub from_bits: HashMap<u32, Character>
}
#[derive(Clone)]
pub struct Character {
    pub bits: u32,
    pub display: String,
}

pub struct Step {
    pub from: (usize, usize),
    pub to: (usize, usize),
    pub result: Vec<u32>,
}

fn get_char(s: &str, idx: usize) -> char{
    s.as_bytes()[idx] as char
}

/// Converts bitstring to u32
pub fn to_u32(s: &str) -> u32 {
    let s = s.as_bytes();
    let mut res = 0;
    for i in 0..s.len() {
        res += (s[i] == b'1') as u32 *(1 << /*(s.len()-i-1)*/i);
    }
    res
}

fn get_bit(a: u32, idx: usize) -> bool {
    (a & (1 << idx)) > 0
}

fn set_bit(a: &mut u32, idx: usize, val: u8) {
    if val == 0 {
        *a &= !(1 << idx);
    }
    else {
        *a |= 1 << idx;
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
    /// How does a -> b affect balance
    pub fn conversion_balance(&self, a: &Character, b: &Character) -> (u64, u64) {
        let mut balance = (0, 0);
        let (r1, r2) = (a.bits, b.bits);
        for i in 0..self.positions as usize {
            if (get_bit(r1, i), get_bit(r2, i)) == (false, true) {
                balance.1 += 1;
            }
            else if (get_bit(r1, i), get_bit(r2, i)) == (true, false) {
                balance.0 += 1;
            }
        }
        balance
    }
    /// What is the cost and balance change of a -> b
    pub fn conversion_effect(&self, a: &Character, b: &Character) -> (f64, i64) {
        let balance = self.conversion_balance(a, b);
        let minb = min(balance.0, balance.1);
        let maxb = max(balance.0, balance.1);
        (minb as f64 + 0.5*(maxb-minb) as f64, balance.1 as i64-balance.0 as i64)
    }
    pub fn stovec(&self, s: &str) -> Vec<&Character> {
        let s = s.as_bytes();
        let mut v = vec![];
        for c in s {
            v.push(&self.from_disp[&(*c as char).to_string()]);
        }
        v
    }
    /// Cost to transform a into b
    pub fn string_cost(&self, a: Vec<&Character>, b: Vec<&Character>) -> u64 {
        assert_eq!(a.len(), b.len());
        let mut balance = (0, 0);
        for i in 0..a.len() {
            let r = self.conversion_balance(a[i], b[i]);
            balance = (balance.0+r.0, balance.1+r.1);
        }
        assert_eq!(balance.0, balance.1, "Strings cannot be transformed!");
        balance.0
    }
    /// Returns steps needed to transform a into b
    pub fn string_steps(&self, a: Vec<&Character>, b: Vec<&Character>) -> Vec<Step> {
        let mut a: Vec<u32> = a.iter().map(|x| {x.bits}).collect();
        let mut b: Vec<u32> = b.iter().map(|x| {x.bits}).collect();
        let mut v = vec![];
        let mut idx = (0, 0);
        let mut take_here = vec![];
        let mut insert_here = vec![];
        while idx.0 != a.len() {
            if get_bit(a[idx.0], idx.1) != get_bit(b[idx.0], idx.1) {
                if (get_bit(a[idx.0], idx.1), get_bit(b[idx.0], idx.1)) == (false, true) {
                    insert_here.push(idx);
                }
                else if (get_bit(a[idx.0], idx.1), get_bit(b[idx.0], idx.1)) == (true, false) {
                    take_here.push(idx);
                }
            }
            while !insert_here.is_empty() && !take_here.is_empty() {
                let from = take_here.pop().unwrap();
                let to = insert_here.pop().unwrap();
                set_bit(&mut a[from.0], from.1, 0);
                set_bit(&mut b[to.0], to.1, 1);
                v.push(Step {from, to, result: a.clone()});
            }
            idx.1 += 1;
            if idx.1 == self.positions as usize {
                idx.0 += 1;
                idx.1 = 0;
            }
        }
        v
    }
}
#[derive(Clone)]
pub struct TInput {
    pub m: u64,
    pub s: String
}

impl TInput {
    pub fn read_from(file_name: &str) -> Result<Self, Box<dyn Error>>{
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

pub struct TOutput {
    pub input: TInput,
    pub s: String,
    pub steps: Option<Vec<Step>>
}

impl TOutput {
    pub fn verify(&self, chars: &Characters) -> bool{
        chars.string_cost(chars.stovec(&self.input.s), chars.stovec(&self.s)) <= self.input.m
    }
}