use std::env;

use BonusAufgabe_proto::io::*;
use BonusAufgabe_proto::structs::*;
use BonusAufgabe_proto::processing::*;

fn main() {
    let filepath = env::args().nth(1).unwrap();
    //let b = "eingaben/BonusAufgabe/stapel1.txt";
    let input = TInput::read_from(filepath.as_str()).unwrap();
    let output = process(&input, &Constraints {max_jobs: 1, s_limit: 1e9 as usize, recursive: false});
    println!("{}", output.as_ref().unwrap());
    println!("Running slow_function() took {} milliseconds.", output.unwrap().runtime);
}