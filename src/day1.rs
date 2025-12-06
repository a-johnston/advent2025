use std::vec::Vec;

use super::types::Part;
use super::util::posmod;

const START: i32 = 50;
const SIZE: i32 = 100;

pub static PARTS: &'static [Part<'static>] = &Part::full(count_zeros, count_zero_passes);

fn parse_rotation(rot: &str) -> Option<i32> {
    let sign = (rot.starts_with('R') as i32) * 2 - 1;
    if let Ok(val) = rot[1..].parse::<i32>() {
        return Some(val * sign);
    }
    println!("Invalid rotation: {}", rot);
    return None;
}

fn get_rotations(input: &str) -> impl Iterator<Item = i32> {
    input.split('\n').filter_map(parse_rotation)
}

fn get_dial_positions(mut pos: i32, input: &str) -> Vec<(i32, i32)> {
    let mut rotations = Vec::new();
    for rot in get_rotations(input) {
        pos = posmod(pos + rot, SIZE);
        rotations.push((rot, pos));
    }
    return rotations;
}

pub fn count_zeros(input: &str) -> String {
    get_dial_positions(START, input)
        .iter()
        .filter(|(_, pos)| *pos == 0)
        .count()
        .to_string()
}

pub fn count_zero_passes(input: &str) -> String {
    let mut last = START;
    let mut count = 0;
    for (change, pos) in get_dial_positions(last, input) {
        // Count how many full rotations the dial did and then a bit extra if
        // the overall delta is "pointing" differently than the rotation.
        count += change.abs() / SIZE;
        let delta = pos - last;
        if pos == 0 || (last != 0 && ((delta < 0) ^ (change < 0))) {
            count += 1;
        }
        last = pos;
    }
    return count.to_string();
}
