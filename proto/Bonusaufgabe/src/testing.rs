use super::io::*;
use super::processing::*;
use rand::thread_rng;
use rand::Rng;
use rand::seq::SliceRandom;
use std::fs;

/// Generate random solvable input 
pub fn generate_solvable(n: usize, k: usize) -> TInput {
    let mut nums = vec![];
    let mut xo = 0;
    for i in 0..k {
        nums.push(thread_rng().gen_range(0..(1_u128 << 127)));
        xo ^= nums.last().unwrap();
    }
    nums.push(xo);
    while nums.len() < n {
        nums.push(thread_rng().gen_range(0..(1_u128 << 127)));
    }
    nums.shuffle(&mut thread_rng());
    TInput {n, k, m: 128, nums }
}

/// Generates random very likely unsolvable input
pub fn generate_random(n: usize, k: usize) -> TInput {
    let mut nums = vec![0; n];
    for el in nums.iter_mut() {
        *el = thread_rng().gen_range(0..(1_u128 << 127));
    }
    TInput {n, k, m: 128, nums }
}

pub fn run_quick_samples() {
    for i in (0..3).chain(5..6) {
        let input = TInput::read_from(&format!("eingaben/stapel{}.txt", i)).unwrap();
        let output = process(&input, &Constraints::new(10 * 1e7 as usize, 4)).unwrap();
        assert!(output.verify());
        //fs::write(format!("ausgaben/ausgabe{}.txt", i), format!("{}", output)).unwrap();
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_random_tests() {
        let r = 100;let maxn = 50;
        for _ in 0..r {
            let n = thread_rng().gen_range(2..maxn);
            let maxk = MAXK.min(n-1);
            let input = generate_random(n, thread_rng().gen_range(1..=maxk));
            let _res = process(&input, &Constraints::new(10 * 1e7 as usize, 4));
        }
    }
    #[test]
    pub fn run_solvable_tests() {
        let r = 100;let maxn = 50;
        for _ in 0..r {
            let n = thread_rng().gen_range(2..maxn);
            let maxk = MAXK.min(n-1);
            let input = generate_solvable(n, thread_rng().gen_range(1..=maxk));
            let res = process(&input, &Constraints::new(10 * 1e7 as usize, 4));
            assert!(res.unwrap().verify());
        }
    }
    #[test]
    pub fn test_quick_samples() {
        run_quick_samples();
    }
}