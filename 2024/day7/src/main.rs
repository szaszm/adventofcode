use std::fs::File;
use std::io::{BufRead, BufReader};

struct Equation {
    test_value: u64,
    numbers: Vec<u64>
}

fn run_numbers(numbers: &[u64], operations_bitset: u64) -> u64 {
    let mut acc = *numbers.first().expect("first number");
    for i in 1..numbers.len() {
        let bit = operations_bitset & ((1 << (numbers.len() - 1)) >> i) != 0;
        if bit {
            acc *= numbers[i];
        } else {
            acc += numbers[i];
        }
    }
    acc
}

fn print_solution(numbers: &[u64], operations_bitset: u64) {
    let mut acc = *numbers.first().expect("first number");
    print!("{}", acc);
    for i in 1..numbers.len() {
        let bit = operations_bitset & ((1 << (numbers.len() - 1)) >> i) != 0;
        if bit {
            print!(" * {}", numbers[i]);
            acc *= numbers[i];
        } else {
            print!(" + {}", numbers[i]);
            acc += numbers[i];
        }
    }
    println!(" = {}", acc);
}

fn main() {
    //let lines = BufReader::new(File::open("test_input.txt").expect("failed to open file")).lines();
    let lines = BufReader::new(File::open("puzzle_input.txt").expect("failed to open file")).lines();
    let mut equations: Vec<Equation> = vec![];
    for l in lines {
        if l.is_err() { break; }
        let l = l.unwrap();
        let mut sides = l.split(':');
        let test_value: u64 = sides.next().unwrap().parse().expect(format!("failed to parse line {}", l).as_str());
        let numbers: Vec<u64> = sides.next().unwrap().split_whitespace().map(|elem| elem.parse().unwrap()).collect();
        equations.push(Equation{ test_value, numbers });
    }

    let mut sum = 0;
    for eq in equations {
        let len = eq.numbers.len();
        for i in 0..(1 << (len - 1)) {
            let n = run_numbers(&eq.numbers, i);
            if n == eq.test_value {
                print_solution(&eq.numbers, i);
                sum += eq.test_value;
                break;
            }
        }
    }
    println!("part1 sum: {sum}");
}
