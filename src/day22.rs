use std::collections::{HashSet, VecDeque};

use regex::{Match, Regex};

use crate::io::read_lines;

pub(crate) fn day22() {
    let lines = read_lines("input/day22/input.txt").unwrap();
    let re = Regex::new(r"^(on|off) x=([\-0-9]+)..([\-0-9]+),y=([\-0-9]+)..([\-0-9]+),z=([\-0-9]+)..([\-0-9]+)$").unwrap();
    let mut commands = VecDeque::new();
    for line in lines.into_iter().map(|l| l.unwrap()) {
        let g = re.captures(line.as_str()).unwrap();
        let command = (
            g.get(1).map_or("", |m| m.as_str()).to_string(),
            parse_i128(g.get(2)),
            parse_i128(g.get(3)),
            parse_i128(g.get(4)),
            parse_i128(g.get(5)),
            parse_i128(g.get(6)),
            parse_i128(g.get(7))
        );
        commands.push_back(command);
    }

    part_a(commands.clone());
    part_b(commands.clone());
}

fn parse_i128(g: Option<Match>) -> i128 {
    return g.map_or(0, |m| m.as_str().parse().unwrap());
}

fn part_a(commands: VecDeque<(String, i128, i128, i128, i128, i128, i128)>) {
    let mut cubes: HashSet<(i128, i128, i128)> = HashSet::new();
    for cmd in commands {
        for x in cmd.1.max(-50)..=cmd.2.min(50) {
            for y in cmd.3.max(-50)..=cmd.4.min(50) {
                for z in cmd.5.max(-50)..=cmd.6.min(50) {
                    if cmd.0 == "on" {
                        cubes.insert((x, y, z));
                    } else {
                        cubes.remove(&(x, y, z));
                    }
                }
            }
        }
    }
    println!("{}", cubes.len());
}

fn part_b(mut commands: VecDeque<(String, i128, i128, i128, i128, i128, i128)>) {
    // Result contains only non-conflicting cubes
    let mut ans: Vec<(String, i128, i128, i128, i128, i128, i128)> = vec!();

    // Iterate over the commands and look for intersections within the result
    while !commands.is_empty() {
        let cube = commands.pop_front().unwrap();
        let mut next_ans = vec!();
        let mut intersections_found = false;
        for other_cube in ans {
            // Intersection
            let min_x = cube.1.max(other_cube.1);
            let min_y = cube.3.max(other_cube.3);
            let min_z = cube.5.max(other_cube.5);
            let max_x = cube.2.min(other_cube.2);
            let max_y = cube.4.min(other_cube.4);
            let max_z = cube.6.min(other_cube.6);

            // No overlap found between cubes
            if max_x - min_x < 0 || max_y - min_y < 0 || max_z - min_z < 0 {
                next_ans.push(other_cube); // Keep the non-conflicting cube
                continue;
            }
            intersections_found = true;

            // Add the original cube for re-evaluation
            if cube.0 == "on" {
                commands.push_front(cube.clone());
            }

            // Explode the original non-conflicting cube into potentially 27 subcubes
            // and only add the parts that are not overlapping with the evaluated cube
            for x in [(other_cube.1, min_x - 1), (min_x, max_x), (max_x + 1, other_cube.2)] {
                if x.1 - x.0 < 0 { continue; }
                for y in [(other_cube.3, min_y - 1), (min_y, max_y), (max_y + 1, other_cube.4)] {
                    if y.1 - y.0 < 0 { continue; }
                    for z in [(other_cube.5, min_z - 1), (min_z, max_z), (max_z + 1, other_cube.6)] {
                        if z.1 - z.0 < 0 { continue; }
                        if x.0 == min_x && y.0 == min_y && z.0 == min_z { continue; }
                        next_ans.push(("on".to_string(), x.0, x.1, y.0, y.1, z.0, z.1));
                    }
                }
            }
        }

        // Once no intersections are found, we can add the cube
        if !intersections_found && cube.0 == "on" {
            next_ans.push(cube);
        }

        ans = next_ans;
    }

    let total_volume: i128 = ans.iter()
        // Do not forget to add +1 as we are dealing with discrete closed intervals
        .map(|c| (c.2 - c.1 + 1) * (c.4 - c.3 + 1) * (c.6 - c.5 + 1))
        .sum();
    println!("{}", total_volume);
}
