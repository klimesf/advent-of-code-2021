use std::collections::HashMap;

use crate::io::read_lines;

pub(crate) fn day20() {
    let mut algo = "".to_string();
    let mut image: HashMap<(i32, i32), u32> = HashMap::new();

    let lines = read_lines("input/day20/input.txt").unwrap();
    let mut line_ctr = 0;
    for line_result in lines {
        let line = line_result.unwrap();
        if line.is_empty() { continue; }
        if algo.is_empty() {
            algo = line;
            continue;
        }
        for (i, c) in line.chars().enumerate() {
            image.insert((line_ctr, i as i32), char_2_int(c));
        }
        line_ctr += 1;
    }

    let mut unknown_pixel = 0;
    let mut dimensions = (-2, line_ctr + 2);
    for _ in 0..50 {
        let (new_image, new_dimensions, new_unknown_pixel) = enhance(
            algo.clone(),
            &mut image,
            dimensions,
            unknown_pixel,
        );
        image = new_image;
        dimensions = new_dimensions;
        unknown_pixel = new_unknown_pixel;
    }

    println!("{}", image.values().sum::<u32>());
}

fn char_2_int(c: char) -> u32 {
    return match c {
        '.' => 0,
        _ => 1,
    };
}

fn enhance(
    algo: String,
    input: &mut HashMap<(i32, i32), u32>,
    dimensions: (i32, i32),
    unknown_pixel: u32,
) -> (HashMap<(i32, i32), u32>, (i32, i32), u32) {
    let mut output = HashMap::new();

    for x in dimensions.0..dimensions.1 {
        for y in dimensions.0..dimensions.1 {
            let mut num = vec!();
            num.push(*input.entry((x - 1, y - 1)).or_insert(unknown_pixel));
            num.push(*input.entry((x - 1, y)).or_insert(unknown_pixel));
            num.push(*input.entry((x - 1, y + 1)).or_insert(unknown_pixel));
            num.push(*input.entry((x, y - 1)).or_insert(unknown_pixel));
            num.push(*input.entry((x, y)).or_insert(unknown_pixel));
            num.push(*input.entry((x, y + 1)).or_insert(unknown_pixel));
            num.push(*input.entry((x + 1, y - 1)).or_insert(unknown_pixel));
            num.push(*input.entry((x + 1, y)).or_insert(unknown_pixel));
            num.push(*input.entry((x + 1, y + 1)).or_insert(unknown_pixel));

            let mut binary = String::with_capacity(9);
            for n in num {
                binary.push(if n == 0 { '0' } else { '1' });
            }
            let algo_index = usize::from_str_radix(&binary, 2).unwrap();
            let new_pixel = char_2_int(algo.chars().nth(algo_index).unwrap());
            output.insert((x, y), new_pixel);
        }
    }

    // Handle infinite size of the image
    let new_unknown_pixel = if unknown_pixel == 0 { algo.chars().nth(0).unwrap() } else { algo.chars().nth(511).unwrap() };

    return (
        output,
        (dimensions.0 - 2, dimensions.1 + 2), // Grow the square by 2 each direction
        char_2_int(new_unknown_pixel)
    );
}
