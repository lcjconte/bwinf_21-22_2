#![allow(dead_code)]
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
        //self.nums.sort();
        let mut context = ExternalStorage {comb_set: FxHashMap::default()};
        context.comb_set.reserve(1e7 as usize);
        let n = self.t_input.unwrap().n as usize;
        let k = (self.t_input.unwrap().k+1) as usize;
        let res = self.explore(&mut context, k, 0);
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

fn enum_combs(nums: &[u128], k: usize, func: &mut dyn FnMut(Combined) -> (), block: (usize, usize), shift: usize, cur: Combined) {
    assert!(block.1 <= nums.len());
    let n = nums.len();
    if k == 0 {
        func(cur);
        return;
    }
    if block.0==block.1 {return;}
    for i in block.0..block.1 {
        enum_combs(nums, k-1, func, (i+1, block.1), shift, cur.add(nums[(i+shift)%n], (i+shift)%n));
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

fn worker_thread() {

}

impl<'a> Solver<'a> {
    fn search_here(&self, context:  &mut ExternalStorage, k: usize, shift: usize, target: u128) -> Option<Combined> {
        let nums = &self.nums;
        let n = nums.len();
        let mut res: Option<Combined> = None;
        let sl = (n as f64/2.0).ceil() as usize;
        let l = (k as f64/2.0).ceil() as usize;
        let r = k-l;
        let blocks = vec![(0, sl), (sl, n)];
        let mut pairs = vec![(l, blocks[0]), (r, blocks[1])];
        pairs.sort();
        let (alt_k, alt_p) = pairs[0];
        let (it_k, it_p) = pairs[1];
        let space_limit = context.comb_set.capacity();
        context.comb_set.clear();
        enum_combs(nums, alt_k, &mut |x| {context.comb_set.insert(x.0, x.1);}, alt_p, shift, Combined::new());
        let mut it_func = |x: Combined| {
            let compl = x.0 ^ target;
            match context.comb_set.get(&compl) {
                Some(c) => {res = Some(x.combine(&Combined(compl, c.clone())));},
                None => ()
            }
        };
        enum_combs(nums, it_k, &mut it_func, it_p, shift, Combined::new());
        res
    }
    fn explore(&self, context: &mut ExternalStorage, k: usize, target: u128) -> Option<Combined> {
        let nums = &self.nums;
        let n = nums.len();
        for s_point in 0..n {
            println!("{}", &s_point);
            let mres = self.search_here(context, k, s_point, target);
            if let Some(c) = mres {
                return Some(c);
            } 
        }
        return None;
    }
    
}
