#![allow(dead_code)]
//use std::{collections::{HashMap}};
use rustc_hash::FxHashMap;
use bit_vec::BitVec;

use super::common::*;
use super::ISolver;

//[lo;hi)

pub struct Solver<'a> {
    t_input: Option<&'a TInput>,
    nums: Vec<u128>,
    pascal: DParray<u128>,
    binom_sum: DParray<u128>,
    cost_dp: DParray<u128>,
    cost_dp_params: Option<(usize, bool)>,
}

impl<'a> ISolver<'a> for Solver<'a> {
    fn new() -> Solver<'a> {
        let mut solver = Solver { 
            t_input: None,
            nums: vec![],
            pascal: DParray::new(0, MAXN, MAXK, 1),
            binom_sum: DParray::new(0, MAXN, MAXK, 1),
            cost_dp: DParray::new(0, MAXN, MAXN+1, MAXK),
            cost_dp_params: None,
        };
        solver.init();
        solver
    }
    fn process(&mut self, t_input: &'a TInput) -> Option<TOutput> {
        self.t_input = Some(t_input);
        self.nums = self.t_input.unwrap().nums.clone();
        self.nums.sort();
        let mut context = ExternalStorage {comb_set: FxHashMap::default()};
        context.comb_set.reserve(1e8 as usize);
        let n = self.t_input.unwrap().n as usize;
        let k = (self.t_input.unwrap().k+1) as usize;
        self.estimate_cost(0, n, k, 1e8 as usize, true);
        let res = self.explore(&mut context, 0, n, k, 0, true, 0);
        if let Some(c) = res {
            println!("Found!");
            println!("{}", c.0);
            let mut v: Vec<u128> = vec![];
            for i in 0..n {
                if c.1.get(i).unwrap() {
                    v.push(self.nums[i]);
                    println!("{}", self.nums[i]);
                }
            }
            println!("{}", v.len());
            let output = TOutput {nums: v};
            println!("{}", output.verify());
            Some(output)
        }
        else {
            println!("Not found!");
            None
        }
        
    }
}

fn msb(num: u128) -> u128 {
    let mut num = num;
    for e in 0..7 {
        num |= num >> (1 << e);
    }
    (num >> 1) + 1
}

fn get_partition(nums: &Vec<u128>, lo: usize, hi: usize) -> ((usize, usize), (usize, usize), u128) { 
    let n = hi-lo;
    if false {
        let sl = (n as f64/2.0).ceil() as usize;
        let sr = (n as f64/2.0).floor() as usize;
    ((lo, lo+sl), (lo+sl, hi), 0)
    }
    else {
        let a = msb(nums[lo]);
        let b = msb(nums[hi-1]);
        let mut up = b;
        let pl: (usize, usize);
        let pr: (usize, usize);
        let mut isdif = 0;
        loop {
            if (a & up) == 0 && (b & up ) != 0 {
                let mid = nums.partition_point(|x| {x < &up});
                pl = (lo, mid);
                pr = (mid, hi);
                isdif = up;
                break;
            }
            if up==1 {
                pl = (lo, lo+(n as f64/2.0).ceil() as usize);
                pr = (lo+(n as f64/2.0).ceil() as usize, hi);
                break;
            }
            up = up >> 1;
        }
        (pl, pr, isdif)
    }
}

