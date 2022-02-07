
mod common;
use std::{f64::INFINITY, collections::BTreeMap, ops::Add, collections::HashMap};
use std::cmp::min;
use common::*;

struct Context<'a> {
    s: &'a Vec<String>,
    chars: &'a Characters,
    dp: &'a mut HashMap<(i64, i64), f64>,
}

fn balance_cost(ctx: &mut Context, k: i64, bal: i64) -> f64{
    if ctx.dp.contains_key(&(k, bal)) {
        return ctx.dp[&(k, bal)];
    }
    let mut cmin: f64 = INFINITY;
    for c in ctx.chars.chars.iter().rev() {
        let effect = ctx.chars.char_transform(&ctx.s[k as usize], &c);
        let nbal = bal + effect.req;
        let mut cost = effect.cost;
        if k == (ctx.s.len()-1) as i64 {
            if nbal == 0 {
                cmin = f64::min(cmin, cost);
            }
        }
        else {
            cost += balance_cost(ctx, k+1, nbal);
            cmin = f64::min(cmin, cost);
        }
    }
    ctx.dp.insert((k, bal), cmin);
    cmin
}

fn process(tInput: TInput) {
    let chars = Characters::read_from("proto/Aufgabe3/chars.json").unwrap();
    let s: Vec<String> = tInput.s.as_bytes().to_vec().iter().map(|&x| {
        (x as char).to_string()
    }).collect();
    let mut dp = HashMap::new();
    let mut context = Context {s: &s, chars: &chars, dp: &mut dp};
    let n: i64 = s.len().try_into().unwrap();
    let (mut cbal, mut cost) = (0, 0 as f64);
    let mut nString: Vec<String> = vec![];
    println!("Starting ...");
    for i in 0..n {
        for c in chars.chars.iter().rev() {
            let effect = chars.char_transform(&s[i as usize], &c);
            let nbal = cbal + effect.req;
            let ncost = cost + effect.cost;
            let mut changed = false;
            if i == n-1 {
                if ncost <= tInput.changes as f64 && nbal == 0 {
                    nString.push(c.to_string());
                    changed = true;
                }
            }
            else {
                let complcost = balance_cost(&mut context, i+1, nbal);
                if ncost+complcost <= tInput.changes as f64 {
                    nString.push(c.to_string());
                    changed = true;
                }
            }
            if changed {
                cost = ncost;
                cbal = nbal;
                break;
            }
        }
    }
    let res = nString.join("");
    println!("Processed!");
    println!("Result: {}", &res);
}
fn addV<T>(a: &(T, T), b: &(T, T)) -> (T, T)
where T: Add<Output = T> + Copy {
    (a.0+b.0, a.1+b.1)
}
fn bal_zero(b: &BVec) -> bool {
    b.0 == 0 && b.1 == 0
}

fn process2(tInput: TInput) {
    let chars = Characters::read_from("proto/Aufgabe3/chars.json").unwrap();
    let s: Vec<String> = tInput.s.as_bytes().to_vec().iter().map(|&x| {
        (x as char).to_string()
    }).collect();
    let mut dp = HashMap::new();
    let mut context = Context {s: &s, chars: &chars, dp: &mut dp};
    let n: i64 = s.len().try_into().unwrap();
    let (mut cbal, mut cost) = (BVec(0, 0), 0 as f64);
    let mut nString: Vec<String> = vec![];
    println!("Starting ...");
    for i in 0..n {
        for c in chars.chars.iter().rev() {
            let effect = chars.char_balance(&s[i as usize], &c);
            let mut nbal = cbal+effect;
            let overlap = min(nbal.0, nbal.1);
            let mut ncost = cost + overlap as f64;
            nbal.0 -= overlap; nbal.1 -= overlap;
            let mut changed = false;
            if i == n-1 {
                if ncost <= tInput.changes as f64 && bal_zero(&nbal) {
                    nString.push(c.to_string());
                    changed = true;
                }
            }
            else {
                ncost += balance_cost2(&mut context, i+1, nbal.1-nbal.0);
                if ncost <= tInput.changes as f64 {
                    nString.push(c.to_string());
                    changed = true;
                }
            }
            if changed {
                cost = ncost;
                cbal = nbal;
                break;
            }
        }
    }
    let res = nString.join("");
    println!("Processed!");
    println!("Result: {}", &res);
}

fn balance_cost2(ctx: &mut Context, k: i64, bal: i64) -> f64{
    if ctx.dp.contains_key(&(k, bal)) {
        return ctx.dp[&(k, bal)];
    }
    let cbal = if bal < 0 {BVec(-bal, 0)} else {BVec(0, bal)};
    let mut cmin: f64 = INFINITY;
    for c in ctx.chars.chars.iter().rev() {
        let effect = ctx.chars.char_balance(&ctx.s[k as usize], &c);
        let mut nbal = cbal+effect;
        let overlap = min(nbal.0, nbal.1);
        let mut cost = overlap as f64;
        nbal.0 -= overlap; nbal.1 -= overlap;
        if k == (ctx.s.len()-1) as i64 {
            if bal_zero(&nbal) {
                cmin = f64::min(cmin, cost);
            }
        }
        else {
            cost += balance_cost2(ctx, k+1, nbal.1-nbal.0);
            cmin = f64::min(cmin, cost);
        }
    }
    ctx.dp.insert((k, bal), cmin);
    cmin
}

fn main() {
    let tInput = TInput::read_from("eingaben/Aufgabe3/hexmax5.txt").unwrap();
    process(tInput);
}