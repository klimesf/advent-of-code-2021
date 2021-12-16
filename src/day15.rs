use crate::io::read_lines;
use priority_queue::double_priority_queue::DoublePriorityQueue;

pub(crate) fn day15() {
    let filename = "input/day15/input.txt";
    part_a(filename);
    part_b(filename);
}

fn part_a(filename: &str) {
    let mut risk: Vec<Vec<u32>> = vec!();
    let mut distances: Vec<Vec<u32>> = vec!();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            let mut row: Vec<u32> = vec!();
            if let Ok(ip) = line {
                for c in ip.chars() {
                    row.push(c.to_digit(10).unwrap());
                }
            }
            let row_len = row.len();
            risk.push(row);
            distances.push(vec![u32::MAX; row_len])
        }
    }

    distances[0][0] = 0;

    find_shortest_path(&mut risk, &mut distances);

    println!("{}", distances.last().unwrap().last().unwrap());
}

fn part_b(filename: &str) {
    let mut risk: Vec<Vec<u32>> = vec!();
    let mut distances: Vec<Vec<u32>> = vec!();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            let mut row: Vec<u32> = vec!();
            if let Ok(ip) = line {
                for c in ip.chars() {
                    row.push(c.to_digit(10).unwrap());
                }
            }
            let row_len = row.len();

            // Aditional cols
            for inc in 1..5 {
                for i in 0..row_len {
                    row.push(wrap(row[i] + inc));
                }
            }

            risk.push(row);
            distances.push(vec![u32::MAX; row_len * 5])
        }
    }

    let risk_len = risk.len();
    // Aditional rows
    for inc in 1..5 {
        for i in 0..risk_len {
            risk.push(risk[i].clone().iter().map(|x| wrap(x + inc)).collect());
            distances.push(vec![u32::MAX; risk[i].len()]);
        }
    }

    distances[0][0] = 0;

    find_shortest_path(&mut risk, &mut distances);

    println!("{}", distances.last().unwrap().last().unwrap());
}

fn find_shortest_path(risk: &mut Vec<Vec<u32>>, distances: &mut Vec<Vec<u32>>) {
    let mut priority_queue: DoublePriorityQueue<(usize, usize), u32> = DoublePriorityQueue::new();
    priority_queue.push((0, 0), 0);

    while !priority_queue.is_empty() {
        let (u, _) = priority_queue.pop_min().unwrap();
        let (x, y) = u;

        if x > 0 {
            let new_dist = distances[x][y] + risk[x - 1][y];
            if new_dist < distances[x - 1][y] {
                distances[x - 1][y] = new_dist;
                priority_queue.push((x - 1, y), new_dist);
            }
        }
        if x < risk.len() - 1 {
            let new_dist = distances[x][y] + risk[x + 1][y];
            if new_dist < distances[x + 1][y] {
                distances[x + 1][y] = new_dist;
                priority_queue.push((x + 1, y), new_dist);
            }
        }
        if y > 0 {
            let new_dist = distances[x][y] + risk[x][y - 1];
            if new_dist < distances[x][y - 1] {
                distances[x][y - 1] = new_dist;
                priority_queue.push((x, y - 1), new_dist);
            }
        }
        if y < risk[x].len() - 1 {
            let new_dist = distances[x][y] + risk[x][y + 1];
            if new_dist < distances[x][y + 1] {
                distances[x][y + 1] = new_dist;
                priority_queue.push((x, y + 1), new_dist);
            }
        }
    }
}

fn wrap(i: u32) -> u32 {
    if i > 9 {
        return i - 9;
    }
    return i;
}
