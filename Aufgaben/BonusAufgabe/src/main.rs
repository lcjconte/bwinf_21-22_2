use std::env;

use Bonusaufgabe::io::*;
use Bonusaufgabe::processing::*;

fn main() {
    let filepath = env::args().nth(1).unwrap();
    let jcount: usize = env::args().nth(2).unwrap().parse().unwrap();
    let mmultiplier: usize = env::args().nth(3).unwrap().parse().unwrap();
    let input = TInput::read_from(filepath.as_str()).unwrap();
    let output = process(&input, &Constraints::new(mmultiplier*1e7 as usize, jcount));
    println!("{}", output.as_ref().expect("Solution not found!"));
    assert!(output.unwrap().verify());
}