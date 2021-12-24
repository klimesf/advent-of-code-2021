use std::collections::{HashMap, VecDeque};

use cached::proc_macro::cached;

use crate::io::read_lines;

pub(crate) fn day24() {
    let constants = [
        [1, 11, 5],
        [1, 13, 5],
        [1, 12, 1],
        [1, 15, 15],
        [1, 10, 2],
        [26, -1, 2],
        [1, 14, 5],
        [26, -8, 8],
        [26, -7, 14],
        [26, -8, 12],
        [1, 11, 7],
        [26, -2, 14],
        [26, -2, 13],
        [26, -13, 6],
    ];

    let mut z_map: HashMap<i64, u128> = HashMap::new();
    z_map.insert(0, 0);

    // Part A
    for i in 0..constants.len() {
        let mut new_z_map: HashMap<i64, u128> = HashMap::new();
        for w in 1..=9 {
            for (z, max) in &z_map {
                let new_z = step(w, *z, constants[i][0], constants[i][1], constants[i][2]);
                let new_max = max * 10 + w as u128;
                let old_max = *new_z_map.get(&new_z).unwrap_or(&0);
                if new_max > old_max {
                    new_z_map.insert(new_z, new_max);
                }
            }
        }
        println!("{}", new_z_map.len());
        z_map = new_z_map;
    }

    println!("{}", z_map.get(&0).unwrap());

    // Part B
    for i in 0..constants.len() {
        let mut new_z_map: HashMap<i64, u128> = HashMap::new();
        for w in 1..=9 {
            for (z, min) in &z_map {
                let new_z = step(w, *z, constants[i][0], constants[i][1], constants[i][2]);
                let new_min = min * 10 + w as u128;
                let old_max = *new_z_map.get(&new_z).unwrap_or(&u128::MAX);
                if new_min < old_max {
                    new_z_map.insert(new_z, new_min);
                }
            }
        }
        println!("{}", new_z_map.len());
        z_map = new_z_map;
    }

    println!("{}", z_map.get(&0).unwrap());
}

#[cached]
fn step(w: i64, old_z: i64, c1: i64, c2: i64, c3: i64) -> i64 {
    let x = old_z % 26;
    let mut z = old_z / c1;
    let mut x = x + c2;
    x = if x != w { 1 } else { 0 };
    let mut y = (25 * x) + 1;
    z = z * y;
    y = (w + c3) * x;
    z = z + y;
    return z;
}
