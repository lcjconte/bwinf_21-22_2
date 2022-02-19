#![allow(dead_code)]
mod common;
mod v1;
mod exp1;
mod exp2;
mod exp3;
//use primitive_types::U256;
use std::time::Instant;
use std::env;

use common::*;
use exp3::Solver;

trait ISolver<'a> {
    fn new() -> Self;
    fn process(&mut self, t_input: &'a TInput) -> Option<TOutput>;
}

fn main() {
    //let filepath = env::args().nth(1).unwrap();
    let b = "eingaben/BonusAufgabe/stapel3.txt";
    let input = TInput::read_from(b).unwrap();
    let mut solver = Solver::new();
    let now = Instant::now();
    solver.process(&input);
    let elapsed_time = now.elapsed();
    println!("Running slow_function() took {} milliseconds.", elapsed_time.as_millis());
}