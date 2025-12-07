use super::{common_parts, types::Part};

pub static PARTS: &'static [Part<'static>] = &common_parts![part_1, part_2];

fn get_max_and_index<'a>(i: &'a [u32]) -> (usize, &'a u32) {
    i.iter()
        .enumerate()
        .max_by_key(|(_, value)| *value)
        .expect("Empty array passed to get_max_and_index")
}

fn max_joltage(bank: &str, count: usize) -> u64 {
    // NB: max_by_key returns the last value for equal elements but we want the first
    // so reverse the order. It also makes the ranges slightly cleaner.
    let digits: Vec<_> = bank.chars().filter_map(|c| c.to_digit(10)).rev().collect();
    let mut sum = 0u64;
    let mut limit = digits.len();
    for i in (0..count).rev() {
        let (idx, val) = get_max_and_index(&digits[i..limit]);
        sum = (sum * 10) + (*val as u64);
        limit = idx + i;
    }
    return sum;
}

fn sum_max_across_banks(input: &str, count: usize) -> String {
    input
        .split('\n')
        .map(|b| max_joltage(b, count))
        .sum::<u64>()
        .to_string()
}

fn part_1(input: &str) -> String {
    sum_max_across_banks(input, 2)
}

fn part_2(input: &str) -> String {
    sum_max_across_banks(input, 12)
}
