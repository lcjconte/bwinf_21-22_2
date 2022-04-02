
mod common;
mod main2;
use std::{f64::INFINITY, ops::Add, collections::HashMap};
use std::cmp::min;
use common::*;
use main2::process as proc2;
/* 
struct Context<'a> {
    s: &'a Vec<String>,
    chars: &'a Characters<'a>,
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

*/
fn main() {
    let chars = Characters::read_from("proto/Aufgabe3/chars.json").unwrap();
    let input = TInput::read_from(&format!("eingaben/Aufgabe3/hexmax{}.txt", 0)).unwrap();
    let output = proc2(&input, &chars);
    println!("Processed!");
    println!("Valid: {:?}", output.verify(&chars));
    println!("Result: {}", output.s);
    let st = chars.string_steps(chars.stovec(&input.s), chars.stovec(&output.s));
    for step in st {
        println!("{:?} {:?}", step.from, step.to);
    }
    /*for i in 0..6 {
        let tInput = TInput::read_from(&format!("eingaben/Aufgabe3/hexmax{}.txt", i)).unwrap();
        proc2(tInput);
    }*/
    
}