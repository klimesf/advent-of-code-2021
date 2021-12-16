use std::collections::VecDeque;

use crate::io::read_lines;

pub(crate) fn day16() {
    let filename = "input/16.txt";

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                let mut d = string_2_deque(hex_2_bin(ip));
                println!("{:?}", solve(&mut d));
            }
        }
    }
}

fn hex_2_bin(s: String) -> String {
    let mut result = String::with_capacity(s.len() * 4);
    for c in s.chars() {
        match c {
            '0' => result.push_str("0000"),
            '1' => result.push_str("0001"),
            '2' => result.push_str("0010"),
            '3' => result.push_str("0011"),
            '4' => result.push_str("0100"),
            '5' => result.push_str("0101"),
            '6' => result.push_str("0110"),
            '7' => result.push_str("0111"),
            '8' => result.push_str("1000"),
            '9' => result.push_str("1001"),
            'A' => result.push_str("1010"),
            'B' => result.push_str("1011"),
            'C' => result.push_str("1100"),
            'D' => result.push_str("1101"),
            'E' => result.push_str("1110"),
            'F' => result.push_str("1111"),
            _ => {}
        }
    }
    return result;
}

fn string_2_deque(s: String) -> VecDeque<char> {
    let mut result = VecDeque::new();
    for c in s.chars() {
        result.push_back(c);
    }
    return result;
}

fn solve(mut s: &mut VecDeque<char>) -> (u64, u64) {
    //println!("solving {:?}", s);

    let mut version_bits = String::with_capacity(3);
    version_bits.push(s.pop_front().unwrap());
    version_bits.push(s.pop_front().unwrap());
    version_bits.push(s.pop_front().unwrap());

    let mut type_id_bits = String::with_capacity(3);
    type_id_bits.push(s.pop_front().unwrap());
    type_id_bits.push(s.pop_front().unwrap());
    type_id_bits.push(s.pop_front().unwrap());

    let mut version_sum = u64::from_str_radix(&version_bits, 2).unwrap();

    match type_id_bits.as_ref() {
        "100" => { // literal
            return (version_sum, solve_literal(&mut s));
        }
        "000" => { // sum
            let (version, numbers) = solve_operator(&mut s);
            version_sum += version;
            return (version_sum, numbers.iter().sum());
        }
        "001" => { // product
            let (version, numbers) = solve_operator(&mut s);
            version_sum += version;
            return (version_sum, numbers.iter().product());
        }
        "010" => { // min
            let (version, numbers) = solve_operator(&mut s);
            version_sum += version;
            return (version_sum, *numbers.iter().min().unwrap());
        }
        "011" => { // max
            let (version, numbers) = solve_operator(&mut s);
            version_sum += version;
            return (version_sum, *numbers.iter().max().unwrap());
        }
        "101" => { // gt
            let (version, numbers) = solve_operator(&mut s);
            version_sum += version;
            return (version_sum, if numbers[0] > numbers[1] { 1 } else { 0 })

        }
        "110" => { // lt
            let (version, numbers) = solve_operator(&mut s);
            version_sum += version;
            return (version_sum, if numbers[0] < numbers[1] { 1 } else { 0 })
        }
        "111" => { // eq
            let (version, numbers) = solve_operator(&mut s);
            version_sum += version;
            return (version_sum, if numbers[0] == numbers[1] { 1 } else { 0 })
        }
        _ => panic!("Corrupted type id")
    }
}

fn solve_literal(s: &mut VecDeque<char>) -> u64 {
    let mut number_bits = String::with_capacity(100);

    loop {
        let group_header = s.pop_front().unwrap();

        number_bits.push(s.pop_front().unwrap());
        number_bits.push(s.pop_front().unwrap());
        number_bits.push(s.pop_front().unwrap());
        number_bits.push(s.pop_front().unwrap());

        if group_header == '0' {
            break;
        }
    }

    return u64::from_str_radix(&number_bits, 2).unwrap();
}

fn solve_operator(s: &mut VecDeque<char>) -> (u64, Vec<u64>) {
    let length_type_id = s.pop_front();

    match length_type_id {
        Some('0') => return solve_operator_type_0(s),
        Some('1') => return solve_operator_type_1(s),
        _ => panic!("Missing or corrupted length type bit"),
    }
}

fn solve_operator_type_0(s: &mut VecDeque<char>) -> (u64, Vec<u64>) {
    let mut length_bits = String::with_capacity(15);
    for _ in 0..15 {
        length_bits.push(s.pop_front().unwrap());
    }
    let length = usize::from_str_radix(&length_bits, 2).unwrap();

    let mut version_sum = 0;
    let mut values = vec!();
    let start = s.len();

    loop {
        let (version, value) = solve(s);
        version_sum += version;
        values.push(value);

        let now = s.len();
        if start - now >= length {
            break;
        }
    }
    return (version_sum, values);
}


fn solve_operator_type_1(s: &mut VecDeque<char>) -> (u64, Vec<u64>) {
    let mut length_bits = String::with_capacity(15);
    for _ in 0..11 {
        length_bits.push(s.pop_front().unwrap());
    }
    let length = usize::from_str_radix(&length_bits, 2).unwrap();

    let mut version_sum = 0;
    let mut values = vec!();

    for _ in 0..length {
        let (version, value) = solve(s);
        version_sum += version;
        values.push(value);
    }
    return (version_sum, values);
}
