extern crate test;

#[cfg(test)]
mod tests {
    use crate::structs::{CombStore, u256};

    use super::*;
    use test::Bencher;
    use super::super::structs::HashMapStore;
    use rand::{thread_rng, Rng};

    #[bench]
    fn bench_insert(b: &mut Bencher) {
        println!("HI");
        b.iter(|| {
            let mut store = HashMapStore::new(1e6 as usize);
            for i in 0..1e6 as usize {
                store.insert(thread_rng().gen_range(0..u128::MAX), u256::zero());
            }
        });
    }
    #[bench]
    fn bench_get(b: &mut Bencher) {
        let mut store = HashMapStore::new(1e6 as usize);
        for i in 0..1e6 as usize {
            store.insert(thread_rng().gen_range(0..u128::MAX), u256::zero());
        }
        println!("Inserted");
        b.iter(|| {
            for i in 0..1e6 as usize {
                store.get(thread_rng().gen_range(0..u128::MAX));
            }
        });
    }
}