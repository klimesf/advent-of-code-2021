use std::collections::HashMap;

pub(crate) fn day21() {
    part_a();
    part_b();
}

fn part_a() {
    let mut player_1_pos: u64 = 5 - 1;
    let mut player_2_pos: u64 = 8 - 1;
    let mut player_1_score: u64 = 0;
    let mut player_2_score: u64 = 0;
    let mut dice = 0;
    let mut player_1_turn = true;
    let mut total_rolls = 0;

    while player_1_score < 1000 && player_2_score < 1000 {
        if player_1_turn {
            let (roll1, roll2, roll3, new_dice) = turn(dice);
            total_rolls += 3;
            dice = new_dice;

            player_1_pos = (player_1_pos + roll1) % 10;
            player_1_pos = (player_1_pos + roll2) % 10;
            player_1_pos = (player_1_pos + roll3) % 10;
            player_1_score += player_1_pos + 1;
            player_1_turn = false;
        } else {
            let (roll1, roll2, roll3, new_dice) = turn(dice);
            total_rolls += 3;
            dice = new_dice;

            player_2_pos = (player_2_pos + roll1) % 10;
            player_2_pos = (player_2_pos + roll2) % 10;
            player_2_pos = (player_2_pos + roll3) % 10;
            player_2_score += player_2_pos + 1;
            player_1_turn = true;
        }
    }

    let losing = if player_1_score > player_2_score { player_2_score } else { player_1_score };
    println!("{} * {} = {}", losing, total_rolls, losing * total_rolls);
}

fn turn(mut dice: u64) -> (u64, u64, u64, u64) {
    dice = roll(dice);
    let roll1 = dice;
    dice = roll(dice);
    let roll2 = dice;
    dice = roll(dice);
    let roll3 = dice;
    return (roll1, roll2, roll3, dice);
}

fn roll(dice: u64) -> u64 {
    return (dice + 1) % 100;
}

fn part_b() {
    // Keep map with game situations and their count
    // (p1 pos, p1 score), (p2 pos, p2 score) => situation counter
    let mut states: HashMap<((u8, u8), (u8, u8)), u128> = HashMap::new();
    let mut p1_wins: u128 = 0;
    let mut p2_wins: u128 = 0;

    // There are 27 possible rolls with the dice for 1 player turn
    let mut possible_rolls: Vec<u8> = vec!();
    for i in 1..=3 {
        for j in 1..=3 {
            for k in 1..=3 {
                possible_rolls.push(i + j + k);
            }
        }
    }

    // Starting position.. since the board is numbered 1-10, but we use 0-9, subtract 1
    states.insert(((5 - 1, 0), (8 - 1, 0)), 1);

    let mut rounds_ctr = 0;
    while !states.is_empty() || !states.is_empty() {
        // P1 turn
        let mut new_states = HashMap::new();
        for (((p1_pos, p1_score), (p2_pos, p2_score)), counter) in states {
            for p1_roll in &possible_rolls {
                let new_p1_pos = (p1_pos + *p1_roll) % 10;
                let new_p1_score = p1_score + new_p1_pos as u8 + 1; // Add 1 because of the board numbering
                if new_p1_score >= 21 {
                    // If p1 wins in these situations, add them to the pile and terminate the exploration
                    p1_wins += counter;
                    continue;
                }

                for p2_roll in &possible_rolls {
                    let new_p2_pos = (p2_pos + *p2_roll) % 10;
                    let new_p2_score = p2_score + new_p2_pos as u8 + 1; // Add 1 because of the board numbering
                    if new_p2_score >= 21 {
                        // If p2 wins in these situations, add them to the pile and terminate the exploration
                        p2_wins += counter;
                        continue;
                    }

                    *new_states.entry(((new_p1_pos, new_p1_score), (new_p2_pos, new_p2_score))).or_insert(0) += counter;
                }
            }
        }
        states = new_states;

        rounds_ctr += 1;
        println!("{}", rounds_ctr);
    }

    println!("{} {} ({})", p1_wins, p2_wins, p1_wins.max(p2_wins))
}
