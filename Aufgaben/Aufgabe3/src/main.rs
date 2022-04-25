use Aufgabe3::processing::*;
use Aufgabe3::io::*;
use Aufgabe3::testing::generate_testcase;
use Aufgabe3::testing::run_randomized;
use Aufgabe3::testing::run_samples;
use std::env;
use std::fs;

fn main() {
    let file_name = env::args().nth(1).unwrap();
    let chars = Characters::read_from("eingaben/chars.json").unwrap();
    let input = TInput::read_from(file_name).unwrap();
    let output = process(&input, &chars, false);
    println!("{}", output);
    //run_samples();
    //run_randomized(100, 1000, false);
}