use std::env;

use BonusAufgabe_proto::io::*;
use BonusAufgabe_proto::structs::*;
use BonusAufgabe_proto::processing::Solver;

fn main() {
    let filepath = env::args().nth(1).unwrap();
    //let b = "eingaben/BonusAufgabe/stapel1.txt";
    let input = TInput::read_from(filepath.as_str()).unwrap();
    let mut solver = Solver::new();
    let output = solver.process(&input);
    println!("Running slow_function() took {} milliseconds.", output.unwrap().runtime);
}