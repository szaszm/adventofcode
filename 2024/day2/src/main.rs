use std::{fs, io};
use std::io::BufRead;

fn part1_report_is_safe(v: &[u32]) -> bool {
    if v.len() < 2 { return true; }
    let increasing = v[1] > v[0];
    for i in 1..v.len() {
        if increasing && v[i] < v[i-1] || !increasing && v[i] > v[i-1] { return false; }
        let absdiff = v[i].abs_diff(v[i-1]);
        if absdiff < 1 || absdiff > 3 { return false; }
    }

    true
}

fn is_increasing(v: &[u32]) -> bool {
    let mut direction = 0;
    if v.len() < 2 { return false }
    for i in 1..v.len() {
        if v[i] > v[i-1] { direction += 1; }
        else if v[i] < v[i-1] { direction -= 1; }
    }
    direction > 0
}

/*
fn part2_report_is_safe(v: &[u32]) -> bool {
    if part1_report_is_safe(v) || part1_report_is_safe(&v[1..]) { return true; }
    let mut skipped: Option<usize> = None;
    let increasing = is_increasing(v);
    let first_absdiff = v[1].abs_diff(v[0]);
    if first_absdiff < 1 || first_absdiff > 3 {
        skipped = Some(1);
    }
    let start = if skipped == Some(1) { 2 } else { 1 };
    for i in start..v.len() {
        let last_index = if skipped != None && skipped == Some(i-1) { i-2 } else { i-1 };
        if increasing && v[i] < v[last_index] || !increasing && v[i] > v[last_index] {
            if skipped == None {
                skipped = Some(i);
                continue;
            } else {
                return false;
            }
        }
        let absdiff = v[i].abs_diff(v[last_index]);
        if absdiff < 1 || absdiff > 3 {
            if skipped == None {
                skipped = Some(i);
                continue;
            } else {
                return false;
            }
        }
    }

    true
}
*/

fn part2_report_is_safe_bruteforce(v: &[u32]) -> bool {
    if part1_report_is_safe(&v) { return true; }
    for i in 0..v.len() {
        let mut v = v.to_vec();
        v.remove(i);
        if part1_report_is_safe(&v) { return true; }
    }
    false
}

fn main() {
    let file = fs::File::open("puzzle_input.txt")
        .expect("couldn't open file");
    let line_reader = io::BufReader::new(file).lines();
    let mut reports: Vec<Vec<u32>> = Vec::new();
    for line in line_reader {
        if line.is_err() { break; }
        let line = line.unwrap();
        let report = line.split_whitespace()
            .map(|level| level.parse::<u32>().expect("failed to parse number"))
            .collect::<Vec<u32>>();
        reports.push(report);
    }

    //let safe_reports = reports.iter().filter(|r| part1_report_is_safe(*r)).count();
    let safe_reports = reports.iter().filter(|r| part2_report_is_safe_bruteforce(*r)).count();
    println!("safe reports: {safe_reports}");
}
