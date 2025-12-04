use std::cmp::max;

pub static PARTS: &'static [super::Part<'static>] = &[
    super::Part::new("Example 1", "example.txt", parse_and_sum_twice_funny),
    super::Part::new("Part 1", "input.txt", parse_and_sum_twice_funny),
    super::Part::new("Example 2", "example.txt", parse_and_sum_all_funny),
    super::Part::new("Part 2", "input.txt", parse_and_sum_all_funny),
];

const fn num_digits(i: u64) -> u32 {
    i.ilog10() + 1
}

fn fixed_len_funny_sum(a: u64, b: u64, repeat_count: u32) -> u64 {
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
        .map(|e| 10_u64.pow(e))
        .sum::<u64>();
    // Divide by mult to get the sub-funny value. Due to integer divison flooring,
    // add 1 to the start unless the start is funny.
    let start = (a / mult) + (a % mult != 0) as u64;
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

fn funny_sum(a: u64, b: u64, repeat: u32) -> u64 {
    if a > b {
        println!("Bad inputs {} {}", a, b);
        return 0;
    }
    let a_d = num_digits(a);
    let b_d = num_digits(b);
    if repeat > max(a_d, b_d) {
        return 0;
    }
    return (a_d..(b_d + 1))
        .map(|d| {
            let start = if a_d == d { a } else { 10_u64.pow(d - 1) };
            let end = if b_d == d { b } else { 10_u64.pow(d) - 1 };
            fixed_len_funny_sum(start, end, repeat)
        })
        .sum();
}

fn parse_range(range: &str) -> Option<(u64, u64)> {
    if let Some((low, high)) = range.split_once('-') {
        match (low.parse::<u64>(), high.parse::<u64>()) {
            (Ok(start), Ok(end)) => {
                return Some((start, end));
            }
            _ => {}
        }
    }
    println!("Bad range: '{}'", range);
    return None;
}

fn parse_ranges(s: &str) -> impl Iterator<Item = (u64, u64)> {
    return s.split(',').filter_map(parse_range);
}

pub fn parse_and_sum_twice_funny(s: &str) -> String {
    parse_ranges(s)
        .map(|(a, b)| funny_sum(a, b, 2))
        .sum::<u64>()
        .to_string()
}

pub fn parse_and_sum_all_funny(s: &str) -> String {
    let ranges: Vec<(u64, u64)> = parse_ranges(s).collect();
    let max_digits = num_digits(*ranges.iter().map(|(a, b)| max(a, b)).max().unwrap_or(&0));
    // Find funny sums for each valid number of repeated sub-funny values
    let mut sums: Vec<u64> = (2..=max_digits)
        .map(|d| ranges.iter().map(|(a, b)| funny_sum(*a, *b, d)).sum())
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
    return sums.iter().sum::<u64>().to_string();
}
