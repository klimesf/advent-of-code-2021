use crate::io::read_lines;
use std::collections::HashSet;

pub(crate) fn day04() {
    if let Ok(mut lines) = read_lines("input/04.txt") {
        let numbers: Vec<i32> = lines.next().unwrap().unwrap()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        lines.next(); // Skip empty line

        let mut boards: Vec<BingoBoard> = vec!();

        let mut rows = vec!();
        let mut i = 0;
        for line in lines {
            if line.as_ref().unwrap().is_empty() {
                boards.push(bingo(rows.clone(), i));
                rows.clear();
                i += 1;
                continue;
            }

            let row = line.as_ref().unwrap().trim().split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            rows.push(row);
        }
        boards.push(bingo(rows.clone(), i)); // Push the final board

        let mut completed: HashSet<i32> = HashSet::new();
        let boards_len = boards.len();

        for number in numbers {
            for board in &mut boards {
                board.mark(number);
                let is_complete = board.is_complete();
                if is_complete {
                    completed.insert(board.number);
                    println!("{} * {} = {}, {}", number, board.sum_unmarked(), number * board.sum_unmarked(), completed.len());
                    if completed.len() == boards_len {
                        println!("---");
                        return;
                    }
                }
            }
        }
    }
}

struct BingoBoard {
    rows: Vec<Vec<i32>>,
    number: i32,
}

impl BingoBoard {
    fn sum_unmarked(&mut self) -> i32 {
        return self.rows.iter()
            .flat_map(|row| row)
            .filter(|num| **num >= 0)
            .sum();
    }

    fn mark(&mut self, number: i32) {
        for i in 0..5 {
            for j in 0..5 {
                if self.rows[i][j] == number {
                    self.rows[i][j] = -1;
                }
            }
        }
    }

    fn is_complete(&mut self) -> bool {
        for i in 0..5 {
            let mut marked = 0;
            for j in 0..5 {
                if self.rows[i][j] < 0 {
                    marked += 1
                }
            }
            if marked == 5 {
                return true;
            }
        }
        for i in 0..5 {
            let mut marked = 0;
            for j in 0..5 {
                if self.rows[j][i] < 0 {
                    marked += 1
                }
            }
            if marked == 5 {
                return true;
            }
        }

        return false;
    }
}

fn bingo(rows: Vec<Vec<i32>>, number: i32) -> BingoBoard {
    BingoBoard {
        rows,
        number
    }
}

