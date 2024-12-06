use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Point(usize, usize);

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}

fn direction_from_char(c: char) -> Direction {
    match c {
        '^' => Direction::Up,
        '>' => Direction::Right,
        'v' => Direction::Down,
        '<' => Direction::Left,
        _ => panic!("invalid direction {c}"),
    }
}

fn grid_at(p: Point, grid: &Vec<Vec<char>>) -> char {
    grid[p.1][p.0]
}

fn turn_right(d: Direction) -> Direction {
    match d {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn point_move(p: Point, direction: Direction, grid: &Vec<Vec<char>>) -> Option<Point> {
    let Point(x, y) = p;
    match direction {
        Direction::Up => {
            if y == 0 { return None; }
            Some(Point(x, y - 1))
        },
        Direction::Right => {
            if x == grid[y].len() - 1 { return None; }
            Some(Point(x + 1, y))
        },
        Direction::Down => {
            if y == grid.len() - 1 { return None; }
            Some(Point(x, y + 1))
        },
        Direction::Left => {
            if x == 0 { return None; }
            Some(Point(x - 1, y))
        }
    }
}

fn main() {
    let file = BufReader::new(File::open("test_input.txt").expect("failed to open file"));
    //let file = BufReader::new(File::open("puzzle_input.txt").expect("failed to open file"));
    let mut grid: Vec<Vec<char>> = vec![];
    let mut guard_pos: Point = Point(0,0);
    for (line_num, line) in file.lines().enumerate() {
        if line.is_err() { break; }
        let line = line.unwrap();
        if let Some(pos) = line.find(&['^', '>', 'v', '<']) {
            guard_pos = Point(pos, line_num);
        }
        grid.push(line.chars().collect());
        println!("{line}");
    }

    let mut step = 0;
    let mut direction = direction_from_char(grid_at(guard_pos, &grid));
    let mut obstructions: Vec<Point> = vec![];
    'outer: loop {
        println!("step {step:3}    pos: {guard_pos:?}, direction: {:?}", direction);
        grid[guard_pos.1][guard_pos.0] = 'X';
        let mut next = Point(0,0);
        let mut turns = 0;
        loop {
            if let Some(maybe_next) = point_move(guard_pos, direction, &grid) {
                next = maybe_next;
            } else {
                break 'outer;
            }
            if grid_at(next, &grid) == '#' {
                direction = turn_right(direction);
                turns += 1;
                if turns > 3 { panic!("infinite turning loop"); }
                continue;
            } else {
                break;
            }
        }
        // part 2: If we have already visited the position to the left, then placing an obstruction
        // on next would MAYBE get us in a loop
        // TODO: walk the path to check for loop. The loop will be a rectangle.
        if let Some(on_our_right) = point_move(guard_pos, turn_right(direction), &grid) {
            if turns == 0 && grid_at(on_our_right, &grid) == 'X' {
                println!("possible obstruction position: {next:?}");
                if !obstructions.contains(&next) {
                    obstructions.push(next);
                }
            }
        }

        guard_pos = next;
        step += 1;
    }
    let mut xes = 0;
    for l in &grid {
        xes += l.iter().filter(|&c| *c == 'X').count();
        let line: String = l.into_iter().collect();
        println!("{line}");
    }
    println!("part1: distinct visited: {xes}");
    println!("part2: obstructions: {obstructions:?}");
}