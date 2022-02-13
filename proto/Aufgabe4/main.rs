#![allow(dead_code)]
mod common;
mod v1;
mod exp1;
//use primitive_types::U256;
use std::time::Instant;
use std::env;

use common::*;
use exp1::Solver;

trait ISolver<'a> {
    fn new() -> Self;
    fn process(&mut self, t_input: &'a TInput) -> Option<TOutput>;
}

fn main() {
    //let filepath = env::args().nth(1).unwrap();
    let b = "eingaben/BonusAufgabe/stapel2.txt";
    let input = TInput::read_from(b).unwrap();
    let mut solver = Solver::new();
    /*let n = tInput.n as usize;
    let k = (tInput.k+1) as usize;
    solver.t_input = Some(tInput);
    let mut context = Context {comb_set: HashMap::with_capacity(1e8 as usize)};
    let res = solver.explore(&mut context, 0, n-1, k, 0, false);
    //enum_combs(&vec![17, 2, 3], 3, &mut |x| {println!("{}", x)}, 0, 0);*/
    let now = Instant::now();
    solver.process(&input);
    let elapsed_time = now.elapsed();
    println!("Running slow_function() took {} milliseconds.", elapsed_time.as_millis());
}