use std::collections::VecDeque;

use itertools::Itertools;

use crate::io::read_lines;

pub(crate) fn day18() {
    let mut numbers: Vec<Vec<SnailfishNumber>> = vec!();
    let lines = read_lines("input/day18/input.txt").unwrap();
    for line in lines {
        numbers.push(parse(line.unwrap()))
    }

    // Part A
    let mut num_stack = VecDeque::from_iter(numbers.iter());
    let mut result = reduce(num_stack.pop_front().unwrap().to_vec());
    while !num_stack.is_empty() {
        result = reduce(add(result, num_stack.pop_front().unwrap().to_vec()));
    }
    println!("{}", magnitude(result));

    // Part B
    let max = numbers.iter().permutations(2)
        .map(|perm| add(perm[0].clone(), perm[1].clone()))
        .map(|n| magnitude(reduce(n)))
        .max().unwrap();
    println!("{}", max);
}

fn parse(s: String) -> Vec<SnailfishNumber> {
    let mut number = vec!();
    for c in s.chars() {
        match c {
            '[' => number.push(SnailfishNumber::OPEN),
            ']' => number.push(SnailfishNumber::CLOSE),
            ',' => {}
            _ => number.push(SnailfishNumber::VALUE(c.to_digit(10).unwrap()))
        }
    }
    return number;
}

fn magnitude(n: Vec<SnailfishNumber>) -> u32 {
    let mut stack = VecDeque::new();

    for x in n {
        match x {
            SnailfishNumber::VALUE(_) => stack.push_back(x),
            SnailfishNumber::OPEN => stack.push_back(x),
            SnailfishNumber::CLOSE => {
                let n2 = stack.pop_back().unwrap();
                let n1 = stack.pop_back().unwrap();
                let _ = stack.pop_back().unwrap(); // Opening brace

                let mut sum = 0;
                match n2 {
                    SnailfishNumber::VALUE(val) => sum += 2 * val,
                    _ => panic!("unexpected n2")
                }
                match n1 {
                    SnailfishNumber::VALUE(val) => sum += 3 * val,
                    _ => panic!("unexpected n1")
                }

                stack.push_back(SnailfishNumber::VALUE(sum));
            }
        }
    }

    let last_item = stack.pop_back();
    return match last_item {
        Some(SnailfishNumber::VALUE(res)) => res,
        _ => panic!("Unexpected value in stack {:?}", last_item)
    };
}

fn add(mut left: Vec<SnailfishNumber>, mut right: Vec<SnailfishNumber>) -> Vec<SnailfishNumber> {
    let mut res = vec!();
    res.push(SnailfishNumber::OPEN);
    res.append(&mut left);
    res.append(&mut right);
    res.push(SnailfishNumber::CLOSE);
    return res;
}

fn reduce(n: Vec<SnailfishNumber>) -> Vec<SnailfishNumber> {
    let mut res = n.clone();
    loop {
        if let Some(r) = explode(res.clone()) {
            res = r;
            continue;
        }
        if let Some(r) = split(res.clone()) {
            res = r;
            continue;
        }
        break;
    }
    return res;
}

fn explode(n: Vec<SnailfishNumber>) -> Option<Vec<SnailfishNumber>> {
    let mut depth = 0;
    let mut break_at = 0;

    for x in &n {
        match x {
            SnailfishNumber::CLOSE => { depth = depth - 1 }
            SnailfishNumber::OPEN => { depth = depth + 1 }
            _ => {}
        }
        if depth == 5 {
            return Some(explode_op(n, break_at));
        }
        break_at += 1;
    }
    return None;
}

fn explode_op(mut n: Vec<SnailfishNumber>, break_at: usize) -> Vec<SnailfishNumber> {
    let left_n = n[break_at + 1].val();
    let right_n = n[break_at + 2].val();

    // The pair's left value is added to the first regular number to the left of the exploding pair (if any)
    for i in (0..break_at).rev() {
        match n[i] {
            SnailfishNumber::VALUE(val) => {
                n[i] = SnailfishNumber::VALUE(val + left_n);
                break;
            }
            _ => {}
        }
    }

    // The pair's right value is added to the first regular number to the right of the exploding pair (if any)
    for i in break_at + 4..n.len() {
        match n[i] {
            SnailfishNumber::VALUE(val) => {
                n[i] = SnailfishNumber::VALUE(val + right_n);
                break;
            }
            _ => {}
        }
    }

    // Then, the entire exploding pair is replaced with the regular number 0
    let slice = [SnailfishNumber::VALUE(0)];
    n.splice(break_at..break_at + 4, slice.into_iter());

    return n;
}

fn split(n: Vec<SnailfishNumber>) -> Option<Vec<SnailfishNumber>> {
    let mut break_at = 0;
    for x in &n {
        match x {
            SnailfishNumber::VALUE(v) => {
                if *v >= 10 {
                    break;
                }
            }
            _ => {}
        }
        break_at += 1;
    }
    if break_at == n.len() {
        return None;
    }

    let num = n[break_at].val();
    let mut left_part = n[0..break_at].to_vec();
    left_part.push(SnailfishNumber::OPEN);
    left_part.push(SnailfishNumber::VALUE(num / 2));
    left_part.push(SnailfishNumber::VALUE(if num % 2 == 0 { num / 2 } else { num / 2 + 1 }));
    left_part.push(SnailfishNumber::CLOSE);
    left_part.extend_from_slice(n[break_at + 1..n.len()].to_vec().as_slice());
    return Some(left_part);
}

#[derive(Copy, Clone, Debug)]
enum SnailfishNumber {
    VALUE(u32),
    OPEN,
    CLOSE,
}

impl SnailfishNumber {
    fn val(self) -> u32 {
        if let SnailfishNumber::VALUE(v) = self { v } else { panic!("Not a VALUE") }
    }
}
