#![allow(dead_code)]
use std::{collections::{HashMap}, mem::swap};
use std::cell::Cell;
use rand::Rng;
use bit_vec::BitVec;
//use primitive_types::U256;
use std::time::Instant;

use super::common::*;

struct Solver {
    t_input: Option<TInput>,
    pascal: Vec<Vec<u128>>,
    binom_sum: Vec<Vec<u128>>,
    cost_dp: Vec<Vec<Cell<u128>>>,
}

impl Solver {
    fn new() -> Solver {
        let mut solver = Solver { 
            t_input: None,
            pascal: vec![vec![0;MAXK+2];MAXN+2], //TODO: Optimize space
            binom_sum: vec![vec![0;MAXK+2];MAXN+2],
            cost_dp: vec![vec![Cell::new(0);MAXK+2];MAXN+2],
        };
        solver.init();
        solver
    }
    /// Initializes solver 
    fn init(&mut self) {
        //Calculate pascal triangle
        self.pascal[0][0] = 1;
        for n in 1..MAXN+1 {
            self.pascal[n][0] = 1;
            if n <= MAXK {
                self.pascal[n][n] = 1;
            }
            for k in 1..n.min(MAXK)+1 {
                self.pascal[n][k] = self.pascal[n-1][k]+self.pascal[n-1][k-1];
            }
        }
        //Calculate enum_cost
        for n in 0..MAXN+1 {
            for k in 0..MAXK+1 {
                let mut res = 0;
                for i in 0..k+1 {
                    res += self.binom(n, i);
                }
                self.binom_sum[n][k] = res;
            }
        }
    }
    /// Binomial coefficient \ 
    /// n <= MAXN and k <= MAXK 
    fn binom(&self, n: usize, k: usize) -> u128 {
        assert!(n <= MAXN && k <= MAXK);
        self.pascal[n][k]
    }
    fn space_cost(&self, n: usize, k: usize) -> u128 {
        self.binom(n, k)
    }
    fn enum_cost(&self, n: usize, k: usize) -> u128 {
        assert!(n <= MAXN && k <= MAXK);
        self.binom_sum[n][k]
    }
    fn estimate_cost(&self, n: usize, k: usize, space_limit: usize, recursive: bool) -> u128 {
        if n == 1 {
            return 1;
        }
        if self.cost_dp[n][k].get() != 0 {
            return self.cost_dp[n][k].get();
        }
        let mut res: u128 = 1;
        let sl = (n as f64/2.0).ceil() as usize;
        let sr = (n as f64/2.0).floor() as usize;
        for l in (if k >= sr {k-sr} else {0})..sl.min(k)+1 {
            let r = k-l;
            let pairs = vec![(sl, l), (sr, r)];
            res += self.best_action(&pairs, space_limit, recursive).0;
        }
        self.cost_dp[n][k].set(res);
        res
    }
    fn best_action(&self, pairs: &Vec<(usize, usize)>, space_limit: usize, recursive: bool) -> (u128, usize, usize) {
        let mut mres = (u128::MAX, 0, 0);
        for i in 0..2 {
            let it_p = pairs[i];
            let alt_p = pairs[1-i];
            let it_tcost = self.enum_cost(it_p.0, it_p.1);
            //
            let alt_tcost = self.enum_cost(alt_p.0, alt_p.1);
            let alt_scost = self.space_cost(alt_p.0, alt_p.1);
            if alt_scost <= space_limit as u128 {
                mres = mres.min((it_tcost+alt_tcost, i, 0));
            }
            //
            if recursive {
                mres = mres.min((it_tcost*self.estimate_cost(alt_p.0, alt_p.1, space_limit, recursive), i, 1));
            }
            //mres = mres.min((it_tcost*alt_tcost, i, 2));
        }
        mres
    }
}

struct Context {
    comb_set: HashMap<u128, BitVec>,
}

#[derive(Debug, Clone)]
struct Combined(u128, BitVec);

impl Combined {
    fn new() -> Self{
        Combined(0, BitVec::from_elem(MAXN+1, false))
    }
    fn add(&self, b: u128, idx: usize) -> Combined {
        let mut c = Combined(self.0 ^ b, self.1.clone());
        c.1.set(idx, true);
        c
    }
    fn combine(&self, b: &Combined) -> Combined {
        let mut c = Combined(self.0 ^ b.0, self.1.clone());
        c.1.or(&b.1);
        c
    }
}

fn enum_combs(nums: &[u128], k: usize, func: &mut dyn FnMut(Combined) -> (), start: usize, end: usize, cur: Combined) {
    assert!(end <= nums.len());
    if k == 0 {
        func(cur);
        return;
    }
    if start == end {return;}
    for i in start..end {
        enum_combs(nums, k-1, func, i+1, end, cur.add(nums[i], i));
    }
}

