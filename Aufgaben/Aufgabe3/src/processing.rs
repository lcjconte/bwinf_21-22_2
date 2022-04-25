use super::io::*;
use std::f64::INFINITY;
use std::cmp::{min, max};
use std::time::Instant;

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
    /// How does a -> b affect balance
    pub fn transform_balance(&self, a: &Character, b: &Character) -> (u64, u64) {
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
    pub fn transform_effect(&self, a: &Character, b: &Character) -> (f64, i64) {
        let balance = self.transform_balance(a, b);
        let minb = min(balance.0, balance.1);
        let maxb = max(balance.0, balance.1);
        (minb as f64 + 0.5*(maxb-minb) as f64, balance.1 as i64-balance.0 as i64)
    }
    /// Converts string to vec of characters
    pub fn stovec(&self, s: &str) -> Vec<&Character> {
        let s = s.as_bytes();
        let mut v = vec![];
        for c in s {
            v.push(&self.from_disp[&(*c as char).to_string()]);
        }
        v
    }
    /// Cost to transform a into b
    pub fn string_cost(&self, a: &Vec<&Character>, b: &Vec<&Character>) -> u64 {
        assert_eq!(a.len(), b.len());
        let mut balance = (0, 0);
        for i in 0..a.len() {
            let r = self.transform_balance(a[i], b[i]);
            balance = (balance.0+r.0, balance.1+r.1);
        }
        assert_eq!(balance.0, balance.1, "Strings cannot be transformed!");
        balance.0
    }
    /// Returns steps needed to transform a into b
    pub fn string_steps(&self, a: &[&Character], b: &[&Character]) -> Vec<Step> {
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
            idx.1 += 1;
            if idx.1 == self.positions as usize {
                while !insert_here.is_empty() && !take_here.is_empty() {  // Assign pairs only after having visited all segments of character
                    let from = take_here.pop().unwrap();
                    let to = insert_here.pop().unwrap();
                    set_bit(&mut a[from.0], from.1, 0);
                    set_bit(&mut a[to.0], to.1, 1);
                    v.push(Step {from, to, result: a.clone()});
                }
                idx.0 += 1;
                idx.1 = 0;
            }
        }
        v
    }
}

struct Context<'a> {
    s: &'a Vec<&'a Character>,
    chars: &'a Characters,
    dp: &'a mut Vec<f64>
}
impl<'a> Context<'a> {
    fn get_dp(&mut self, k: usize, bal: i64) -> &mut f64 {
        let balsize = self.s.len() as i64*self.chars.positions;
        &mut self.dp[(2*k as i64*balsize+bal) as usize+balsize as usize] 
    }
}

/// Cost to balance suffix
fn balancing_cost(ctx: &mut Context, i: usize, bal: i64) -> f64 {
    if i == ctx.s.len() {
        if bal==0 {
            return 0.0;
        }
        return INFINITY;
    }
    if *ctx.get_dp(i, bal) != -1.0 {
        return *ctx.get_dp(i, bal);
    }
    let mut cmin: f64 = INFINITY;
    for c in ctx.chars.chars.iter().rev() {
        let effect = ctx.chars.transform_effect(ctx.s[i], c);
        cmin = f64::min(cmin, balancing_cost(ctx, i+1, bal+effect.1)+effect.0);
    }
    *ctx.get_dp(i, bal) = cmin;
    cmin
}

pub fn process(input: &TInput, chars: &Characters, include_steps: bool) -> TOutput {
    let start_time = Instant::now();
    let n = input.s.len();
    let mut dp = vec![-1.0; n*n*chars.positions as usize*2];
    let mut context = Context {s: &chars.stovec(&input.s), chars, dp: &mut dp};
    let (mut cbal, mut cost) = (0, 0 as f64);
    let mut n_string: Vec<&Character> = vec![];
    for i in 0..n {
        for c in chars.chars.iter().rev() { //Reverse to iterate in descending order
            let effect = chars.transform_effect(context.s[i], c);
            let nbal = cbal + effect.1;
            let ncost = cost + effect.0;
            if ncost+balancing_cost(&mut context, i+1, nbal) <= input.m as f64 {
                n_string.push(c);
                cost = ncost;
                cbal = nbal;
                break;
            }
        }
    }
    let res: Vec<String> = n_string.iter().map(|x| {x.display.to_string()}).collect();
    let res = res.join("");
    let mut steps: Option<Vec<Step>> = None;
    if include_steps {
        steps = Some(chars.string_steps(context.s, &n_string));
    }
    TOutput {input: input.to_owned(), s: res, initial: context.s.iter().map(|x| {x.bits}).collect(), steps, runtime: start_time.elapsed().as_millis()}
}

impl TOutput {
    /// Verifies s
    pub fn verify(&self, chars: &Characters) -> bool{
        chars.string_cost(&chars.stovec(&self.input.s), &chars.stovec(&self.s)) <= self.input.m
    }
}