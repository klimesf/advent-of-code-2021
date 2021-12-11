use std::collections::HashSet;

use crate::io::read_lines;

pub(crate) fn day11() {
    let filename = "input/11.txt";

    let mut numbers: Vec<Vec<u32>> = vec!();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                let mut row: Vec<u32> = vec!();
                for c in ip.chars() {
                    row.push(c.to_digit(10).unwrap());
                }
                numbers.push(row);
            }
        }
    }

    let mut flash_counter: u64 = 0;
    for i in 0..1000 {
        let flash_count = simulate_step(&mut numbers) as u64;
        if flash_count == 100 {
            println!("first sync: {}", i + 1);
            break; // assuming the first sync happens after 100th iteration
        }
        flash_counter += flash_count;
        if i == 99 {
            println!("after 100 iterations: {}", flash_counter)
        }
    }
}

fn simulate_step(numbers: &mut Vec<Vec<u32>>) -> usize {
    let mut to_flash: Vec<Point> = vec!();
    for x in 0..numbers.len() {
        for y in 0..numbers[x].len() {
            numbers[x][y] += 1;
            if numbers[x][y] > 9 {
                to_flash.push(Point { x, y });
            }
        }
    }

    let mut flashes: HashSet<Point> = HashSet::new();

    while !to_flash.is_empty() {
        let p = to_flash.pop().unwrap();

        if flashes.contains(&p) {
            continue; // Already flashed
        }

        let x = p.x;
        let y = p.y;

        if numbers[x][y] > 9 && !flashes.contains(&p) {
            for n in neighbors(p, numbers.len()) {
                numbers[n.x][n.y] += 1;
                if numbers[n.x][n.y] > 9 {
                    to_flash.push(n);
                }
            }
            flashes.insert(p);
        }
    }

    let flash_count = flashes.len();

    for p in flashes {
        numbers[p.x][p.y] = 0;
    }

    return flash_count;
}

fn neighbors(p: Point, max: usize) -> Vec<Point> {
    let x = p.x as i32;
    let y = p.y as i32;
    let mut res = vec!();

    add_point(x - 1, y, &mut res, max);
    add_point(x + 1, y, &mut res, max);
    add_point(x, y - 1, &mut res, max);
    add_point(x, y + 1, &mut res, max);
    add_point(x - 1, y - 1, &mut res, max);
    add_point(x + 1, y - 1, &mut res, max);
    add_point(x - 1, y + 1, &mut res, max);
    add_point(x + 1, y + 1, &mut res, max);

    return res;
}

fn add_point(x: i32, y: i32, res: &mut Vec<Point>, max: usize) {
    if x >= 0 && y >= 0 && x < max as i32 && y < max as i32 {
        res.push(Point { x: x as usize, y: y as usize })
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}
