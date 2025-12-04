use std::fs::File;
use std::io::{BufRead, BufReader};

fn roll_at(grid: &Vec<Vec<bool>>, x: isize, y: isize) -> u64 {
    if y < 0 || y >= grid.len() as isize { return 0; }
    if x < 0 || x >= grid[y as usize].len() as isize { return 0; }

    if grid[y as usize][x as usize] {
        1
    } else {
        0
    }
}

fn rolls_around(grid: &Vec<Vec<bool>>, x: usize, y: usize) -> u64 {
    roll_at(grid,   x as isize - 1, y as isize - 1)
    + roll_at(grid, x as isize + 0, y as isize - 1)
    + roll_at(grid, x as isize + 1, y as isize - 1)
    + roll_at(grid, x as isize - 1, y as isize + 0)
    + roll_at(grid, x as isize + 1, y as isize + 0)
    + roll_at(grid, x as isize - 1, y as isize + 1)
    + roll_at(grid, x as isize + 0, y as isize + 1)
    + roll_at(grid, x as isize + 1, y as isize + 1)
}

fn part2(mut grid: Vec<Vec<bool>>) -> i32 {
    let mut p2_number_of_removed = 0;
    let mut number_of_removed_this_cycle = -1;
    while number_of_removed_this_cycle != 0 {
        number_of_removed_this_cycle = 0;
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] && rolls_around(&grid, x, y) < 4 {
                    println!("[{x},{y}]");
                    number_of_removed_this_cycle += 1;
                    grid[y][x] = false;
                }
            }
        }
        p2_number_of_removed += number_of_removed_this_cycle;
    }

    p2_number_of_removed
}

fn main() {
    let input = BufReader::new(File::open("input.txt").expect("failed to open file")).lines()
        .filter(|l| l.is_ok())
        .map(|l| l.expect("filter above ensures that only Ok() elements remain"))
        .map(|l| l.chars()
            .map(|ch| ch == '@')
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut p1_number_of_accessible = 0;
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if input[y][x] && rolls_around(&input, x, y) < 4 {
                println!("[{x},{y}]");
                p1_number_of_accessible += 1;
            }
        }
    }
    println!("part1: {p1_number_of_accessible}");
    let p2 = part2(input);
    println!("part2: {p2}");
}
