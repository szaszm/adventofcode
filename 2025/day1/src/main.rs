use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Rotation {
    Left(u32),
    Right(u32),
}

fn from_line(line: &str) -> Option<Rotation> {
    if line.len() < 1 { return None; }
    let first_byte = line.as_bytes()[0];
    if first_byte != 'L' as u8 && first_byte != 'R' as u8 { return None; }
    let parse_result = line[1..].parse::<u32>();
    if let Ok(num) = parse_result {
        if first_byte == 'L' as u8 {
            Some(Rotation::Left(num))
        } else {
            Some(Rotation::Right(num))
        }
    } else {
        None
    }
}

fn positive_mod(n: i32, q: i32) -> i32 {
    if n < 0 { n % q + if n % q != 0 { q } else { 0 }} else { n % q }
}

fn main() {
    let lines = BufReader::new(File::open("input.txt").expect("failed to open file")).lines()
        .filter(|line| line.is_ok())
        .flat_map(|line| from_line(&line.unwrap()))
        .collect::<Vec<_>>();
    let mut acc: i32 = 50;
    let mut num_of_zeroes_part1 = 0;
    let mut num_of_clicks_part2 = 0;
    for line in lines {
        let mut diff_this_cycle = 0;
        if let Rotation::Left(num) = line {
            diff_this_cycle -= num as i32
        } else if let Rotation::Right(num) = line {
            diff_this_cycle += num as i32;
        }
        let accmod100 = positive_mod(acc, 100);
        let tmp = accmod100 + diff_this_cycle;
        let clicks_this_cycle = if accmod100 != 0 && tmp < 1 { i32::abs(tmp) / 100 + 1 } else { i32::abs(tmp) / 100 };
        acc += diff_this_cycle;
        if acc == 0 { num_of_zeroes_part1 += 1; }
        let accmod100 = positive_mod(acc, 100);
        println!("{line:?} \tacc: {acc}/{accmod100}\tzeroes: {num_of_zeroes_part1}\ttmp: {tmp}  \tclicks_this_cycle(part2): {clicks_this_cycle}");
        num_of_clicks_part2 += clicks_this_cycle;
    }
    println!("num of clicks (part2): {num_of_clicks_part2}");
}
