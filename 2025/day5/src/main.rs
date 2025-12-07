use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = BufReader::new(File::open("input.txt").expect("failed to open file"))
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<_>>();
    let mut splits = input.split(|l| l.is_empty());
    let id_ranges = splits.next().unwrap().iter()
        .map(|l| {
            let mut it = l.split('-').map(|spl| spl.parse::<u64>().unwrap());
            (it.next().unwrap(), it.next().unwrap())
        }).collect::<Vec<_>>();
    let ids = splits.next().unwrap().iter().map(|l| l.parse::<u64>().unwrap()).collect::<Vec<_>>();
    for l in &id_ranges { println!("{l:?}"); }

    let mut fresh_count = 0;
    for i in &ids {
        if id_ranges.iter().any(|(lo, hi)| i >= lo && i <= hi) {
            println!("{i}  fresh");
            fresh_count += 1;
        } else {
            println!("{i} spoiled")
        }
    }
    println!("part1 fresh count: {fresh_count}");

}
