#![allow(dead_code)]
use std::collections::{HashMap, BTreeMap};
use std::sync::Arc;
use std::sync::mpsc::{channel, Sender};
use std::thread::{self, JoinHandle};
use std::time::SystemTime;
use ahash::{RandomState, AHasher};

use bit_vec::BitVec;

use super::common::*;

//[lo;hi)
//This version introduces different CombStores and multithreading
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
        let n = self.t_input.unwrap().n as usize;
        let k = (self.t_input.unwrap().k+1) as usize;
        let res = self.explore(3, 1e7 as usize, k, 0);
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
trait CombStore {
    fn new(size: usize) -> Self;
    fn insert(&mut self, k: u128, v: BitVec) -> ();
    fn get(&mut self, k: u128) -> Option<BitVec>;
    fn clear(&mut self) -> ();
}

#[derive(Clone)]
struct HashMapStore(HashMap<u128, BitVec, RandomState>);
impl CombStore for HashMapStore {
    fn new(size: usize) -> Self{
        let mut hmap = HashMap::<u128, BitVec, RandomState>::default();
        //hmap.reserve(size);
        HashMapStore(hmap)
    }
    fn insert(&mut self, k: u128, v: BitVec) {
        self.0.insert(k, v);
    }
    fn get(&mut self, k: u128) -> Option<BitVec> {
        self.0.get(&k).map(|x| {x.clone()})
    }
    fn clear(&mut self) {
        self.0.clear();
    }
}

#[derive(Clone)]
struct BTStore(BTreeMap<u128, BitVec>);
impl CombStore for BTStore {
    fn new(size: usize) -> Self{
        BTStore(BTreeMap::new())
    }
    fn insert(&mut self, k: u128, v: BitVec) {
        self.0.insert(k, v);
    }
    fn get(&mut self, k: u128) -> Option<BitVec> {
        self.0.get(&k).map(|x| {x.clone()})
    }
    fn clear(&mut self) {
        self.0.clear();
    }
}

#[derive(Clone)]
struct AltStore(Vec<(u128, BitVec)>, bool);
impl CombStore for AltStore {
    fn new(size: usize) -> Self {
        AltStore(Vec::with_capacity(size), false)
    }
    fn insert(&mut self, k: u128, v: BitVec) {
        self.1 = false;
        self.0.push((k, v));
    }
    fn get(&mut self, k: u128) -> Option<BitVec> {
        if !self.1 {self.0.sort();self.1 = true;}
        if self.0[self.0.len()-1].0 < k {return None;}
        let ppoint = self.0.partition_point(|x| {x.0 < k});
        let val = &self.0[ppoint];
        if val.0 != k {
            None
        }else {
            Some(val.1.clone())
        }
    }
    fn clear(&mut self) {
        self.1 = false;
        self.0.clear()
    }
}

#[derive(Debug, Clone)]
struct Combination(u128, BitVec);
impl Combination {
    fn new() -> Self{
        Combination(0, BitVec::from_elem(MAXN+1, false))
    }
    fn add(&self, b: u128, idx: usize) -> Combination {
        let mut c = Combination(self.0 ^ b, self.1.clone());
        c.1.set(idx, true);
        c
    }
    fn combine(&self, b: &Combination) -> Combination {
        let mut c = Combination(self.0 ^ b.0, self.1.clone());
        c.1.or(&b.1);
        c
    }
}

fn enum_combs(nums: &[u128], k: usize, func: &mut dyn FnMut(Combination) -> (), block: (usize, usize), shift: usize, cur: Combination) {
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

type SearchRes = Option<Combination>;
fn search_here<T: CombStore>(t_idx: usize, sender: Sender<(SearchRes, T)>, nums: Arc<Vec<u128>>, mut context: T, k: usize, shift: usize, target: u128) -> Option<Combination> {
    //let nums = &self.nums;
    let n = nums.len();
    let mut res: Option<Combination> = None;
    let sl = (n as f64/2.0).ceil() as usize;
    let l = (k as f64/2.0).ceil() as usize;
    let r = k-l;
    let blocks = vec![(0, sl), (sl, n)];
    let mut pairs = vec![(l, blocks[0]), (r, blocks[1])];
    pairs.sort();
    let (alt_k, alt_p) = pairs[0];
    let (it_k, it_p) = pairs[1];
    context.clear();
    enum_combs(&nums, alt_k, &mut |x| {context.insert(x.0, x.1);}, alt_p, shift, Combination::new());
    let mut it_func = |x: Combination| {
        let compl = x.0 ^ target;
        match context.get(compl) {
            Some(c) => {res = Some(x.combine(&Combination(compl, c.clone())));},
            None => ()
        }
    };
    enum_combs(&nums, it_k, &mut it_func, it_p, shift, Combination::new());
    sender.send((res.clone(), context)).unwrap();
    res
}

impl<'a> Solver<'a> {
    fn explore(&self, jobs: usize, size_limit: usize, k: usize, target: u128) -> Option<Combination> {
        let nums = &self.nums;
        let n = nums.len();
        let mut handles: Vec<JoinHandle<Option<Combination>>> = vec![];
        let (sender, receiver) = channel();
        let mut storage: Vec<HashMapStore> = vec![HashMapStore::new(size_limit);jobs];
        let anums = Arc::new(nums.clone());
        let mut res: SearchRes = None;
        for s_point in 0..n {
            println!("{}", &s_point);
            let st: HashMapStore;
            if storage.is_empty() {
                let t_idx: usize;
                let mres: (Option<Combination>, HashMapStore) = receiver.recv().unwrap();
                st = mres.1;
                if let Some(c) = mres.0 {
                    res = Some(c.clone());
                    break;
                }
            }
            else {
                st = storage.pop().unwrap();
            }
            let aanums = anums.clone();let hlen = handles.len();let msender = sender.clone();
            let nthread = thread::spawn(move || {
                search_here(hlen, msender, aanums, st, k, s_point, target)
            });
            handles.push(nthread);
        }
        while !handles.is_empty() {
            let h = handles.pop().unwrap();
            h.join().unwrap();
        }
        return res;
    }
    
}
