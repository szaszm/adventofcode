use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn make_rule_table(rules: &[(u32, u32)]) -> HashMap<u32, Vec<(u32, u32)>> {
    let mut rules_table: HashMap<u32, Vec<(u32, u32)>> = HashMap::new();
    for &(pre, post) in rules {
        rules_table.entry(pre).or_insert(Vec::new()).push((pre, post));
        rules_table.entry(post).or_insert(Vec::new()).push((pre, post));
    }
    rules_table
}

fn p1_pages_follow_rules(pages: &[u32], rules: &HashMap<u32, Vec<(u32, u32)>>) -> bool {
    let mut seen: HashSet<u32> = HashSet::new();
    for num in pages {
        let rules_for_num = rules.get(num);
        if let Some(rules) = rules_for_num {
            if rules.iter().any(|(_, post)| seen.contains(post)) {
                return false;
            }
        }
        seen.insert(*num);
    }
    true
}

fn main() {
    let (rules, updates) = {
        let file = BufReader::new(File::open("puzzle_input.txt").expect("failed to open file"));
        let mut rules_lines: Vec<String> = vec![];
        let mut pages_lines: Vec<String> = vec![];
        let mut rules_end = false;
        for line in file.lines() {
            if line.is_err() { break; }
            let line = line.unwrap();
            if line.is_empty() { rules_end = true; continue; }
            if !rules_end {
                rules_lines.push(line);
            } else {
                pages_lines.push(line);
            }
        }
        let rules = rules_lines.iter()
            .map(|line| {
                let mut iter = line
                    .split('|')
                    .map(|num| num
                        .parse::<u32>()
                        .expect("failed to parse number {num}"));
                (iter.next().unwrap(), iter.next().unwrap())
            }).collect::<Vec<_>>();
        let pages = pages_lines.iter()
            .map(|line| line.split(",")
                .map(|num| num.parse::<u32>().expect("failed to parse number {num}"))
                .collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let rules = make_rule_table(&rules);
        (rules, pages)
    };
    let good_updates = updates.iter().filter(|update| p1_pages_follow_rules(update, &rules));
    let mut p1sum = 0;
    for pages in good_updates {
        let middle_number = pages.get(pages.len() / 2).unwrap();
        p1sum += middle_number;
        println!("{:?}  mid: {middle_number}", pages);
    }
    println!("part 1: sum of middle numbers of good updates: {p1sum}\n");

    let bad_updates = updates.iter()
        .filter(|update| !p1_pages_follow_rules(update, &rules))
        .map(|v| v.clone())
        .collect::<Vec<_>>();
    let mut p2sum = 0;
    for pages in &bad_updates {
        let mut pages = pages.clone();
        println!("{pages:?}");
        pages.sort_by(|a, b| {
            rules.get(a).map(|r| {
                for (smol, chonk) in r {
                    if smol == b {
                        return Ordering::Less;
                    }
                    if chonk == b {
                        return Ordering::Greater;
                    }
                }
                Ordering::Equal
            }).unwrap_or(Ordering::Equal)
        });
        println!("after sort: {pages:?}");
        let middle_number = pages.get(pages.len() / 2).unwrap();
        p2sum += middle_number;
    }
    println!("\npart 1: sum of middle numbers of good updates: {p1sum}");
    println!("part 2: sum of middle numbers of bad updates after sort: {p2sum}");
}
