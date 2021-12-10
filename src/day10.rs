use crate::io::read_lines;

pub(crate) fn day10() {
    let filename = "input/10.txt";

    let mut codes: Vec<String> = vec!();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                codes.push(ip)
            }
        }
    }

    let mut score_a = 0;
    let mut scores_b: Vec<u64> = vec!();
    for line in codes {
        let (part_a, part_b) = evaluate(line);
        score_a += part_a;
        if part_b != 0 {
            scores_b.push(part_b);
        }
    }
    scores_b.sort();
    println!("{}", score_a);
    println!("{}", scores_b[(scores_b.len() / 2)]);
}

fn evaluate(s: String) -> (u32, u64) {
    let mut stack: Vec<char> = vec!();
    for c in s.chars() {
        match c {
            '(' => stack.push(c),
            '[' => stack.push(c),
            '{' => stack.push(c),
            '<' => stack.push(c),
            ')' => {
                let cpr = stack.pop().unwrap();
                if cpr != '(' {
                    return (3, 0);
                }
            }
            ']' => {
                let cpr = stack.pop().unwrap();
                if cpr != '[' {
                    return (57, 0);
                }
            }
            '}' => {
                let cpr = stack.pop().unwrap();
                if cpr != '{' {
                    return (1197, 0);
                }
            }
            '>' => {
                let cpr = stack.pop().unwrap();
                if cpr != '<' {
                    return (25137, 0);
                }
            }
            _ => {}
        }
    }

    let mut score: u64 = 0;
    stack.reverse();
    for rest in stack {
        match rest {
            '(' => score = score * 5 + 1,
            '[' => score = score * 5 + 2,
            '{' => score = score * 5 + 3,
            '<' => score = score * 5 + 4,
            _ => {}
        }
    }

    return (0, score);
}
