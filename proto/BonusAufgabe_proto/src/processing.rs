use std::mem::swap;
use std::sync::Arc;
use std::sync::mpsc::{channel, Sender};
use std::thread::{self, JoinHandle};
use std::cell::RefCell;
use std::time::Instant;

use super::io::*;
use super::structs::*;
/// [lo;hi)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Segment(usize, usize);
/// Tuning via python? 
struct Tuning {
    store_insertion: f64, //Cost of insertion into Combstore
    store_find: f64,      //Cost of find operation
    recursive_call: f64,  //Call overhead
}
// TODO: 
//Measure tuning costs

/// Processing params and constraints
#[derive(Clone, Default)]
pub struct Constraints {
    pub s_limit: usize,
    pub recursive: bool,
    pub max_jobs: usize
} 
impl Constraints {
    pub fn new(size_limit: usize, recursive: bool, max_jobs: usize) -> Constraints {
        let obj = Constraints {s_limit: size_limit, recursive, max_jobs};
        assert!(obj.valid());
        obj
    }
    pub fn valid(&self) -> bool {
        self.s_limit > 0 && self.max_jobs > 0
    }
}
/// Contains cost estimation functions
#[derive(Clone)]
pub struct CalcUnit {
    nums: Vec<u128>, //In case efficient partition is introduced
    pascal: DParray<u128>,
    binom_sum: DParray<u128>,
    cost_dp: RefCell<DParray<(u128, bool)>>,
    cost_dp_params: Constraints, 
}
impl Default for CalcUnit {
    fn default() -> Self {
        let mut unit = CalcUnit {
            nums: vec![],
            pascal: DParray::new(0, MAXN, MAXK, 1),
            binom_sum: DParray::new(0, MAXN, MAXK, 1),
            cost_dp: RefCell::new(DParray::new((0, false), MAXN, MAXK, 1)),
            cost_dp_params: Constraints::default(),
        };
        unit.init();
        unit
    }
}
impl CalcUnit {
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
        //Calculate binom_sum
        for n in 0..MAXN+1 {
            for k in 0..MAXK+1 {
                let mut res = 0;
                for i in 0..k+1 {
                    res += self.binom(n, i);
                }
                *self.binom_sum.get2_mut(n, k) = res;
            }
        }
    }
    pub fn binom(&self, n: usize, k: usize) -> u128 {
        assert!(n <= MAXN && k <= MAXK);
        self.pascal.get2(n, k)
    }
    pub fn binom_sum(&self, n: usize, k: usize) -> u128 {
        assert!(n <= MAXN && k <= MAXK);
        self.binom_sum.get2(n, k.min(n-k))
    }
    /// bool is false if not enough space is available
    pub fn shift_search_cost(&self, n: usize, k: usize) -> (u128, bool) {
        assert!(self.cost_dp_params.valid(), "Unit not properly initialized");
        let Constraints { s_limit: space_limit, recursive: _, max_jobs } = self.cost_dp_params;
        let mut usable_jobs = space_limit / self.binom(n/2, k/2) as usize;
        usable_jobs = usable_jobs.min(max_jobs);
        if usable_jobs > 0 {
            ((n as u128*self.binom_sum(n/2, k/2) * (2 + 1))/usable_jobs as u128, true)
        } else {
            (u128::MAX, false)
        }
    }
    pub fn lr_search_cost(&self, n: usize, k: usize) -> (u128, bool) {
        let mut res = 1;
        let sl = (n as f64/2.0).ceil() as usize;
        let sr = n-sl;
        for l in (if k >= sr {k-sr} else {0})..sl.min(k)+1 {
            let r = k-l;
            res += self.lr_single_cost(n, l, r).0;
        }
        (res, false)
    }
    /// Cost of single lr search iteration for given l and r
    pub fn lr_single_cost(&self, n: usize, l: usize, r: usize) -> (u128, usize) {
        assert!(self.cost_dp_params.valid(), "Unit not properly initialized");
        let Constraints { s_limit: space_limit, recursive, max_jobs: _ } = self.cost_dp_params;
        let blocks = split_segment_simple(Segment(0,  n));
        let pass = assign_k_simple(blocks, l, r);
        let it_n = pass.it.0.1-pass.it.0.0;
        let ca_n = pass.ca.0.1-pass.ca.0.0;
        let it_cost = self.binom_sum(it_n, pass.it.1); //*find
        let ca_cost = self.binom_sum(ca_n, pass.ca.1); //*insert
        let space_cost = self.binom(ca_n, pass.ca.1);
        let mut mres = (u128::MAX, 0);
        if space_cost <= space_limit as u128 {
            mres = mres.min((it_cost+ca_cost, 0));
        }
        if recursive {
            mres = mres.min((/*Call cost */ it_cost * self.expected_cost(ca_n, pass.ca.1).0, 1));
        }
        //mres = mres.min((it_tcost*alt_tcost, i, 2));
        mres
    }
    /// Expected cost to find target in space of dimensions n and k
    pub fn expected_cost(&self, n: usize, k: usize) -> (u128, bool) {
        if n==1 {
            return (1, false);
        }
        if self.cost_dp.borrow().get2(n, k).0 != 0 {
            return self.cost_dp.borrow().get2(n, k);
        }
        let mut cres = self.lr_search_cost(n, k);
        cres = cres.min(self.shift_search_cost(n, k));
        *self.cost_dp.borrow_mut().get2_mut(n, k) = cres;
        cres
    }
}
/// One pass over described search space
struct OnePass {
    /// Iterated half
    it: (Segment, usize),
    /// Memorized half
    ca: (Segment, usize),
}

