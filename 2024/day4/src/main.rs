use std::fs;
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone, Debug)]
struct Point(i32, i32);
#[derive(Copy, Clone)]
struct Vec2(i32, i32);

fn search_in_direction(direction: Vec2, starting_point: Point, line_len: usize, lines: &[&str], searched: &str) -> bool {
    let mut point = starting_point;
    for c in searched.chars() {
        let Point(x, y) = point;
        if x < 0 || x as usize >= line_len || y < 0 || y as usize >= lines.len() {
            return false;
        }
        let char = lines[y as usize].chars().nth(x as usize);
        if char.unwrap() != c {
            return false;
        }
        point.0 += direction.0;
        point.1 += direction.1;
    }
    true
}

fn search_part1(starting_point: Point, lines: &[&str], line_len: usize, searched: &str) -> u8 {
    const DIRECTIONS: [Vec2; 8] = [
        Vec2(1, 0),
        Vec2(1, 1),
        Vec2(0, 1),
        Vec2(-1, 1),
        Vec2(-1, 0),
        Vec2(-1, -1),
        Vec2(0, -1),
        Vec2(1, -1)
    ];
    let mut matches = 0;
    for dir in DIRECTIONS {
        if search_in_direction(dir, starting_point, line_len, lines, searched) {
            matches += 1;
        }
    }
    //if matches > 0 { println!("[{starting_point:?}] matches: {matches}"); }
    matches
}

fn search_part2(starting_point: Point, lines: &[&str], line_len: usize, searched: &str) -> bool {
    const DIRECTIONS: [Vec2; 4] = [
        Vec2(-1, -1),
        Vec2(1, -1),
        Vec2(1, 1),
        Vec2(-1, 1),
    ];
    let mut matches = 0;
    for dir in DIRECTIONS {
        let half_len = (searched.len() / 2) as i32;
        let point = Point(starting_point.0 - dir.0 * half_len, starting_point.1 - dir.1 * half_len);
        if search_in_direction(dir, point, line_len, lines, searched) {
            matches += 1;
        }
    }
    let result = matches == 2;
    //if result { println!("[{starting_point:?}] match"); }
    result
}

fn main() {
    let lines: Vec<String> = {
        let file = fs::File::open("puzzle_input.txt")
            .expect("couldn't open file");
        let reader = BufReader::new(file);
        reader.lines().map(Result::unwrap).collect()
    };
    let lines: Vec<&str> = lines.iter().map(String::as_str).collect();
    let line_length = lines[0].len();
    let mut matches_part1: u32 = 0;
    let mut matches_part2: u32 = 0;
    for line_num in 0..lines.len() {
        for col_num in 0..line_length {
            matches_part1 += search_part1(Point(col_num as i32, line_num as i32), &lines, line_length, "XMAS") as u32;
            matches_part2 += search_part2(Point(col_num as i32, line_num as i32), &lines, line_length, "MAS") as u32;
        }
    }
    println!("matches part 1: {matches_part1}");
    println!("matches part 2: {matches_part2}");
}
