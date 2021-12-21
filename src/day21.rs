use cached::proc_macro::cached;

pub(crate) fn day21() {
    part_a((5, 8));
    part_b((5, 8));
}

fn part_a(starting_pos: (u64, u64)) {
    let mut p1_pos: u64 = starting_pos.0 - 1;
    let mut p2_pos: u64 = starting_pos.1 - 1;
    let mut p1_score: u64 = 0;
    let mut p2_score: u64 = 0;
    let mut die = 1;
    let mut total_rolls = 0;

    loop {
        let (p1_roll, new_die) = roll_deterministic(die);
        die = new_die;
        total_rolls += 3;
        p1_pos = (p1_pos + p1_roll) % 10;
        p1_score += p1_pos as u64 + 1;
        if p1_score >= 1000 {
            break;
        }

        let (p2_roll, new_die) = roll_deterministic(die);
        die = new_die;
        total_rolls += 3;
        p2_pos = (p2_pos + p2_roll) % 10;
        p2_score += p2_pos as u64 + 1;
        if p2_score >= 1000 {
            break;
        }
    }

    let losing = p1_score.min(p2_score);
    println!("{} * {} = {}", losing, total_rolls, losing * total_rolls);
}

fn roll_deterministic(mut die: u64) -> (u64, u64) {
    let mut total: u64 = 0;
    for _ in 0..3 {
        total += die as u64;
        die = (die + 1) % 100;
    }
    return (total, die);
}

fn part_b(starting_pos: (u8, u8)) {
    let (p1_wins, p2_wins) = play(starting_pos.0 - 1, 0, starting_pos.1 - 1, 0);
    if p1_wins > p2_wins {
        println!("_{}_ {}", p1_wins, p2_wins);
    } else {
        println!("{} _{}_", p1_wins, p2_wins);
    }
}

#[cached]
fn play(p1_pos: u8, p1_score: u8, p2_pos: u8, p2_score: u8) -> (u128, u128) {
    if p1_score >= 21 { return (1, 0); }
    if p2_score >= 21 { return (0, 1); }

    let mut possible_rolls: Vec<u8> = vec!();
    for i in 1..=3 { for j in 1..=3 { for k in 1..=3 { possible_rolls.push(i + j + k) } } }

    let mut p1_wins = 0;
    let mut p2_wins = 0;
    for p1_roll in &possible_rolls {
        let new_p1_pos = (p1_pos + *p1_roll) % 10;
        // Swap p1 <-> p2, meaning they change turns
        let (p2w, p1w) = play(
            p2_pos,
            p2_score,
            new_p1_pos,
            p1_score + new_p1_pos + 1,
        );
        p1_wins += p1w;
        p2_wins += p2w;
    }

    return (p1_wins, p2_wins);
}