/// Splits segment in half (usable in both searches)
fn split_segment_simple(segment: Segment) -> Vec<Segment>{
    let sl = ((segment.1-segment.0) as f64 / 2.0).ceil() as usize;
    vec![Segment(segment.0, segment.0+sl), Segment(segment.0+sl, segment.1)]
}
/// Splits segemnt along 0/1 msb (usable only in lr)
fn split_segment_smart(segment: Segment) -> Vec<Segment> {
    todo!()
}
// Missing
fn assign_k_simple(blocks: Vec<Segment>, l: usize, r: usize) -> OnePass {
    let mut obj = OnePass { it: (blocks[0], l), ca: (blocks[1], r) };
    if r > l {
        swap(&mut obj.it, &mut obj.ca);
    }
    obj
}
/// Contains everything necessary for a search
pub struct Solver {
    pub nums: Arc<Vec<u128>>,
    pub calcu: CalcUnit,
}

pub fn process(input: &TInput, constraints: &Constraints) -> Option<TOutput> {
    println!("Started processing ...");
    let start_time = Instant::now();
    let mut solver = Solver::new();
    solver.nums = Arc::new(input.nums.clone());
    //self.nums.sort(); //Needed for smart split
    let n = solver.nums.len();
    let k = input.k+1;
    solver.calcu = CalcUnit::default();
    solver.calcu.cost_dp_params = constraints.clone();
    let res = solver.search(Segment(0, n), k, 0);
    if let Some(c) = res {
        println!("Found!");
        let mut v: Vec<u128> = vec![];
        for i in 0..n {
            if c.1.get(i) {
                v.push(solver.nums[i]);
            }
        }
        let output = TOutput {input: input.clone(), nums: v, runtime: start_time.elapsed().as_millis()};
        assert!(output.verify());
        Some(output)
    }
    else {
        None
    }
}

impl Solver {
    pub fn new() -> Solver {
        Solver {
            nums: Arc::new(vec![]),
            calcu: CalcUnit::default(),
        }
    }
}

fn call_combs(nums: &[u128], k: usize, func: &mut dyn FnMut(Combination), block: Segment, shift: usize, window: Segment, cur: Combination) {
    let w = block.1-block.0;
}

/// Calls func on all combinations of length k in the window
/// The combination space can be shifted FIXME: URGENT!!!
fn enum_combs(nums: &[u128], k: usize, func: &mut dyn FnMut(Combination), block: Segment, shift: usize, window: Segment, cur: Combination) {
    assert!(block.1 <= nums.len());
    if k == 0 {
        func(cur);
        return;
    }
    if block.0==block.1 {return;}
    for i in block.0..block.1 {
        let num_idx = if i+shift < window.1 {
            i+shift
        }else {
            window.0+((i+shift)%window.1)
        };
        enum_combs(nums, k-1, func, Segment(i+1, block.1), shift, window, cur.add(nums[num_idx], num_idx));
    }
}

