use crate::io::read_lines;

fn day01() {
    let filename = "input/01.txt";

    let mut numbers: Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                numbers.push(ip.parse().unwrap())
            }
        }
    }

    part_a(numbers.as_slice());
    part_b(numbers.as_slice());
}

fn part_a(numbers: &[i32]) {
    let mut cnt = 0;
    let mut prev = i32::MAX;
    for x in numbers {
        if *x > prev {
            cnt += 1;
        }
        prev = *x;
    }
    println!("{}", cnt)
}

fn part_b(numbers: &[i32]) {
    let mut cnt = 0;
    let mut prev = i32::MAX;
    for i in 0..(numbers.len() - 2) {
        let sum = numbers[i] + numbers[i + 1] + numbers[i + 2];
        if sum > prev {
            cnt += 1;
        }
        prev = sum;
    }

    println!("{}", cnt)
}
