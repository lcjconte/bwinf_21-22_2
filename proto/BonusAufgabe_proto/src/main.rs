use std::time::Instant;
use std::env;

use BonusAufgabe_proto::io::*;
use BonusAufgabe_proto::structs::*;
use BonusAufgabe_proto::processing::Solver;

fn main() {
    let filepath = env::args().nth(1).unwrap();
    //let b = "eingaben/BonusAufgabe/stapel1.txt";
    let input = TInput::read_from(&filepath).unwrap();
    let mut solver = Solver::new();
    let now = Instant::now();
    solver.process(&input);
    let elapsed_time = now.elapsed();
    println!("Running slow_function() took {} milliseconds.", elapsed_time.as_millis());
}