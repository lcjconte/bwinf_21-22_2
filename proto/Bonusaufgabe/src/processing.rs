use std::mem::swap;
use std::sync::Arc;
use std::sync::mpsc::{channel, Sender};
use std::thread::{self, JoinHandle};
use std::time::Instant;
use crate::math::BinomC;

use super::io::*;
use super::structs::*;
/// [lo;hi)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Segment(pub usize, pub usize);

/// Processing params and constraints
#[derive(Clone, Default)]
pub struct Constraints {
    pub s_limit: usize,
    pub max_jobs: usize
} 
impl Constraints {
    pub fn new(size_limit: usize, max_jobs: usize) -> Constraints {
        let obj = Constraints {s_limit: size_limit, max_jobs};
        assert!(obj.valid());
        obj
    }
    pub fn valid(&self) -> bool {
        self.s_limit > 0 && self.max_jobs > 0
    }
}

/// One pass over described search space
pub struct OnePass {
    /// Iterated half
    pub it: (Segment, usize),
    /// Memorized half
    pub ca: (Segment, usize),
}

/// Splits segment in half
#[inline]
pub fn split_segment_simple(segment: Segment) -> Vec<Segment>{
    let sl = ((segment.1-segment.0) as f64 / 2.0).ceil() as usize;
    vec![Segment(segment.0, segment.0+sl), Segment(segment.0+sl, segment.1)]
}
/// Assigns memoization to smaller half
#[inline]
pub fn assign_k_simple(blocks: Vec<Segment>, l: usize, r: usize) -> OnePass {
    let mut obj = OnePass { it: (blocks[0], l), ca: (blocks[1], r) };
    if r > l {
        swap(&mut obj.it, &mut obj.ca);
    }
    obj
}

pub fn combination_nums(nums: &[u128], c: &Combination) -> Vec<u128> {
    let mut v: Vec<u128> = vec![];
    for (i, num) in nums.iter().enumerate() {
        if c.1.get(i) {
            v.push(*num);
        }
    }
    v
}

pub fn process(input: &TInput, constraints: &Constraints) -> Option<TOutput> {
    let start_time = Instant::now();
    let solver = Solver::new(Arc::new(input.nums.clone()), constraints);
    let n = solver.nums.len();
    let k = input.k+1;
    let res = solver.search(Segment(0, n), k, 0);
    if let Some(c) = res {
        let v = combination_nums(&solver.nums, &c);
        assert_eq!(v.len(), k);
        let output = TOutput {input: input.clone(), nums: v, runtime: start_time.elapsed().as_millis()};
        assert!(output.verify());
        Some(output)
    }
    else {
        None
    }
}

impl TOutput {
    pub fn verify(&self) -> bool {
        let mut a = 0;
        for i in &self.nums {
            a ^= i;
        }
        a == 0 && self.nums.len() == self.input.k+1
    }
}

/// Calls func on all combinations of length k 
pub fn map_combs_simple(nums: &[u128], k: usize, func: &mut dyn FnMut(&Combination), block: Segment, shift: usize, cur: Combination) {
    assert!(block.1 <= nums.len());
    if k == 0 {
        func(&cur);
        return;
    }
    if block.0==block.1 {return;}
    for i in block.0..block.1 {
        let num_idx = (i+shift) % nums.len();
        map_combs_simple(nums, k-1, func, Segment(i+1, block.1), shift, cur.add(nums[num_idx], num_idx));
    }
}
/// Optimized version of map_combs_simple
pub fn map_combs_adv(nums: &[u128], k: usize, func: &mut dyn FnMut(&Combination), block: Segment, shift: usize) {
    let mut prefix = vec![Combination::default(); nums.len()];
    prefix[0] = Combination::default().add(nums[0], 0);
    for i in 1..nums.len() {
        prefix[i] = prefix[i-1].add(nums[i], i);
    }
    map_combs_inner(nums, k, func, block, shift, Combination::default(), &prefix);
}
fn map_combs_inner(nums: &[u128], mut k: usize, func: &mut dyn FnMut(&Combination), block: Segment, shift: usize, mut cur: Combination, prefix: &Vec<Combination>) {
    assert!(block.1 <= nums.len());
    if k == 0 {
        return func(&cur);
    }
    let n = block.1-block.0;
    if block.0==block.1 || k > n {return;}
    if k > n/2 {
        k = n-k;
        let lo = (block.0+shift) % nums.len();
        let hi = (block.1+shift) % nums.len();
        // Toggle block space
        if hi > lo {
            cur.toggle_inplace(&prefix[hi-1]);
            if lo > 0 {
                cur.toggle_inplace(&prefix[lo-1]);
            }
        }
        else {
            cur.toggle_inplace(&prefix[nums.len()-1]);
            cur.toggle_inplace(&prefix[lo-1]);
            if hi > 0 {
                cur.toggle_inplace(&prefix[hi-1]);
            }
        }
    }
    if k == 0 { //Equivalent to k == n before toggling
        return func(&cur);
    }
    for i in block.0..block.1 {
        let num_idx = (i+shift) % nums.len();
        map_combs_inner(nums, k-1, func, Segment(i+1, block.1), shift, cur.apply(nums[num_idx], num_idx), prefix);
    }
}

