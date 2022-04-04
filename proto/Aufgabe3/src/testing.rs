#![feature(test)]
use super::io::*;
use super::processing::*;
use rand::thread_rng;
use rand::Rng;
use std::fs;

fn generate_testcase(n: usize) -> TInput {
    let chars = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'];
    let mut s: Vec<char> = vec!['0'; n];
    for i in 0..n {
        s[i] = chars[thread_rng().gen_range(0..16)];
    }
    TInput {s: s.iter().collect(), m: thread_rng().gen_range(0..(4.875*n as f64).floor() as u64)} //4.875 = average active segemnts
}

pub fn run_samples() {
    let chars = Characters::read_from(&manifest_plus("chars.json")).unwrap();
    for i in 0..6 {
        println!("Running hexmax{}.txt", i);
        let input = TInput::read_from(&workspace_plus(format!("eingaben/Aufgabe3/hexmax{}.txt", i))).unwrap();
        let output = process(&input, &chars, i < 3);
        assert!(output.verify(&chars));
        println!("Took {}ms", output.runtime);
        fs::write(manifest_plus(format!("ausgaben/ausgabe{}.txt", i)), format!("{}", output)).unwrap();
    }
}

pub fn run_randomized(r: usize, maxn: usize, save_runtimes: bool) {
    let chars = Characters::read_from(&manifest_plus("chars.json")).unwrap();
    let mut times: Vec<(usize, u128)> = vec![];
    for _ in 0..r {
        let input = generate_testcase(thread_rng().gen_range(1..maxn+1));
        let output = process(&input, &chars, input.s.len() < 40);
        assert!(output.verify(&chars));
        if save_runtimes {
            times.push((input.s.len(), output.runtime));
        }
    }
    if save_runtimes {
        let mut wstring: Vec<String> = vec![];
        for ti in times {
            wstring.push(format!("{} {}", ti.0, ti.1));
        }
        let wstring = wstring.join("\n");
        fs::write(&manifest_plus("benchmarks/runtimes1.txt"), wstring).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run_tests() {
        let r = 200;let maxn = 500;
        run_randomized(r, maxn, true);
    }
    #[test]
    fn test_samples() {
        run_samples();
    }
}
