use std::collections::{HashSet, VecDeque};
use std::iter::FromIterator;

use itertools::Itertools;

use crate::io::read_lines;

type Point = (i32, i32, i32);
type Scan = Vec<Point>;

// Not all the ideas are mine, inspired heavily by solutions on reddit

pub(crate) fn day19() {
    let mut scans: VecDeque<Scan> = VecDeque::new();
    let mut scan: Scan = vec!();
    let lines = read_lines("input/day19/test.txt").unwrap();
    for line_result in lines {
        let line = line_result.unwrap();

        if line.starts_with("---") {
            continue;
        }
        if line.is_empty() {
            scans.push_back(scan);
            scan = vec!();
            continue;
        }

        let mut parts = line.split(",");
        let x: i32 = parts.next().unwrap().parse().unwrap();
        let y: i32 = parts.next().unwrap().parse().unwrap();
        let z: i32 = parts.next().unwrap().parse().unwrap();
        scan.push((x, y, z));
    }
    scans.push_back(scan); // Do not forget to add the last one

    // Try to merge the first scan with the others, until only one remains
    let mut scanner_positions: Scan = vec!();
    let mut final_scan = scans.pop_front().unwrap();
    while scans.len() > 0 {
        let other = scans.pop_front().unwrap();
        let mut matched = false;
        for other_r in scan_rotations(&other) {
            if let Some(scanner_position) = merge(&mut final_scan, &other_r) {
                scanner_positions.push(scanner_position);
                matched = true;
                break;
            }
        }
        if !matched { scans.push_back(other); } // No match, return to the back
    }
    println!("{}", final_scan.len());

    let max_distance = scanner_positions.into_iter().combinations(2)
        .map(|v| {
            let (x1, y1, z1) = v[0];
            let (x2, y2, z2) = v[1];
            return i32::abs(x1 - x2) + i32::abs(y1 - y2) + i32::abs(z1 - z2);
        })
        .max().unwrap();
    println!("{}", max_distance);
}

fn point_rotations(p: Point) -> Scan {
    let mut res = vec!();
    let (x, y, z) = p;

    // @formatter:off
    res.push(( x,  y,  z));
    res.push(( y,  z,  x));
    res.push(( z,  x,  y));
    res.push(( z,  y, -x));
    res.push(( y,  x, -z));
    res.push(( x,  z, -y));

    res.push(( x, -y, -z));
    res.push(( y, -z, -x));
    res.push(( z, -x, -y));
    res.push(( z, -y,  x));
    res.push(( y, -x,  z));
    res.push(( x, -z,  y));

    res.push((-x,  y, -z));
    res.push((-y,  z, -x));
    res.push((-z,  x, -y));
    res.push((-z,  y,  x));
    res.push((-y,  x,  z));
    res.push((-x,  z,  y));

    res.push((-x, -y,  z));
    res.push((-y, -z,  x));
    res.push((-z, -x,  y));
    res.push((-z, -y, -x));
    res.push((-y, -x, -z));
    res.push((-x, -z, -y));
    // @formatter:on

    return res;
}

fn scan_rotations(scan: &Scan) -> Vec<Scan> {
    let mut res = vec![Vec::new(); 24];
    for p in scan {
        for (i, x) in point_rotations(*p).iter().enumerate() {
            res[i].push(*x);
        }
    }
    return res;
}

fn merge(first: &mut Scan, other: &Scan) -> Option<Point> {
    // Assuming point P is included in both scan 1 and scan 2, but having different relative coords:
    // If you have vector from S1 -> P and then S2 -> P, you can subtract S2 -> P and you get the position of the
    // scanner 2 relatively to the scanner 1
    let mut possible_scanner_positions: Vec<Point> = vec!();
    for p1 in first.as_slice() {
        for p2 in other.as_slice() {
            possible_scanner_positions.push((p1.0 - p2.0, p1.1 - p2.1, p1.2 - p2.2));
        }
    }

    // Iterate over possible scanner positions, translate the points of other scan by the vector S1 -> S2
    // and find out if at least 12 points overlap. If yes, we can merge and return early.
    let first_set: HashSet<Point> = HashSet::from_iter(first.iter().cloned()); // For better perf
    for scanner_pos in possible_scanner_positions {
        let translated = other.iter().map(|(x, y, z)| (x + scanner_pos.0, y + scanner_pos.1, z + scanner_pos.2));

        // Count translated that overlap
        let mut overlap_counter = 0;
        for x in translated.clone() {
            if first_set.contains(&x) {
                overlap_counter += 1;
            }
        }
        if overlap_counter < 12 { continue; } // No match, next!

        translated
            .filter(|p| !first_set.contains(p))
            .for_each(|p| first.push(p));

        return Some(scanner_pos);
    }

    // We didn't find any overlapping scans, therefore we cannot determine the scanner position
    return None;
}
