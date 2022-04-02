use std::{f64::INFINITY, ops::{Add, Deref}, collections::HashMap};
use super::common::*;

struct Context<'a> {
    s: &'a Vec<&'a Character>,
    chars: &'a Characters,
    dp: &'a mut HashMap<(usize, i64), f64>,
}

/// Cost to balance suffix
fn balancing_cost(ctx: &mut Context, k: usize, bal: i64) -> f64 {
    if k == ctx.s.len() {
        if bal==0 {
            return 0.0;
        }
        return INFINITY;
    }
    if ctx.dp.contains_key(&(k, bal)) {
        return ctx.dp[&(k, bal)];
    }
    let mut cmin: f64 = INFINITY;
    for c in ctx.chars.chars.iter().rev() {
        let effect = ctx.chars.conversion_effect(ctx.s[k], c);
        cmin = f64::min(cmin, balancing_cost(ctx, k+1, bal+effect.1)+effect.0);
    }
    ctx.dp.insert((k, bal), cmin);
    cmin
}

pub fn process(input: &TInput, chars: &Characters) -> TOutput {
    let mut dp = HashMap::new();
    let mut context = Context {s: &chars.stovec(&input.s), chars: &chars, dp: &mut dp};
    let n = input.s.len();
    let (mut cbal, mut cost) = (0, 0 as f64);
    let mut n_string: Vec<&Character> = vec![];
    println!("Starting ...");
    for i in 0..n {
        for c in chars.chars.iter().rev() {
            let effect = chars.conversion_effect(context.s[i], c);
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
    TOutput {input: input.to_owned(), s: res, steps: None}
}