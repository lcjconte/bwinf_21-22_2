use Aufgabe3::processing::*;
use Aufgabe3::io::*;

fn main() {
    let chars = Characters::read_from(&manifest_plus("chars.json")).unwrap();
    let input = TInput::read_from(&workspace_plus(format!("eingaben/Aufgabe3/hexmax{}.txt", 0))).unwrap();
    let output = process(&input, &chars, true);
    println!("Processed!");
    println!("Valid: {:?}", output.verify(&chars));
    println!("Result: {}", output.s);
    let st = chars.string_steps(&chars.stovec(&input.s), &chars.stovec(&output.s));
    for step in st {
        println!("{:?} {:?}", step.from, step.to);
    }
    /*for i in 0..6 {
        let tInput = TInput::read_from(&format!("eingaben/Aufgabe3/hexmax{}.txt", i)).unwrap();
        proc2(tInput);
    }*/
    
}