use crate::io::read_lines;

pub(crate) fn day07() {
    let filename = "input/day07/input.txt";

    if let Ok(mut lines) = read_lines(filename) {
        let mut numbers: Vec<i32> = lines.next().unwrap().unwrap()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        part_a(&mut numbers);
        part_b(&mut numbers);
    }
}

fn part_a(numbers: &mut Vec<i32>) {
    numbers.sort();

    let median = numbers[numbers.len() / 2];

    let sum: i32 = numbers.iter()
        .map(|n| n - median)
        .map(|n| n.abs())
        .sum();

    println!("{}: {}", median, sum);
}

fn part_b(numbers: &mut Vec<i32>) {
    let mut min_dist = i32::MAX;
    let mut min_pos = 0;

    let max = *numbers.iter().max().unwrap();

    for i in 0..=max {
        let dist = numbers.iter()
            .map(|n| n - i)
            .map(|n| n.abs())
            .map(|n| (n * (1 + n)) / 2)
            .sum();

        if dist < min_dist {
            min_dist = dist;
            min_pos = i;
        }
    }

    println!("{}: {}", min_pos, min_dist);
}
