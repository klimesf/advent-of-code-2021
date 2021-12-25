use crate::io::read_lines;

pub(crate) fn day25() {
    let lines = read_lines("input/day25/input.txt").unwrap();
    let mut map = vec!();
    for line in lines.into_iter().map(|l| l.unwrap()) {
        let row: Vec<char> = line.chars().collect();
        map.push(row);
    }
    print(&map);

    let mut step_ctr = 0;
    loop {
        if !step(&mut map) {
            break;
        }
        step_ctr += 1;
        print(&map);
    }
    println!("{}", step_ctr + 1);
}

fn step(map: &mut Vec<Vec<char>>) -> bool {
    let mut move_ctr = 0;
    let mut snapshot = map.clone();

    // East facing
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            match snapshot[y][x] {
                '>' => {
                    let new_coords = wrap(map, y, x + 1);
                    if snapshot[new_coords.0][new_coords.1] == '.' {
                        map[new_coords.0][new_coords.1] = '>';
                        map[y][x] = '.';
                        move_ctr += 1;
                    }
                },
                _ => {},
            }
        }
    }

    snapshot = map.clone();

    // South facing
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            match snapshot[y][x] {
                'v' => {
                    let new_coords = wrap(map, y + 1, x);
                    if snapshot[new_coords.0][new_coords.1] == '.' {
                        map[new_coords.0][new_coords.1] = 'v';
                        map[y][x] = '.';
                        move_ctr += 1;
                    }
                },
                _ => {},
            }
        }
    }

    return move_ctr > 0;
}

fn wrap(map: &mut Vec<Vec<char>>, mut y: usize, mut x: usize) -> (usize, usize) {
    if y >= map.len() {
        y = 0;
    }
    if x >= map[y].len() {
        x = 0;
    }
    return (y, x);
}

fn print(map: &Vec<Vec<char>>) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            print!("{}", map[y][x]);
        }
        println!();
    }
    println!();
}
