use std::cmp::{max, min};
use std::collections::HashMap;

use crate::io::read_lines;

struct Line {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: usize,
    y: usize,
}

pub fn day05() {
    let filename = "input/day05/input.txt";

    for i in 2..=1 {
        print!("{}", i)
    }

    let mut lines: Vec<Line> = vec!();

    if let Ok(file_lines) = read_lines(filename) {
        for file_line in file_lines {
            if let Ok(s) = file_line {
                let line_count = s.split(" -> ").count();
                if line_count != 2 {
                    break;
                }

                let mut parts = s.split(" -> ");
                let mut part1 = parts.next().unwrap().split(",");
                let mut part2 = parts.next().unwrap().split(",");
                let line = Line {
                    x1: part1.next().unwrap().parse().unwrap(),
                    y1: part1.next().unwrap().parse().unwrap(),
                    x2: part2.next().unwrap().parse().unwrap(),
                    y2: part2.next().unwrap().parse().unwrap(),
                };
                lines.push(line)
            }
        }
    }

    part_a(lines.as_slice());
    println!();
    println!();
    part_b(lines.as_slice());
}

fn part_a(lines: &[Line]) {
    let mut points: Vec<Point> = vec!();
    let mut points_map: HashMap<Point, usize> = HashMap::new();

    // Generate points
    for line in lines {
        if line.x1 == line.x2 {
            for i in min(line.y1, line.y2)..=max(line.y1, line.y2) {
                points.push(Point { x: line.x1, y: i });
            }
        } else if line.y1 == line.y2 {
            for i in min(line.x1, line.x2)..=max(line.x1, line.x2) {
                points.push(Point { x: i, y: line.y1 });
            }
        }
    }

    // Count points occurrence
    for point in points {
        *points_map.entry(point).or_default() += 1;
    }

    println!("{}", points_map.iter().filter(|(_, c)| **c > 1).count());

    print_map(&mut points_map)
}

fn part_b(lines: &[Line]) {
    let mut points: Vec<Point> = vec!();
    let mut points_map: HashMap<Point, usize> = HashMap::new();

    // Generate points
    for line in lines {
        if line.x1 == line.x2 {
            for i in min(line.y1, line.y2)..=max(line.y1, line.y2) {
                points.push(Point { x: line.x1, y: i });
            }
        } else if line.y1 == line.y2 {
            for i in min(line.x1, line.x2)..=max(line.x1, line.x2) {
                points.push(Point { x: i, y: line.y1 });
            }
        } else {
            let mut x = line.x1;
            let mut y = line.y1;
            while x != line.x2 && y != line.y2 {
                points.push(Point { x, y });
                x = if x < line.x2 { x + 1 } else { x - 1 };
                y = if y < line.y2 { y + 1 } else { y - 1 };
            }
            points.push(Point { x, y });
        }
    }

    // Count points occurrence
    for point in points {
        *points_map.entry(point).or_default() += 1;
    }

    println!("{}", points_map.iter().filter(|(_, c)| **c > 1).count());

    print_map(&mut points_map)
}

fn print_map(points_map: &mut HashMap<Point, usize>) {
    for y in 0..=9 {
        for x in 0..=9 {
            print!("{}", points_map.entry(Point { x, y }).or_default());
        }
        print!("\n")
    }
}
