pub static PARTS: &'static [super::Part<'static>] = &[
    super::Part::new("Example 1", "example.txt", parse_and_sum_invalid),
    super::Part::new("Part 1", "input.txt", parse_and_sum_invalid),
];

fn num_digits(i: u64) -> u32 {
    return ((i as f64).log10().floor() as u32) + 1;
}

fn fixed_len_funny_sum(a: u64, b: u64) -> u64 {
    let d = num_digits(a);
    if d != num_digits(b) || a > b {
        println!("Bad inputs {} {}", a, b);
        return 0;
    }
    if d % 2 == 1 {
        return 0; // Odd-length values cannot be funny
    }
    // For any even-length value, there is one funny value for the upper half of
    // the digits. For example, for some 4-digit value ABCD, the funny value is
    // ABAB. Dividing by 100 gives us the sub-funny AB and then multiplying by 101
    // gives the full funny value. However dividing by 101 also can be used to
    // determine if a and b are above or below their respective funny values.
    // Since we are summing the funny values, we can instead sum 101 times the sub-
    // funny values and move the multiplication outside the sum. This allows the
    // classic integer range sum formula to be applied.
    let mult = 10_u64.pow(d / 2) + 1;
    let start = (a / mult) + (a % mult != 0) as u64;
    let end = b / mult;
    if start > end {
        // This happens if both a and b are close and less than their funny value
        return 0;
    }
    let sum = mult * (end - start + 1) * (start + end) / 2;
    return sum;
}

fn funny_sum(a: u64, b: u64) -> u64 {
    if a > b {
        println!("Bad inputs {} {}", a, b);
        return 0;
    }
    let a_d = num_digits(a);
    let b_d = num_digits(b);
    return (a_d..(b_d + 1))
        .map(|d| {
            let start = if a_d == d { a } else { 10_u64.pow(d - 1) };
            let end = if b_d == d { b } else { 10_u64.pow(d) - 1 };
            fixed_len_funny_sum(start, end)
        })
        .sum();
}

fn parse_range(range: &str) -> Option<(u64, u64)> {
    if let Some((low, high)) = range.split_once('-') {
        match (low.parse::<u64>(), high.parse::<u64>()) {
            (Ok(start), Ok(end)) => return Some((start, end)),
            _ => {}
        }
    }
    println!("Bad range: '{}'", range);
    return None;
}

pub fn parse_and_sum_invalid(s: &str) -> String {
    s.split(',')
        .filter_map(parse_range)
        .map(|(a, b)| funny_sum(a, b))
        .sum::<u64>()
        .to_string()
}
