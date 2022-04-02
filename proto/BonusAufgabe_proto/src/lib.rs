#![feature(test)]

pub mod io;
pub mod processing;
pub mod structs;
pub mod benchmarks;

#[cfg(test)]
mod tests {
    use super::io::TOutput;
    #[test]
    fn it_works() {
        let a = TOutput {nums: vec![], runtime: 1};
        a.verify();
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
