use std::cmp::max;

use super::types::Part;
use super::util::ClosedInterval;

pub static PARTS: &'static [Part<'static>] =
    &Part::full(parse_and_sum_twice_funny, parse_and_sum_all_funny);

const fn num_digits(i: i64) -> u32 {
    i.ilog10() + 1
}

fn fixed_len_funny_sum(a: i64, b: i64, repeat_count: u32) -> i64 {
    let d = num_digits(a);
    if repeat_count < 2 || d != num_digits(b) || a > b {
        println!("Bad inputs a={}, b={}, repeat_count={}", a, b, repeat_count);
        return 0;
    }
    if d % repeat_count != 0 {
        return 0;
    }
    // For example consider some ABCDEF with repeat_count=3 and d=6. The stepped
    // range would be 0, 2, 4 and the sum would be for 10^0 + 10^2 + 10^4 = 10101.
    // This is used to construct the funny value ABABAB and sub-funny value AB.
    let mult = (0..d)
        .step_by((d / repeat_count) as usize)
        .map(|e| 10_i64.pow(e))
        .sum::<i64>();
    // Divide by mult to get the sub-funny value. Due to integer divison flooring,
    // add 1 to the start unless the start is funny.
    let start = (a / mult) + (a % mult != 0) as i64;
    let end = b / mult;
    if start > end {
        // This happens if both a and b are less than a shared funny value
        return 0;
    }
    // Since funny = mult * sub_funny, sum(funny_values) = mult * sum(sub_funny_values)
    // and the classic integer range sum formula can be used.
    let sum = mult * (end - start + 1) * (start + end) / 2;
    return sum;
}

fn funny_sum(range: ClosedInterval, repeat: u32) -> i64 {
    if range.0 > range.1 {
        println!("Bad range [{}]", range);
        return 0;
    }
    let a_d = num_digits(range.0);
    let b_d = num_digits(range.1);
    if repeat > max(a_d, b_d) {
        return 0;
    }
    return (a_d..(b_d + 1))
        .map(|d| {
            let start = if a_d == d { range.0 } else { 10_i64.pow(d - 1) };
            let end = if b_d == d { range.1 } else { 10_i64.pow(d) - 1 };
            fixed_len_funny_sum(start, end, repeat)
        })
        .sum();
}

fn parse_ranges(s: &str) -> impl Iterator<Item = ClosedInterval> {
    return s.split(',').filter_map(ClosedInterval::parse);
}

pub fn parse_and_sum_twice_funny(s: &str) -> String {
    parse_ranges(s)
        .map(|r| funny_sum(r, 2))
        .sum::<i64>()
        .to_string()
}

pub fn parse_and_sum_all_funny(s: &str) -> String {
    let ranges: Vec<_> = parse_ranges(s).collect();
    let max_digits = num_digits(ranges.iter().map(|r| max(r.0, r.1)).max().unwrap_or(0));
    // Find funny sums for each valid number of repeated sub-funny values
    let mut sums: Vec<_> = (2..=max_digits)
        .map(|d| ranges.iter().map(|r| funny_sum(*r, d)).sum())
        .collect();
    // Only count each funny number once. In general, any funny number composed
    // of AB repeats can also be composed by A or B repeats. For example, AAAAAA
    // is A repeated 6 times but also AA repeated 3 times and AAA repeated twice.
    for i in (2..=max_digits).rev() {
        for j in (i + 1)..=max_digits {
            if j % i == 0 {
                sums[(i - 2) as usize] -= sums[(j - 2) as usize];
            }
        }
    }
    return sums.iter().sum::<i64>().to_string();
}
