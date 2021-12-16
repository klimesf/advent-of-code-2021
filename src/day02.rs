use crate::io::read_lines;

pub fn day02() {
    let filename = "input/day02/input.txt";

    let mut commands: Vec<String> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                commands.push(ip.parse().unwrap())
            }
        }
    }

    part_a(commands.as_slice());
    part_b(commands.as_slice());
}

fn part_a(commands: &[String]) {
    let mut distance: i32 = 0;
    let mut depth: i32 = 0;

    for x in commands {
        let s = x.split_whitespace();
        let cmd = s.collect::<Vec<&str>>();
        if cmd[0] == "forward" {
            distance += cmd[1].parse::<i32>().unwrap();
        }
        if cmd[0] == "down" {
            depth += cmd[1].parse::<i32>().unwrap();
        }
        if cmd[0] == "up" {
            depth -= cmd[1].parse::<i32>().unwrap();
        }
    }

    println!("{}", distance * depth)
}

fn part_b(commands: &[String]) {
    let mut distance: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;

    for x in commands {
        let s = x.split_whitespace();
        let cmd = s.collect::<Vec<&str>>();
        if cmd[0] == "forward" {
            let x = cmd[1].parse::<i32>().unwrap();
            distance += x;
            depth += aim * x
        }
        if cmd[0] == "down" {
            aim += cmd[1].parse::<i32>().unwrap();
        }
        if cmd[0] == "up" {
            aim -= cmd[1].parse::<i32>().unwrap();
        }
    }

    println!("{}", distance * depth)
}
