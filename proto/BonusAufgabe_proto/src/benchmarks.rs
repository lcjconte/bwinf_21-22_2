extern crate test;

#[cfg(test)]
mod tests {
    use crate::structs::{CombStore, u256};
    use crate::testing::generate_solvable;

    use super::*;
    use test::Bencher;
    use super::super::structs::HashMapStore;
    use rand::{thread_rng, Rng};
    use crate::processing::process as p1;
    use crate::processing::Constraints as C1;
    use crate::testing;

    #[bench]
    fn bench_p1(b: &mut Bencher) {
        b.iter(|| {
            let input = generate_solvable(30, 5);
            let res = p1(&input, &C1::new(10*1e7 as usize, 4));
            assert!(res.unwrap().verify());
        });
    }
}