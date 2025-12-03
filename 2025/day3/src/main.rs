extern crate core;

use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Num = i64;
const NUM_OF_BATTERIES_PART2: usize = 12;

fn max_joltage_part1(line: &[Num]) -> Num {
    let mut max: Num = 0;
    for i in 0..line.len() {
        for j in (i+1)..line.len() {
            max = cmp::max(max, line[i] * 10 + line[j]);
        }
    }
    max
}

fn max_joltage_part2(line: &[Num], num_of_batteries: usize) -> Num {
    // we need to pick 12 digits, and eagerly find the biggest number for our digit each time,
    // starting after the last max digit, while making sure to leave enough numbers behind
    // for the rest of the digits (future iterations).
    let mut max: Num = 0;
    let mut maxi: Option<usize> = None;

    for nth_digit in 0..num_of_batteries {
        // start search after the index of the last found digit, or at the beginning the first time
        let start_idx = maxi.map(|v| v+1).unwrap_or(0);;
        let end_idx = line.len() - (num_of_batteries - 1) + nth_digit;

        let (new_maxi, new_maxv) = &line[start_idx..end_idx]
            .iter()
            .enumerate()
            .map(|(k, v)| (k + start_idx, v))
            .rev()  // reverse because max_by_key returns the last max element but we need the first
            .max_by_key(|(_, v)| *v)
            .expect("The index calculations above should make sure the slice is never empty");

        max *= 10;
        max += *new_maxv;
        maxi = Some(*new_maxi);
        // println!("[{nth_digit}]: [{start_idx}..{end_idx}]  maxi:{new_maxi}  maxv:{new_maxv}  max:{max}");
    }

    max
}

fn main() {
    let input = BufReader::new(File::open("input.txt").expect("failed to open file")).lines()
        .filter(|l| l.is_ok())
        .map(|l| l.expect("filter above ensures that only Ok() elements remain"))
        .map(|l| l.chars()
            .map(|ch| ch.to_digit(10).expect("char must be number") as Num)
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut sum_p1 = 0;
    let mut sum_p2 = 0;
    for line in input {
        for num in &line {
            print!("{num}");
        }
        let mj_p1 = max_joltage_part1(&line);  // same as max_joltage_part2(&line, 2)
        let mj_p2 = max_joltage_part2(&line, NUM_OF_BATTERIES_PART2);
        println!("\tmax_joltage part1: {mj_p1}  part2: {mj_p2}");
        sum_p1 += mj_p1;
        sum_p2 += mj_p2;
    }
    println!("sum of joltages part1: {sum_p1}  part2: {sum_p2}");
}
