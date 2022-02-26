use std::sync::Arc;
use std::sync::mpsc::{channel, Sender};
use std::thread::{self, JoinHandle};
use std::cell::RefCell;
//This version was able to solve 4 in 34 mins
use super::io::*;
use super::structs::*;
//[lo;hi)

struct Tuning {
    store_insertion: f64,
    store_find: f64,
}
/// Processing params and constraints
#[derive(Clone, Default)]
pub struct CalcParams {
    s_limit: usize,
    recursive: bool,
    max_jobs: usize
} 
impl CalcParams {
    pub fn new(size_limit: usize, recursive: bool, max_jobs: usize) -> CalcParams{
        let obj = CalcParams {s_limit: size_limit, recursive, max_jobs};
        assert!(obj.valid());
        obj
    }
    pub fn valid(&self) -> bool {
        self.s_limit > 0 && self.max_jobs > 0
    }
}
#[derive(Clone)]
pub struct CalcUnit {
    nums: Vec<u128>, //In case efficient partition is introduced
    pascal: DParray<u128>,
    binom_sum: DParray<u128>,
    cost_dp: RefCell<DParray<(u128, bool)>>,
    cost_dp_params: CalcParams, 
}
impl CalcUnit {
    pub fn new() -> Self {
        let mut unit = CalcUnit {
            nums: vec![],
            pascal: DParray::new(0, MAXN, MAXK, 1),
            binom_sum: DParray::new(0, MAXN, MAXK, 1),
            cost_dp: RefCell::new(DParray::new((0, false), MAXN, MAXK, 1)),
            cost_dp_params: CalcParams::default(),
        };
        unit.init();
        unit
    }
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
    }
    /// Binomial coefficient \ 
    /// n <= MAXN and k <= MAXK 
    pub fn binom(&self, n: usize, k: usize) -> u128 {
        assert!(n <= MAXN && k <= MAXK);
        self.pascal.get2(n, k)
    }
    pub fn space_cost(&self, n: usize, k: usize) -> u128 {
        self.binom(n, k)
    }
    pub fn enum_cost(&self, n: usize, k: usize) -> u128 {
        assert!(n <= MAXN && k <= MAXK);
        self.binom_sum.get2(n, k)
    }
    pub fn shifted_search_cost(&self, n: usize, k: usize) -> u128 {
        n as u128*self.enum_cost(n/2, k/2) //TODO: Improve accuracy and add tuning
    }
    pub fn expected_cost(&self, n: usize, k: usize) -> (u128, bool) {
        assert!(self.cost_dp_params.valid(), "Unit not properly initialized");
        let CalcParams { s_limit: space_limit, recursive: _, max_jobs } = self.cost_dp_params;
        if n==1 {
            return (1, false);
        }
        if self.cost_dp.borrow().get2(n, k).0 != 0 {
            return self.cost_dp.borrow().get2(n, k);
        }
        let mut res = 1;
        let sl = (n as f64/2.0).ceil() as usize;
        let sr = n-sl;
        for l in (if k >= sr {k-sr} else {0})..sl.min(k)+1 {
            let r = k-l;
            let pairs = vec![(sl, l), (sr, r)];
            res += self.best_action(&pairs).0;
        }
        let mut usable_jobs = space_limit / self.binom(n/2, k/2) as usize;
        usable_jobs = usable_jobs.min(max_jobs);
        let cres = if usable_jobs > 0 && res > self.shifted_search_cost(n, k)/usable_jobs as u128{
            (self.shifted_search_cost(n, k)/usable_jobs as u128, true)
        } else {
            (res, false)
        };
        *self.cost_dp.borrow_mut().get2_mut(n, k) = cres;
        cres
    }
    pub fn best_action(&self, pairs: &Vec<(usize, usize)>) -> (u128, usize, usize) {
        assert!(self.cost_dp_params.valid(), "Unit not properly initialized");
        let (alpha, beta, gamma) = (1.0, 1.0, 1.0); //TODO: Add tuning
        let CalcParams { s_limit: space_limit, recursive, max_jobs: _ } = self.cost_dp_params;
        let mut mres = (u128::MAX, 0, 0);
        for i in 0..2 {
            let it_p = pairs[i];
            let alt_p = pairs[1-i];
            let it_tcost = self.enum_cost(it_p.0, it_p.1);

            let alt_tcost = self.enum_cost(alt_p.0, alt_p.1);
            let alt_scost = self.space_cost(alt_p.0, alt_p.1);
            if alt_scost <= space_limit as u128 {
                mres = mres.min((it_tcost+alt_tcost, i, 0));
            }

            if recursive {
                mres = mres.min((/*3**/it_tcost*self.expected_cost(alt_p.0, alt_p.1).0, i, 1));
            }
            //mres = mres.min((it_tcost*alt_tcost, i, 2));
        }
        mres
    }
}