impl Solver {
    fn explore(&self, context: &mut Context, lo: usize, hi: usize, k: usize, target: u128, recursive: bool) -> Option<Combined> {
        let nums = &self.t_input.as_ref().unwrap().nums;
        let space_limit = context.comb_set.capacity();
        let n = hi-lo+1;
        let sl = (n as f64/2.0).ceil() as usize;
        let sr = (n as f64/2.0).floor() as usize;
        let mut res: Option<Combined> = None;
        for l in (if k >= sr {k-sr} else {0})..sl.min(k)+1 {
            let r = k-l;
            let mut pairs = vec![(sl, l), (sr, r)];
            let action = self.best_action(&pairs, space_limit, recursive);
            let (mut it_start, mut alt_start) = (lo, lo+sl);
            if action.1 > 0 {pairs.swap(0, 1);swap(&mut it_start, &mut alt_start)}
            match action.2 {
                0 => {
                    context.comb_set.clear();
                    if alt_start+pairs[1].0 > hi+1 {
                        println!("{} {} {} {}", &lo, &hi, &sl, &sr);
                        println!("{} {:?}", &alt_start, pairs);
                        let a = 1;
                    }
                    enum_combs(nums, pairs[1].1, &mut |x| {context.comb_set.insert(x.0, x.1);}, alt_start, alt_start+pairs[1].0, Combined::new());
                    let mut it_func = |x: Combined| {
                        let compl = x.0 ^ target;
                        match context.comb_set.get(&compl) {
                            Some(c) => res = Some(x.combine(&Combined(compl, c.clone()))),
                            None => ()
                        }
                    };
                    enum_combs(nums, pairs[0].1, &mut it_func, it_start, it_start+pairs[0].0, Combined::new());
                }
                1 => {
                    let mut it_func = |x: Combined| {
                        let compl = x.0 ^ target;
                        match self.explore(context, alt_start, alt_start+pairs[1].0-1, pairs[1].1, compl, recursive) {
                            Some(c) => res = Some(x.combine(&c)),
                            None => ()
                        }
                    };
                    enum_combs(nums, pairs[0].1, &mut it_func, it_start, it_start+pairs[0].0, Combined::new());
                }
                _ => {
                    unimplemented!()
                }
            }
            if let Some(c) = res {
                return Some(c);
            } 
        }
        return None;
    }
    fn process(&mut self, t_input: TInput) {
        self.t_input = Some(t_input);
        let mut context = Context {comb_set: HashMap::with_capacity(1e7 as usize)};
        let n = self.t_input.as_ref().unwrap().n as usize;
        let k = (self.t_input.as_ref().unwrap().k+1) as usize;
        let res = self.explore(&mut context, 0, n-1, k, 0, true);
        if let Some(c) = res {
            println!("Found!");
            println!("{}", c.0);
            let mut v: Vec<u128> = vec![];
            for i in 0..n {
                if c.1.get(i).unwrap() {
                    v.push(self.t_input.as_ref().unwrap().nums[i]);
                    println!("{}", self.t_input.as_ref().unwrap().nums[i]);
                }
            }
            println!("{}", v.len());
            let tOutput = TOutput {nums: v};
            println!("{}", tOutput.verify());
        }
        else {
            println!("Not found!");
        }
        
    }
}

fn main() {
    let tInput = TInput::read_from("eingaben/BonusAufgabe/stapel2.txt").unwrap();
    let mut solver = Solver::new();
    /*let n = tInput.n as usize;
    let k = (tInput.k+1) as usize;
    solver.t_input = Some(tInput);
    let mut context = Context {comb_set: HashMap::with_capacity(1e8 as usize)};
    let res = solver.explore(&mut context, 0, n-1, k, 0, false);
    //enum_combs(&vec![17, 2, 3], 3, &mut |x| {println!("{}", x)}, 0, 0);*/
    let now = Instant::now();
    solver.process(tInput);
    let elapsed_time = now.elapsed();
    println!("Running slow_function() took {} seconds.", elapsed_time.as_secs());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        let solver = Solver::new();
        assert_eq!(solver.binom(8, 4), 70);
        assert_eq!(solver.binom(76, 13), 152724276564800);
        assert_eq!(solver.enum_cost(55, 12), 595443690122);
        assert_eq!(solver.enum_cost(35, 4), 59536);
        assert_eq!(solver.estimate_cost(111, 11, 1e8 as usize, true), 22856129090);
        assert_eq!(solver.estimate_cost(69, 16, 1e8 as usize, true), 11565982879);
    }
    #[test]
    fn test_enum_combs() {
        let solver = Solver::new();
        for _ in 0..10 {
            let si = rand::thread_rng().gen_range(1..20);
            let mut v: Vec<u128> = Vec::with_capacity(si);
            for i in 0..si {
                v.push(rand::thread_rng().gen_range(1..1000));
            }
            let mut counter = 0;
            let k = rand::thread_rng().gen_range(0..si+1);
            enum_combs(&v, k, &mut |x| {counter += 1;}, 0, si, Combined::new());
            assert_eq!(counter, solver.binom(si, k));
        }
    }
}