pub struct Solver {
    pub nums: Arc<Vec<u128>>,
    pub binomc: BinomC,
    pub cons: Constraints,
}
impl Solver {
    fn new(nums: Arc<Vec<u128>>, cons: &Constraints) -> Self {
        Solver {
            nums,
            binomc: BinomC::default(),
            cons: cons.clone()
        }
    }
}

/// Search on limited segement if nums with equal distribution of k and specific shift
pub fn search_single_shift<T: CombStore>(nums: &[u128], segment: Segment, k: usize, shift: usize, target: u128, store: &mut T) -> SearchRes {
    let mut res: SearchRes = None;
    let l = (k as f64/2.0).ceil() as usize;
    let r = k-l;
    let blocks = split_segment_simple(segment);
    let pass = assign_k_simple(blocks, l, r);
    store.clear();
    map_combs_adv(nums, pass.ca.1, &mut |x| {store.insert(x.0, x.1);}, pass.ca.0, shift/*, Combination::default()*/);
    let mut it_func = |x: &Combination| {
        let compl = x.0 ^ target;
        match store.get(compl) {
            Some(c) => {res = Some(x.combine(&Combination(compl, c)));},
            None => ()
        }
    };
    map_combs_adv(nums, pass.it.1, &mut it_func, pass.it.0, shift/*, Combination::default()*/);
    res
}

pub fn search_shift_thread<T: CombStore>(sender: Sender<(SearchRes, T)>, nums: Arc<Vec<u128>>, mut store: T, segment: Segment, k: usize, shift: usize, target: u128) {
    let res = search_single_shift(&nums, segment, k, shift, target, &mut store);
    sender.send((res, store)).unwrap();
}

impl Solver {
    fn search(&self, segment: Segment, k: usize, target: u128) -> SearchRes {
        self.shift_search(segment, k, target)
    }
    fn shift_search(&self, segment: Segment, k: usize, target: u128) -> SearchRes {
        let Constraints { s_limit: cap, max_jobs: jcount } = self.cons;
        type Store = HashMapStore;
        let nums = self.nums.clone();
        let Segment(lo, hi) = segment;
        let n = hi-lo;
        let mut handles: Vec<JoinHandle<()>> = vec![];
        let (sender, receiver) = channel();
        let recap = self.binomc.binom(n/2, k/2) as usize;
        let rjcount = jcount.min(cap/recap);
        let mut storage: Vec<Store> = vec![Store::new(recap) ;rjcount];
        let mut res: SearchRes = None;
        for s_point in 0..(((n as f64/2.0).floor()+1.0) as usize) {
            let st: Store;
            if storage.is_empty() {
                let mres: (SearchRes, Store) = receiver.recv().unwrap();
                st = mres.1;
                if let Some(c) = mres.0 {
                    res = Some(c);
                    break;
                }
            }
            else {
                st = storage.pop().unwrap();
            }
            let aanums = nums.clone();let msender = sender.clone();
            let nthread = thread::spawn(move || {
                search_shift_thread(msender, aanums, st, Segment(lo, hi), k, s_point, target)
            });
            handles.push(nthread);
        }
        drop(sender);
        while let Ok(mres) = receiver.recv() {
            if let Some(c) = mres.0 {
                res = Some(c);
            }
        }
        while !handles.is_empty() {
            let h = handles.pop().unwrap();
            h.join().unwrap();
        }
        res
    }
}
