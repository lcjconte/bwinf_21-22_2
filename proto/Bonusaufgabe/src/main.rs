use std::env;

use Bonusaufgabe::io::*;
use Bonusaufgabe::structs::*;
use Bonusaufgabe::processing::*;
use Bonusaufgabe::testing::*;

fn main() {
    //tests::run_solvable_tests();
    println!("Done!");
    let input = generate_solvable(100, 90);
    let res = process(&input, &Constraints::new(10 * 1e7 as usize, 4));
    assert!(res.unwrap().verify());
    run_quick_samples();
    /*let filepath = env::args().nth(1).unwrap();
    //let b = "eingaben/BonusAufgabe/stapel1.txt";
    let input = TInput::read_from(filepath.as_str()).unwrap();
    let output = process(&input, &Constraints::new(1e9 as usize, 4));
    println!("{}", output.as_ref().unwrap());
    println!("Running slow_function() took {} milliseconds.", output.unwrap().runtime);*/
}