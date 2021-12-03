use crate::io::read_lines;

pub fn day03() {
    let filename = "input/03.txt";

    let mut numbers: Vec<String> = Vec::new();

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

fn part_a(numbers: &[String]) {
    let half = numbers.len() as i32 / 2;

    let mut common_bits = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for number in numbers {
        for (i, digit) in number.chars().enumerate() {
            if digit == '1' {
                common_bits[i] += 1;
            }
        }
    }

    let mut gamma = ['0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0'];
    let mut epsilon = ['0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0'];
    for (i, bit) in common_bits.iter().enumerate() {
        if *bit > half {
            gamma[i] = '1';
            epsilon[i] = '0';
        } else {
            gamma[i] = '0';
            epsilon[i] = '1';
        }
    }

    let result = i32::from_str_radix(&String::from_iter(gamma), 2).unwrap()
        * i32::from_str_radix(&String::from_iter(epsilon), 2).unwrap();
    println!("{}", result);
}

fn part_b(numbers: &[String]) {
    let num_len = 12;
    let mut oxy = numbers.to_vec();
    let mut co2 = numbers.to_vec();
    for i in 0..num_len {
        oxy = filter(oxy, i, true);
    }
    for i in 0..num_len {
        co2 = filter(co2, i, false);
    }

    let result = i32::from_str_radix(&oxy[0], 2).unwrap()
        * i32::from_str_radix(&co2[0], 2).unwrap();
    println!("{}", result);
}

fn filter(numbers: Vec<String>, pos: usize, oxygen: bool) -> Vec<String> {
    if numbers.len() == 1 {
        return numbers;
    }

    // Split numbers by ones and zeroes at position
    let mut ones = Vec::new();
    let mut zeroes = Vec::new();

    for number in numbers {
        if number.chars().nth(pos).unwrap() == '1' {
            ones.push(number)
        } else {
            zeroes.push(number)
        }
    }

    // Filter out
    if ones.len() >= zeroes.len() {
        return if oxygen { ones } else { zeroes };
    } else {
        return if oxygen { zeroes } else { ones };
    }
}
