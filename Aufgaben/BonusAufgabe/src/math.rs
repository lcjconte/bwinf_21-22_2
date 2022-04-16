use crate::io::{MAXN, MAXK};
use crate::structs::DParray;

/// Contains cost estimation functions
#[derive(Clone)]
pub struct BinomC {
    pascal: DParray<u128>,
}
impl Default for BinomC {
    fn default() -> Self {
        let mut unit = BinomC {
            pascal: DParray::new(0, MAXN, MAXK, 1),
        };
        unit.init();
        unit
    }
}

impl BinomC {
    fn init(&mut self) {
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
    }
    pub fn binom(&self, n: usize, mut k: usize) -> u128 {
        if k > n/2 {k = n-k;}
        assert!(n <= MAXN && k <= MAXK);
        self.pascal.get2(n, k)
    }
}