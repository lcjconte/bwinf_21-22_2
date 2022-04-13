use crate::math::BinomC;
use crate::processing::*;
use crate::structs::*;
use crate::io::*;
use std::cell::RefCell;
use std::sync::Arc;

/// Contains cost estimation functions
#[derive(Clone)]
pub struct CalcUnit {
    nums: Vec<u128>, //In case efficient partition is introduced
    binomc: BinomC,
    cost_dp: RefCell<DParray<(u128, bool)>>,
    pub cost_dp_params: Constraints, 
}
impl Default for CalcUnit {
    fn default() -> Self {
        let mut unit = CalcUnit {
            nums: vec![],
            binomc: BinomC::default(),
            cost_dp: RefCell::new(DParray::new((0, false), MAXN, MAXK, 1)),
            cost_dp_params: Constraints::default(),
        };
        unit
    }
}
impl CalcUnit {
    pub fn binom(&self, n: usize, k: usize) -> u128 {
        self.binomc.binom(n, k)
    }
    pub fn binom_sum(&self, n: usize, k: usize) -> u128 {
        self.binomc.binom_sum(n, k)
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
        let Constraints { s_limit: space_limit, max_jobs: _ } = self.cost_dp_params;
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
        mres = mres.min((/*Call cost */ it_cost * self.expected_cost(ca_n, pass.ca.1).0, 1));
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
        *self.cost_dp.borrow_mut().get2_mut(n, k) = cres;
        cres
    }
}

/// Contains everything necessary for a search
pub struct Solverv2 {
    pub nums: Arc<Vec<u128>>,
    pub calcu: CalcUnit,
}

/// Search on limited segment of nums with specific distribution of k
fn search_single_lr<T: CombStore>(nums: &[u128], segment: Segment, l: usize, r: usize, target: u128, store: &mut T) -> SearchRes {
    let mut res: SearchRes = None;
    let blocks = split_segment_simple(segment);
    let pass = assign_k_simple(blocks, l, r);
    store.clear();
    enum_combs(nums, pass.ca.1, &mut |x| {store.insert(x.0, x.1.clone());}, pass.ca.0, 0, segment, Combination::default());
    let mut it_func = |x: &Combination| {
        let compl = x.0 ^ target;
        match store.get(compl) {
            Some(c) => {res = Some(x.combine(&Combination(compl, c)));},
            None => ()
        }
    };
    enum_combs(nums, pass.it.1, &mut it_func, pass.it.0, 0, segment, Combination::default());
    res
}

impl Solverv2 {
    pub fn new() -> Solverv2 {
        Solverv2 {
            nums: Arc::new(vec![]),
            calcu: CalcUnit::default(),
        }
    }
    fn search_include_lr(&self, segment: Segment, k: usize, target: u128) -> SearchRes {
        let action = self.calcu.expected_cost(segment.1-segment.0, k);
        self.lr_search(segment, k, target)
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
                    let mut it_func = |x: &Combination| {
                        let compl = x.0 ^ target;
                        match self.search_include_lr(pass.ca.0, pass.ca.1, compl) {
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
}