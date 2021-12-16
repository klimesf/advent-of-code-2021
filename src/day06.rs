use crate::io::read_lines;

pub(crate) fn day06() {
    let filename = "input/day06/input.txt";

    if let Ok(mut lines) = read_lines(filename) {
        let numbers: Vec<usize> = lines.next().unwrap().unwrap()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        let mut counter: [u64; 9] = [0; 9];
        for number in numbers {
            counter[number] += 1;
        }

        for _ in 0..256 {
            let mut new_counter: [u64; 9] = [0; 9];
            new_counter[0] = counter[1];
            new_counter[1] = counter[2];
            new_counter[2] = counter[3];
            new_counter[3] = counter[4];
            new_counter[4] = counter[5];
            new_counter[5] = counter[6];
            new_counter[6] = counter[7] + counter[0];
            new_counter[7] = counter[8];
            new_counter[8] = counter[0];

            counter = new_counter
        }

        println!("{}", counter.iter().sum::<u64>());
    }
}
