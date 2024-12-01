use std::fs;
use std::io;
use std::io::BufRead;
use std::iter::zip;
use std::collections::HashMap;
use std::hash::Hash;

fn part1(mut col1: Vec<i32>, mut col2: Vec<i32>) {
    col1.sort();
    col2.sort();

    let mut sum: i32 = 0;

    for i in zip(col1, col2) {
        let (a, b) = i;
        let diff = (b-a).abs();
        //println!("{i:?}, diff: {diff}");
        sum += diff;
    }

    println!("part1 sum: {sum}");
}

fn histogram<T>(v: &Vec<T>) -> HashMap<T, u32> where T: Eq, T: Hash, T: Clone, T: Copy {
    let mut result: HashMap<T, u32> = HashMap::new();
    for i in v {
        result.entry(*i)
            .and_modify(|value| *value += 1)
            .or_insert(1);
    }

    result
}

fn part2(col1: &Vec<i32>, col2: &Vec<i32>) {
    let hg = histogram(&col2);

    let pairing = col1.iter()
        .map(|number| (number.clone(), hg.get(&number).unwrap_or(&0u32).clone()))
        .collect::<Vec<(i32, u32)>>();

    let mut similarity_score = 0;
    for (num, occurrences) in pairing {
        similarity_score += num as u32 * occurrences;
    }
    println!("part2 similarity score: {similarity_score}");
}

fn main() {
    let file = fs::File::open("puzzle_input.txt")
    //let file = fs::File::open("test_input.txt")
        .expect("couldn't open file");
    let line_reader = io::BufReader::new(file).lines();
    let mut col1: Vec<i32> = Vec::new();
    let mut col2: Vec<i32> = Vec::new();
    for line in line_reader {
        if line.is_err() { break; }
        let line = line.unwrap();
        let splits = line.split(" ")
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>();
        if splits.len() != 2 { break; }
        let num1: i32 = splits[0].parse().unwrap();
        let num2: i32 = splits[1].parse().unwrap();
        col1.push(num1);
        col2.push(num2);
    }

    part1(col1.clone(), col2.clone());
    part2(&col1, &col2);
}