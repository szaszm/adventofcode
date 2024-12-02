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

fn part2_report_is_safe_impl(v: &[u32]) -> bool {
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
fn part2_report_is_safe(v: &[u32]) -> bool {
    // instead of fixing the algorithm, running it backwards filters out the few remaining false unsafes
    if part2_report_is_safe_impl(v) { return true }
    let mut reversed = v.to_vec();
    reversed.reverse();
    part2_report_is_safe_impl(&reversed)
}

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

    let p1_safe_reports = reports.iter().filter(|r| part1_report_is_safe(*r)).count();
    let p2_safe_reports = reports.iter().filter(|r| part2_report_is_safe(*r)).count();
    let p2_broken_attempt_safe_reports = reports.iter().filter(|r| part2_report_is_safe_impl(*r)).count();
    let p2_safe_reports_quadratic = reports.iter().filter(|r| part2_report_is_safe_bruteforce(*r)).count();
    println!("part 1 safe reports: {p1_safe_reports}");
    println!("part 2 quadratic algo - safe reports: {p2_safe_reports_quadratic}");
    println!("part 2 linear algo - safe reports: {p2_safe_reports}");
    println!("part 2 broken linear algo - safe reports: {p2_broken_attempt_safe_reports}");

    println!("\nBroken algo mistakes:");
    for r in reports {
        let quadratic_is_safe = part2_report_is_safe_bruteforce(&r);
        let broken_linear_is_safe = part2_report_is_safe_impl(&r);
        if quadratic_is_safe != broken_linear_is_safe {
            println!("difference: {}, but incorrectly detected as {}: \t {:?}",
                if quadratic_is_safe { "safe" } else { "unsafe" },
                if broken_linear_is_safe { "safe" } else { "unsafe" },
                r);
        }
    }

    // Output:
    // part 1 safe reports: 526
    // part 2 quadratic algo - safe reports: 566
    // part 2 linear algo - safe reports: 566
    // part 2 broken linear algo - safe reports: 564
    //
    // Broken algo mistakes:
    // difference: safe, but incorrectly detected as unsafe:    [79, 80, 83, 81, 82]
    // difference: safe, but incorrectly detected as unsafe:    [16, 13, 11, 8, 9, 8]
}