impl<'a> Solver<'a> {
    /// Initializes solver 
    fn init(&mut self) {
        //Calculate pascal triangle
        *self.pascal.get2_mut(0, 0) = 1;
        for n in 1..MAXN+1 {
            *self.pascal.get2_mut(n, 0) = 1;
            if n <= MAXK {
                *self.pascal.get2_mut(0, 0) = 1;
            }
            for k in 1..n.min(MAXK)+1 {
                *self.pascal.get2_mut(n, k) = self.pascal.get2(n-1, k)+self.pascal.get2(n-1, k-1);
            }
        }
        //Calculate enum_cost
        for n in 0..MAXN+1 {
            for k in 0..MAXK+1 {
                let mut res = 0;
                for i in 0..k+1 {
                    res += self.binom(n, i);
                }
                *self.binom_sum.get2_mut(n, k) = res;
            }
        }
        //
    }
    /// Binomial coefficient \ 
    /// n <= MAXN and k <= MAXK 
    fn binom(&self, n: usize, k: usize) -> u128 {
        assert!(n <= MAXN && k <= MAXK);
        self.pascal.get2(n, k)
    }
    fn space_cost(&self, n: usize, k: usize) -> u128 {
        self.binom(n, k)
    }
    fn enum_cost(&self, n: usize, k: usize) -> u128 {
        assert!(n <= MAXN && k <= MAXK);
        self.binom_sum.get2(n, k)
    }
    pub fn estimate_cost(&mut self, lo: usize, hi: usize, k: usize, space_limit: usize, recursive: bool) -> u128 {
        assert!(lo < hi);
        self.cost_dp_params = Some((space_limit, recursive));
        let n = hi-lo;
        let nums = &self.nums;
        assert!(n <= MAXN && k <= MAXK);
        if n == 1 {
            return 1;
        }
        if self.cost_dp.get3(lo, hi, k) != 0 {
            return self.cost_dp.get3(lo, hi, k);
        }
        let mut res: u128 = 1;
        let (pl, pr, _) = get_partition(nums, lo, hi);
        let sl = pl.1-pl.0;let sr = pr.1-pr.0;
        for l in (if k >= sr {k-sr} else {0})..sl.min(k)+1 {
            let r = k-l;
            let pairs = vec![(pl, l), (pr, r)];
            res += (self.calculate_best_action(&pairs, space_limit, recursive).0 as f64/2.0).ceil() as u128;
        }
        *self.cost_dp.get3_mut(lo, hi, k) = res;
        res
    }
    fn estimated_cost(&self, lo: usize, hi: usize, k: usize) -> u128 {
        assert!(!self.cost_dp_params.is_none());
        self.cost_dp.get3(lo, hi, k)
    }
    fn calculate_best_action(&mut self, pairs: &Vec<((usize, usize), usize)>, space_limit: usize, recursive: bool) -> (u128, usize, usize) {
        let mut mres = (u128::MAX, 0, 0);
        for i in 0..2 {
            let it_p = pairs[i];
            let alt_p = pairs[1-i];
            let itl = it_p.0.1 - it_p.0.0;
            let altl = alt_p.0.1 - alt_p.0.0;
            let it_tcost = self.enum_cost(itl, it_p.1);
            let alt_tcost = self.enum_cost(altl, alt_p.1);
            let alt_scost = self.space_cost(altl, alt_p.1);
            if alt_scost <= space_limit as u128 {
                mres = mres.min((it_tcost+alt_tcost, i, 0));
            }
            if recursive {
                mres = mres.min((3*it_tcost*self.estimate_cost(alt_p.0.0, alt_p.0.1, alt_p.1, space_limit, recursive), i, 1));
            }
            //mres = mres.min((it_tcost*alt_tcost, i, 2));
        }
        mres
    }
    fn best_action(&self, pairs: &Vec<((usize, usize), usize)>, space_limit: usize, recursive: bool) -> (u128, usize, usize) {
        let mut mres = (u128::MAX, 0, 0);
        for i in 0..2 {
            let it_p = pairs[i];
            let alt_p = pairs[1-i];
            let itl = it_p.0.1 - it_p.0.0;
            let altl = alt_p.0.1 - alt_p.0.0;
            let it_tcost = self.enum_cost(itl, it_p.1);
            let alt_tcost = self.enum_cost(altl, alt_p.1);
            let alt_scost = self.space_cost(altl, alt_p.1);
            if alt_scost <= space_limit as u128 {
                mres = mres.min((it_tcost+alt_tcost, i, 0));
            }
            if recursive {
                mres = mres.min((3*it_tcost*self.estimated_cost(alt_p.0.0, alt_p.0.1, alt_p.1), i, 1));
            }
            //mres = mres.min((it_tcost*alt_tcost, i, 2));
        }
        mres
    }
}

