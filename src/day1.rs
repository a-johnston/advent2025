use std::vec::Vec;

pub static PARTS: super::Parts = &[
    ("Example 1", "example.txt", count_zeros),
    ("Part 1", "input.txt", count_zeros),
    ("Example 2", "example.txt", count_zero_passes),
    ("Part 2", "input.txt", count_zero_passes),
];

const START: i32 = 50;
const SIZE: i32 = 100;

fn posmod(i: i32, m: i32) -> i32 {
    return ((i % m) + m) % m;
}

fn get_dial_positions(mut pos: i32, input: String) -> Vec<(i32, i32)> {
    let mut rotations = Vec::new();
    for rot in input.trim().split('\n') {
        match rot[1..].parse::<i32>() {
            Ok(val) => {
                let change = val * (((rot[0..1].eq("R")) as i32) * 2 - 1);
                pos = posmod(pos + change, SIZE);
                rotations.push((change, pos));
            }
            Err(_) => println!("Invalid rotation: {}", rot),
        }
    }
    return rotations;
}

pub fn count_zeros(input: String) -> String {
    return get_dial_positions(START, input)
        .iter()
        .filter(|(_, pos)| *pos == 0)
        .count()
        .to_string();
}

pub fn count_zero_passes(input: String) -> String {
    let mut last = START;
    let mut count = 0;
    for (change, pos) in get_dial_positions(last, input) {
        count += change.abs() / SIZE;
        let delta = pos - last;
        if pos == 0 || (last != 0 && ((delta < 0) ^ (change < 0))) {
            count += 1;
        }
        last = pos;
    }
    return count.to_string();
}
