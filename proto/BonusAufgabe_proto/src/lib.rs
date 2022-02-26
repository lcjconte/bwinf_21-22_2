pub mod io;
pub mod processing;
pub mod structs;
use io::*;
//use processing::*;

#[cfg(test)]
mod tests {
    use super::io::TOutput;
    #[test]
    fn it_works() {
        let a = TOutput {nums: vec![]};
        a.verify();
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