struct ExternalStorage {
    comb_set: FxHashMap<u128, BitVec>,
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

fn get_xor(nums: &[u128], b: &BitVec) -> u128{
    let mut d = 0;
    for i in 0..nums.len() {
        if b.get(i).unwrap() {
            d ^= nums[i];
        }
    }
    d
}

impl<'a> Solver<'a> {
    fn explore(&self, context: &mut ExternalStorage, lo: usize, hi: usize, k: usize, target: u128, recursive: bool, depth: i64) -> Option<Combined> {
        let nums = &self.nums;
        let space_limit = context.comb_set.capacity();
        let mut res: Option<Combined> = None;
        let (pl, pr, is_dif) = get_partition(nums, lo, hi);
        let sl = pl.1-pl.0;let sr = pr.1-pr.0;
        /*if hi-lo == 1 {
            if nums[lo] == target {
                let mut c = Combined::new();
                c.0 = nums[lo];
                c.1.set(lo, true);
                return Some(c);
            }
            else {
                return None;
            }
        }*/
        let minl = if k >= sr {k-sr} else {0};
        let maxl = sl.min(k)+1;
        let k_range = if is_dif>0 {
            ((minl+(((is_dif & target)>0)as usize+k)%2)..maxl).step_by(2)
        }else {
            (minl..maxl).step_by(1)
        };
        for l in k_range {
            let r = k-l;
            let mut pairs = vec![(pl, l), (pr, r)];
            let action = self.best_action(&pairs, space_limit, recursive);
            if action.1 > 0 {
                pairs.swap(0, 1);
            }
            let (it_p, it_k) = pairs[0];
            let (alt_p, alt_k) = pairs[1];
            match action.2 {
                0 => {
                    context.comb_set.clear();
                    enum_combs(nums, alt_k, &mut |x| {context.comb_set.insert(x.0, x.1);}, alt_p.0, alt_p.1, Combined::new());
                    let mut it_func = |x: Combined| {
                        let compl = x.0 ^ target;
                        match context.comb_set.get(&compl) {
                            Some(c) => {res = Some(x.combine(&Combined(compl, c.clone())))},
                            None => ()
                        }
                    };
                    enum_combs(nums, it_k, &mut it_func, it_p.0, it_p.1, Combined::new());
                }
                1 => {
                    let mut it_func = |x: Combined| {
                        let compl = x.0 ^ target;
                        match self.explore(context, alt_p.0, alt_p.1, alt_k, compl, recursive, depth+1) {
                            Some(c) => res = Some(x.combine(&c)),
                            None => ()
                        }
                    };
                    enum_combs(nums, it_k, &mut it_func, it_p.0, it_p.1, Combined::new());
                }
                _ => {
                    unimplemented!()
                }
            }
            if let Some(c) = res {
                //if depth < 3 {println!("{}", depth);}
                return Some(c);
            } 
        }
        //if depth < 3 {println!("{}", depth);}
        return None;
    }
    
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::Rng;

    #[test]
    fn basic() {
        let mut solver = Solver::new();
        solver.nums = vec![0;110];
        assert_eq!(solver.binom(8, 4), 70);
        assert_eq!(solver.binom(76, 13), 152724276564800);
        assert_eq!(solver.enum_cost(55, 12), 595443690122);
        assert_eq!(solver.enum_cost(35, 4), 59536);
        solver.estimate_cost(0, 110, 11, 1e8 as usize, true);
        assert_eq!(solver.estimated_cost(0, 110, 11), 6161574467);
        //assert_eq!(solver.estimated_cost(0, 68, 16), 11565982879);
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
    #[test]
    fn test_msb() {
        assert_eq!(msb(3), 2);
        assert_eq!(msb(5), 4);
    }
}