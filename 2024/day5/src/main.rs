use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn p1_pages_follow_rules(pages: &[u32], rules: &[(u32, u32)]) -> bool {
    let mut seen: HashSet<u32> = HashSet::new();
    let rules = {
        let mut rules_table: HashMap<u32, Vec<u32>> = HashMap::new();
        for &(pre, post) in rules {
            rules_table.entry(pre).or_insert(Vec::new()).push(post);
        }
        rules_table
    };
    for num in pages {
        let rules_for_num = rules.get(num);
        if let Some(rules) = rules_for_num {
            if rules.iter().any(|post| seen.contains(post)) {
                return false;
            }
        }
        seen.insert(*num);
    }
    true
}

fn main() {
    let (rules, updates) = {
        let file = BufReader::new(File::open("test_input.txt").expect("failed to open file"));
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
        (rules, pages)
    };
    let good_updates = updates.iter().filter(|update| p1_pages_follow_rules(update, &rules));
    let mut sum = 0;
    for pages in good_updates {
        let middle_number = pages.get(pages.len() / 2).unwrap();
        sum += middle_number;
        println!("{:?}  mid: {middle_number}", pages);
    }
    println!("part 1: sum of middle numbers of good updates: {sum}\n");

    let bad_updates = updates.iter()
        .filter(|update| !p1_pages_follow_rules(update, &rules))
        .collect::<Vec<_>>();
    for pages in updates {
        println!("{pages:?}");
    }
}