pub struct Solver<'a> {
    t_input: Option<&'a TInput>,
    nums: Arc<Vec<u128>>,
    calcu: CalcUnit,
}

impl<'a> ISolver<'a> for Solver<'a> {
    fn new() -> Solver<'a> {
        Solver { 
            t_input: None,
            nums: Arc::new(vec![]),
            calcu: CalcUnit::new(),
        }
    }
    fn process(&mut self, t_input: &'a TInput) -> Option<TOutput> {
        println!("Processing");
        self.t_input = Some(t_input);
        self.nums = Arc::new(self.t_input.unwrap().nums.clone());
        //self.nums.sort();
        let n = self.t_input.unwrap().n as usize;
        let k = (self.t_input.unwrap().k+1) as usize;
        self.calcu.cost_dp_params = CalcParams {s_limit: 1e7 as usize, recursive: true, max_jobs: 4};
        let res = self.entry_point(4, 1e7 as usize, k, 0);
        if let Some(c) = res {
            println!("Found!");
            println!("{}", c.0);
            let mut v: Vec<u128> = vec![];
            for i in 0..n {
                if c.1.get(i) {
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

fn enum_combs(nums: &[u128], k: usize, func: &mut dyn FnMut(Combination), block: (usize, usize), shift: usize, window: (usize, usize), cur: Combination) {
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
        enum_combs(nums, k-1, func, (i+1, block.1), shift, window, cur.add(nums[num_idx], num_idx));
    }
}

type SearchRes = Option<Combination>;

/// Search on limited segment of nums with specific distribution of k
fn search_divided<T: CombStore>(nums: &[u128], lo: usize, hi: usize, l: usize, r: usize, target: u128, store: &mut T) -> SearchRes {
    let n = hi-lo;
    let mut res: SearchRes = None;
    let sl = (n as f64/2.0).ceil() as usize;
    let blocks = vec![(lo, sl), (sl, hi)];
    let mut pairs = vec![(l, blocks[0]), (r, blocks[1])];
    pairs.sort(); //Assume insert more expensive than get
    let (alt_k, alt_p) = pairs[0];
    let (it_k, it_p) = pairs[1];
    store.clear();
    enum_combs(nums, alt_k, &mut |x| {store.insert(x.0, x.1);}, alt_p, 0, (lo, hi), Combination::default());
    let mut it_func = |x: Combination| {
        let compl = x.0 ^ target;
        match store.get(compl) {
            Some(c) => {res = Some(x.combine(&Combination(compl, c)));},
            None => ()
        }
    };
    enum_combs(nums, it_k, &mut it_func, it_p, 0, (lo, hi), Combination::default());
    res
}
/// Shifted search on all nums with equal distribution of k
fn search_shift<T: CombStore>(nums: &[u128], lo: usize, hi: usize, k: usize, shift: usize, target: u128, store: &mut T) -> SearchRes {
    let n = hi-lo;
    let mut res: SearchRes = None;
    let sl = (n as f64/2.0).ceil() as usize;
    let l = (k as f64/2.0).ceil() as usize;
    let r = k-l;
    let blocks = vec![(lo, lo+sl), (lo+sl, hi)];
    let mut pairs = vec![(l, blocks[0]), (r, blocks[1])];
    pairs.sort();
    let (alt_k, alt_p) = pairs[0];
    let (it_k, it_p) = pairs[1];
    store.clear();
    enum_combs(nums, alt_k, &mut |x| {store.insert(x.0, x.1);}, alt_p, shift, (lo, hi), Combination::default());
    let mut it_func = |x: Combination| {
        let compl = x.0 ^ target;
        match store.get(compl) {
            Some(c) => {res = Some(x.combine(&Combination(compl, c)));},
            None => ()
        }
    };
    enum_combs(nums, it_k, &mut it_func, it_p, shift, (lo, hi), Combination::default());
    res
}
//TODO: Shorten args
fn search_shift_thread<T: CombStore>(sender: Sender<(SearchRes, T)>, nums: Arc<Vec<u128>>, mut store: T, lo: usize, hi: usize, k: usize, shift: usize, target: u128) {
    let res = search_shift(&nums, lo, hi, k, shift, target, &mut store);
    sender.send((res.clone(), store)).unwrap();
}

impl<'a> Solver<'a> {
    fn entry_point(&mut self, jobs: usize, size_limit: usize, k: usize, target: u128) -> SearchRes {
        self.calcu.cost_dp_params = CalcParams::new(size_limit, true, jobs);
        println!("Estimated {}", self.calcu.expected_cost(self.nums.len(), k).0);
        self.explore(0, self.nums.len(), k, target)
        //self.explore_shifts(0, self.nums.len(), k, target)
        //self.explore(0, self.nums.len(), k, target)
        //self.explore_distribs(0, self.nums.len(), k, target)
    }
    fn explore(&self, lo: usize, hi: usize, k: usize, target: u128) -> SearchRes {
        let mut action = self.calcu.expected_cost(hi-lo, k);
        action.1 = true;
        if action.1 {
            self.explore_shifts(lo, hi, k, target)
        }
        else {
            self.explore_distribs(lo, hi, k, target)
        }
    }
    fn explore_distribs(&self, lo: usize, hi: usize, k: usize, target: u128) -> SearchRes {
        type Store = HashMapStore;
        let n = hi-lo;
        let CalcParams { s_limit: cap, recursive, max_jobs: jcount } = self.calcu.cost_dp_params;
        let sl = (n as f64/2.0).ceil() as usize;
        let sr = (n as f64/2.0).floor() as usize;
        let mut res: SearchRes = None;
        for l in (if k >= sr {k-sr} else {0})..sl.min(k)+1 { 
            println!("{}", l);
            let r = k-l;
            let blocks = vec![(0, sl), (sl, n)];
            let mut pairs = vec![(l, blocks[0]), (r, blocks[1])];
            pairs.sort();
            let (alt_k, alt_p) = pairs[1]; //Here different
            let (it_k, it_p) = pairs[0];
            let spairs = vec![(sl, l), (sr, r)];
            let action = self.calcu.best_action(&spairs);
            match action.2 {
                0 => {
                    if let Some(c) = search_divided(&self.nums, lo, hi, l, r, target, &mut Store::new(0)) { //Calc cap and fix paairs
                        return Some(c);
                    }
                }
                1 => {
                    let mut it_func = |x: Combination| {
                        let compl = x.0 ^ target;
                        match self.explore(alt_p.0, alt_p.1, alt_k, compl) {
                            Some(c) => res = Some(x.combine(&c)),
                            None => ()
                        }
                    };
                    enum_combs(&self.nums, it_k, &mut it_func, it_p, 0, (lo, hi), Combination::default());
                }
                _ => {
                    unimplemented!()
                }
            }
            
        }
        None
    }
    fn explore_shifts(&self, lo: usize, hi: usize, k: usize, target: u128) -> SearchRes {
        let CalcParams { s_limit: cap, recursive: _, max_jobs: jcount } = self.calcu.cost_dp_params;
        type Store = HashMapStore;
        let nums = self.nums.clone();
        let n = hi-lo;
        let mut handles: Vec<JoinHandle<()>> = vec![];
        let (sender, receiver) = channel();
        let recap = self.calcu.binom(n/2, k/2) as usize;
        let rjcount = jcount.min(cap/recap);
        let mut storage: Vec<Store> = vec![Store::new(recap) ;rjcount];
        let mut res: SearchRes = None;
        for s_point in 0..((n as f64/2.0).ceil() as usize) {
            //println!("{}", &s_point);
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
                search_shift_thread(msender, aanums, st, lo, hi, k, s_point, target)
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
