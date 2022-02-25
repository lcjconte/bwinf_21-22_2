#![allow(dead_code)]
mod common;
mod v1;
mod exp1;
mod exp2;
mod exp3;
mod exp4;
#[macro_use]
extern crate lazy_static;
use std::time::Instant;
use std::env;

use common::*;
use exp4::Solver;

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