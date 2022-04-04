
/*
#[cfg(test)]
mod tests {
    use crate::structs::{CombStore, u256};

    use super::*;
    use test::Bencher;
    use super::super::structs::HashMapStore;
    use rand::{thread_rng, Rng};

    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2));
    }

    #[bench]
    fn bench_insert(b: &mut Bencher) {
        let mut store = HashMapStore::new(1e6 as usize);
        b.iter(|| {
            for i in 0..1e6 as usize {
                store.insert(thread_rng().gen_range(0..u128::MAX), u256::zero());
            }
        });
    }
    #[bench]
    fn bench_get(b: &mut Bencher) {
        let mut store = HashMapStore::new(1e6 as usize);
        b.iter(|| {
            for i in 0..1e6 as usize {
                store.insert(thread_rng().gen_range(0..u128::MAX), u256::zero());
            }
            for
        });
    }
}*/