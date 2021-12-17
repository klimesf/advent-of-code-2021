pub(crate) fn day17() {
    let (x_min, x_max, y_min, y_max) = (201, 230, -99 as i32, -65);

    let mut max = i32::MIN;
    let mut hit_counter = 0;

    for x in (0..=x_max).rev() {
        for y in y_min..=y_min.abs() {
            let (hits, local_max) = simulate(x_min, x_max, y_min, y_max, (x, y));
            max = local_max.max(max);
            hit_counter += if hits { 1 } else { 0 };
        }
    }
    println!("{}, {}", max, hit_counter);
}

fn simulate(x_min: i32, x_max: i32, y_min: i32, y_max: i32, mut velocity: (i32, i32)) -> (bool, i32) {
    let mut pos = (0, 0);
    let mut max_y = i32::MIN;

    while pos.0 <= x_max && pos.1 >= y_min {
        pos = (pos.0 + velocity.0, pos.1 + velocity.1);
        velocity = (
            if velocity.0 > 0 { velocity.0 - 1 } else if velocity.0 < 0 { velocity.0 + 1 } else { 0 },
            velocity.1 - 1
        );
        max_y = pos.1.max(max_y);
        if pos.0 >= x_min && pos.0 <= x_max && pos.1 >= y_min && pos.1 <= y_max {
            return (true, max_y);
        }
    }

    return (false, i32::MIN);
}
