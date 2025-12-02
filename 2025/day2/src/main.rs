use std::collections::BTreeSet;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

#[derive(Debug)]
struct Range<'a> {
    low: u64,
    hi: u64,
    low_str: &'a str,
    hi_str: &'a str,
}

fn only_repeats(haystack: &str, part_len: usize) -> bool {
    if haystack.len() <= part_len { return false; }
    if haystack.len() % part_len != 0 { return false; }
    let needle = &haystack[0..part_len];
    for i in 1..(haystack.len() / part_len) {
        if &haystack[(i*part_len)..((i+1)*part_len)] != needle {
            return false;
        }
    }
    true
}

fn main() {
    let bytes = BufReader::new(File::open("input.txt").expect("failed to open file")).bytes()
        .filter(|b| b.as_ref().is_ok_and(|x| *x != ('\n' as u8)))
        .map(|b| b.unwrap())
        .collect::<Vec<_>>();
    let contents = String::from_utf8(bytes).unwrap();
    let ranges = contents.split(',')
        .collect::<Vec<&str>>();
    let mut range_objs: Vec<Range> = vec![];
    for range in ranges {
        let mut halfs = range.split("-");
        let low_str = halfs.next().unwrap();
        let low = low_str.parse::<u64>().unwrap();
        let hi_str = halfs.next().unwrap();
        let hi = hi_str.parse::<u64>().unwrap();
        let range = Range {
            low, hi,
            low_str, hi_str
        };
        range_objs.push(range);
    }

    let mut repeats: Vec<u64> = vec![];
    for range in &range_objs {
        let mut local_repeats: BTreeSet<u64> = BTreeSet::new();
        let low_half: u64 = if range.low < 10 {
            0
        } else {
            range.low_str[0..range.low_str.len() / 2].parse::<u64>().unwrap()
        };
        let hi_half: u64 = if range.hi_str.len() % 2 == 0 {
            range.hi_str[0..range.hi_str.len()/2].parse().unwrap()
        } else {
            let mut string = range.hi_str[0..range.hi_str.len() / 2].to_owned();
            string.push('0');
            string.parse::<u64>().unwrap() + 10
        };
        for i in low_half..=hi_half {
            let repeated = i.to_string().repeat(2).parse::<u64>().unwrap();
            if repeated > range.hi { break; }
            if repeated >= range.low {
                local_repeats.insert(i.to_string().repeat(2).parse::<u64>().unwrap());
            }
        }
        for r in local_repeats {
            repeats.push(r);
        }
    }

    let sum: u64 = repeats.iter().sum();
    println!("part1 sum: {sum}");

    let mut p2sum: u64 = 0;
    for range in &range_objs {
        println!("{range:?}");
        'nums_in_range: for num_in_range in range.low..=range.hi {
            let num_str = num_in_range.to_string();
            for repeat_len in 1..=range.hi_str.len()/2 {
                if only_repeats(&num_str, repeat_len) {
                    p2sum += num_in_range;
                    println!("r {num_in_range} rep_size:{repeat_len}");
                    continue 'nums_in_range;
                }
            }
        }
    }
    println!("part2 sum: {p2sum}");
}