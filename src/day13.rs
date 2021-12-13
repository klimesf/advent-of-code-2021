use crate::io::read_lines;
use std::collections::HashSet;

pub(crate) fn day13() {
    let filename = "input/13.txt";

    let mut points: HashSet<Point> = HashSet::new();
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut points_loaded = false;

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                if ip.trim().is_empty() {
                    points_loaded = true;
                    continue;
                }
                if points_loaded {
                    let mut s = ip.split("=");
                    let direction: char = s.next().unwrap().chars().last().unwrap();
                    let pos: usize = s.next().unwrap().parse().unwrap();
                    instructions.push(Instruction { pos, direction })
                } else {
                    let mut s = ip.split(",");
                    let x: usize = s.next().unwrap().parse().unwrap();
                    let y: usize = s.next().unwrap().parse().unwrap();
                    points.insert(Point { x, y });
                }
            }
        }
    }

    for (cnt, i) in instructions.iter().enumerate() {
        let mut result: HashSet<Point> = HashSet::new();
        for p in points {
            if i.direction == 'x' {
                if p.x > i.pos {
                    result.insert(Point {
                        x: i.pos - (p.x - i.pos),
                        y: p.y,
                    });
                } else {
                    result.insert(p);
                }
            } else {
                if p.y > i.pos {
                    result.insert(Point {
                        x: p.x,
                        y: i.pos - (p.y - i.pos),
                    });
                } else {
                    result.insert(p);
                }
            }
        }
        let remaining_count = result.len();
        points = result;
        println!("{}: {}", cnt, remaining_count);
    }

    let mut printer = [['.'; 10]; 60];
    for p in &points {
        printer[p.x][p.y] = '#';
    }

    for y in 0..10 {
        for x in 0..60 {
            print!("{}", printer[x][y]);
        }
        println!();
    }
    println!();
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Debug)]
struct Instruction {
    pos: usize,
    direction: char,
}