/// Search on limited segment of nums with specific distribution of k
fn search_single_lr<T: CombStore>(nums: &[u128], segment: Segment, l: usize, r: usize, target: u128, store: &mut T) -> SearchRes {
    let mut res: SearchRes = None;
    let blocks = split_segment_simple(segment);
    let pass = assign_k_simple(blocks, l, r);
    store.clear();
    enum_combs(nums, pass.ca.1, &mut |x| {store.insert(x.0, x.1);}, pass.ca.0, 0, segment, Combination::default());
    let mut it_func = |x: Combination| {
        let compl = x.0 ^ target;
        match store.get(compl) {
            Some(c) => {res = Some(x.combine(&Combination(compl, c)));},
            None => ()
        }
    };
    enum_combs(nums, pass.it.1, &mut it_func, pass.it.0, 0, segment, Combination::default());
    res
}
/// Search on limited segement if nums with equal distribution of k and specific shift
fn search_single_shift<T: CombStore>(nums: &[u128], segment: Segment, k: usize, shift: usize, target: u128, store: &mut T) -> SearchRes {
    let mut res: SearchRes = None;
    let l = (k as f64/2.0).ceil() as usize;
    let r = k-l;
    let blocks = split_segment_simple(segment);
    let pass = assign_k_simple(blocks, l, r);
    store.clear();
    enum_combs(nums, pass.ca.1, &mut |x| {store.insert(x.0, x.1);}, pass.ca.0, shift, segment, Combination::default());
    let mut it_func = |x: Combination| {
        let compl = x.0 ^ target;
        match store.get(compl) {
            Some(c) => {res = Some(x.combine(&Combination(compl, c)));},
            None => ()
        }
    };
    enum_combs(nums, pass.it.1, &mut it_func, pass.it.0, shift, segment, Combination::default());
    res
}

fn search_shift_thread<T: CombStore>(sender: Sender<(SearchRes, T)>, nums: Arc<Vec<u128>>, mut store: T, segment: Segment, k: usize, shift: usize, target: u128) {
    let res = search_single_shift(&nums, segment, k, shift, target, &mut store);
    sender.send((res, store)).unwrap();
}

impl Solver {
    fn search(&self, segment: Segment, k: usize, target: u128) -> SearchRes {
        let action = self.calcu.expected_cost(segment.1-segment.0, k);
        if action.1 {
            self.shift_search(segment, k, target)
        }
        else {
            self.lr_search(segment, k, target)
        }
    }
    fn lr_search(&self, segment: Segment, k: usize, target: u128) -> SearchRes {
        type Store = HashMapStore;
        let Segment(lo, hi) = segment;
        let n = hi-lo;
        let sl = (n as f64/2.0).ceil() as usize;
        let sr = (n as f64/2.0).floor() as usize;
        let blocks = split_segment_simple(segment);  //Wouldn't work with smart partition
        let mut res: SearchRes = None;
        for l in (if k >= sr {k-sr} else {0})..sl.min(k)+1 { 
            let r = k-l;
            let pass = assign_k_simple(blocks.clone(), l, r);
            let action = self.calcu.lr_single_cost(n, l, r);
            match action.1 {
                0 => {
                    if let Some(c) = search_single_lr(&self.nums, segment, l, r, target, &mut Store::new(self.calcu.binom(n/2, pass.ca.1) as usize)) { 
                        return Some(c);
                    }
                }
                1 => {
                    let mut it_func = |x: Combination| {
                        let compl = x.0 ^ target;
                        match self.search(pass.ca.0, pass.ca.1, compl) {
                            Some(c) => res = Some(x.combine(&c)),
                            None => ()
                        }
                    };
                    enum_combs(&self.nums, pass.it.1, &mut it_func, pass.it.0, 0, segment, Combination::default());
                }
                _ => {
                    unimplemented!()
                }
            }
            
        }
        None
    }
    fn shift_search(&self, segment: Segment, k: usize, target: u128) -> SearchRes {
        let Constraints { s_limit: cap, recursive: _, max_jobs: jcount } = self.calcu.cost_dp_params;
        type Store = HashMapStore;
        let nums = self.nums.clone();
        let Segment(lo, hi) = segment;
        let n = hi-lo;
        let mut handles: Vec<JoinHandle<()>> = vec![];
        let (sender, receiver) = channel();
        let recap = self.calcu.binom(n/2, k/2) as usize;
        let rjcount = jcount.min(cap/recap);
        let mut storage: Vec<Store> = vec![Store::new(recap) ;rjcount];
        let mut res: SearchRes = None;
        for s_point in 0..(((n as f64/2.0).floor()+1.0) as usize) {
            println!("{}", s_point);
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
        while !handles.is_empty() {
            let h = handles.pop().unwrap();
            h.join().unwrap();
        }
        res
    }
    
}
