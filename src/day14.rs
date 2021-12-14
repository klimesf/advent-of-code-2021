use std::collections::HashMap;

use crate::io::read_lines;

pub(crate) fn day14() {
    let filename = "input/14.txt";

    let mut input: String = "".to_string();
    let mut rules: HashMap<(char, char), char> = HashMap::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                if ip.trim().is_empty() {
                    continue;
                }
                if input == "" {
                    input = ip;
                    continue;
                }

                let chars = ip.as_bytes();
                rules.insert((chars[0] as char, chars[1] as char), chars[6] as char);
            }
        }
    }

    let mut rule_counter: HashMap<(char, char), u64> = HashMap::new();

    // Prefill with input pattern
    let mut prev_c = ' ';
    for c in input.chars() {
        if prev_c == ' ' {
            prev_c = c;
            continue;
        }

        *rule_counter.entry((prev_c, c)).or_default() += 1;
        prev_c = c;
    }

    // On every step, take the current rules and increment the resulting ones
    for i in 0..40 {
        let mut new_rule_counter: HashMap<(char, char), u64> = HashMap::new();

        for e in &rule_counter {
            let middle = *rules.get(&(e.0.0, e.0.1)).unwrap();
            *new_rule_counter.entry((e.0.0, middle)).or_default() += e.1;
            *new_rule_counter.entry((middle, e.0.1)).or_default() += e.1;
        }

        rule_counter = new_rule_counter;

        println!("{}: {}", i, count_chars(&rule_counter, prev_c)); // prev_c is now the last_char of the input
    }
}

fn count_chars(rule_counter: &HashMap<(char, char), u64>, last_char: char) -> u64 {
    let mut counts: HashMap<char, u64> = HashMap::new();
    // Since we count only the first char in the pair, we need to adjust for the last character of input
    // since it is not included in any pair (it stays the same for every iteration)
    *counts.entry(last_char).or_default() += 1;
    for e in rule_counter {
        *counts.entry(e.0.0).or_default() += e.1;
    }

    let min = counts.iter().min_by_key(|e| e.1).unwrap().1;
    let max = counts.iter().max_by_key(|e| e.1).unwrap().1;

    return max - min;
}
