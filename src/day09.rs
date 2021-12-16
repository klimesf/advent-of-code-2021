use crate::io::read_lines;
use std::collections::HashMap;

pub(crate) fn day09() {
    let filename = "input/day09/input.txt";
    let mut map: Vec<Vec<u32>> = vec!();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            let mut row: Vec<u32> = vec!();
            for c in line.unwrap().chars() {
                row.push(c.to_digit(10).unwrap());
            }
            map.push(row);
        }
    }

    let mut sum = 0;
    let mut low_points: Vec<Point> = vec!();
    for x in 0..map.len() {
        for y in 0..map[x].len() {
            let mut is_lowest = true;
            let num = map[x][y];
            if x > 0 && num >= map[x - 1][y] {
                is_lowest = false;
            }
            if x < map.len() - 1 && num >= map[x + 1][y] {
                is_lowest = false;
            }
            if y > 0 && num >= map[x][y - 1] {
                is_lowest = false;
            }
            if y < map[x].len() - 1 && num >= map[x][y + 1] {
                is_lowest = false;
            }
            if is_lowest {
                sum += num + 1;
                low_points.push(Point { x, y, z: num })
            }
        }
    }
    println!("{}", sum); // part A

    let mut basin_size: HashMap<usize, i32> = HashMap::new();
    for (i, p) in low_points.iter().enumerate() {
        let mut stack: Vec<Point> = vec!();

        // depth-first search
        stack.append(&mut neighbours(p.x, p.y, p.z));
        while !stack.is_empty() {
            let scrutinized = stack.pop();
            let s = scrutinized.unwrap();
            let x = s.x;
            let y = s.y;

            if x >= map.len() || y >= map[x].len() {
                continue; // out of bounds
            }

            if map[x][y] != 9 && map[x][y] > s.z {
                *basin_size.entry(i).or_insert(1) += 1;
                stack.append(&mut neighbours(x, y, map[x][y]));
                map[x][y] = 9; // insert 9 to mark it as visited
            }
        }
    }

    let mut x: Vec<i32> = basin_size.values().cloned().collect();
    x.sort_by(|a, b| b.cmp(a));
    println!("{}", x[0] * x[1] * x[2]);
}

fn neighbours(x: usize, y: usize, z: u32) -> Vec<Point> {
    let mut result = vec!();

    result.push(Point { x: x + 1, y, z });
    result.push(Point { x, y: y + 1, z });

    // protect from underflow, overflow is handled within the algorithm
    if x > 0 {
        result.push(Point { x: x - 1, y, z });
    }
    if y > 0 {
        result.push(Point { x, y: y - 1, z });
    }

    return result;
}

struct Point {
    x: usize,
    y: usize,
    z: u32,
}
