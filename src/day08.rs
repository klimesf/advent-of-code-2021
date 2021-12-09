use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::io::read_lines;

struct Measurement {
    patterns: Vec<String>,
    output: Vec<String>,
}

pub(crate) fn day08() {
    let filename = "input/08.txt";

    let mut measurements: Vec<Measurement> = vec!();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            let s = line.unwrap();
            let mut parts = s.split(" | ");

            measurements.push(Measurement {
                patterns: parts.next().unwrap()
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect(),
                output: parts.next().unwrap()
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect(),
            })
        }
    }

    part_a(&mut measurements);
    part_b(&mut measurements);
}

fn part_a(measurements: &mut Vec<Measurement>) {
    let mut cntr: u32 = 0;
    for measurement in measurements {
        for digit in &measurement.output {
            let digit_len = digit.len();
            match digit_len {
                2 => cntr += 1,
                3 => cntr += 1,
                4 => cntr += 1,
                7 => cntr += 1,
                _ => {}
            }
        }
    }

    println!("{}", cntr)
}

fn part_b(measurements: &mut Vec<Measurement>) {
    let mut numbers: Vec<u32> = vec!();
    for measurement in measurements {
        numbers.push(solve(measurement))
    }
    println!("{}", numbers.iter().sum::<u32>())
}

fn solve(measurement: &mut Measurement) -> u32 {
    let items = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];

    let mut pattern_map = HashMap::new();
    pattern_map.insert(1110111, 0);
    pattern_map.insert(0010010, 1);
    pattern_map.insert(1011101, 2);
    pattern_map.insert(1011011, 3);
    pattern_map.insert(0111010, 4);
    pattern_map.insert(1101011, 5);
    pattern_map.insert(1101111, 6);
    pattern_map.insert(1010010, 7);
    pattern_map.insert(1111111, 8);
    pattern_map.insert(1111011, 9);

    let required: HashSet<_> = pattern_map.keys().cloned().collect();

    // Permutate all the variations and find the permutation which constitues all required patterns
    for perm in items.iter().permutations(items.len()).unique() {
        let mut char_map = HashMap::new();
        char_map.insert(*perm[0], 1000000);
        char_map.insert(*perm[1], 100000);
        char_map.insert(*perm[2], 10000);
        char_map.insert(*perm[3], 1000);
        char_map.insert(*perm[4], 100);
        char_map.insert(*perm[5], 10);
        char_map.insert(*perm[6], 1);

        let mut ctr = 0;
        for digit in &measurement.patterns {
            let mut num = 0;
            for c in digit.chars() {
                num += char_map.get(&c).unwrap();
            }
            if required.contains(&num) {
                ctr += 1;
            } else {
                break;
            }
        }

        // If all patterns are present, decipher the measurement output and return
        if ctr == 10 {
            let mut i = 1000;
            let mut output = 0;
            for o in &measurement.output {
                let mut total = 0;
                for c in o.chars() {
                    total += char_map.get(&c).unwrap();
                }

                let digit = pattern_map.get(&total).unwrap();
                output += i * digit;
                i = i / 10;
            }
            return output;
        }
    }

    return 0;
}
