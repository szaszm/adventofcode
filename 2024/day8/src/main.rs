use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

struct Point(i32, i32);
struct Vec2(i32, i32);
struct Rect(Point, Vec2);
impl Point {
    fn sub(&self, other: &Point) -> Vec2 {
        Vec2(self.0 - other.0, self.1 - other.1)
    }
    fn add(&self, vec: &Vec2) -> Point {
        Point(self.0 + vec.0, self.1 + vec.1)
    }
    fn mul(&self, n: i32) -> Point {
        Point(self.0 * n, self.1 * n)
    }
}
impl Vec2 {

}

fn main() {
    let map: Vec<Vec<char>> = BufReader::new(File::open("test_input.txt").expect("failed to open file")).lines()
        .filter(|line| line.is_ok())
        .map(|line| line.unwrap().chars().collect())
        .collect();
    let is_frequency = |c: char| c != '.' && c.is_ascii() && c.is_alphanumeric();
    let frequencies: Vec<char> = map.iter()
        .flatten()
        .filter(|c| is_frequency(**c))
        .unique()
        .map(|&c| c)
        .collect();
    let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
    let mut antinodes: HashMap<char, Point> = HashMap::new();
    for (y, line) in &map.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            print!("{c}");
            if is_frequency(c) {
                antennas.entry(c).or_insert(Vec::new()).push(Point(x, y));
            }
        }
        println!();
    }

}
