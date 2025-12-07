use super::{common_parts, types::Part, util::parse};

pub static PARTS: &'static [Part<'static>] =
    &common_parts![|s| sum_ops(s, parse_rows), |s| sum_ops(s, parse_cols)];

fn parse_rows(source: &Vec<&str>, start: usize, end: usize) -> Vec<u64> {
    (source.iter())
        .map(move |s| parse::<u64>(s[start..end].trim()))
        .collect()
}

fn parse_cols(source: &Vec<&str>, start: usize, end: usize) -> Vec<u64> {
    (start..end)
        .map(|i| {
            source
                .iter()
                .filter_map(|s| s[i..i + 1].parse::<u64>().ok())
                .fold(0, |a, b| a * 10 + b)
        })
        .filter(|i| *i > 0)
        .collect()
}

pub fn sum_ops(input: &str, parser: fn(&Vec<&str>, usize, usize) -> Vec<u64>) -> String {
    let mut rows: Vec<_> = input.split('\n').collect();
    let max = rows.iter().map(|s| s.len()).max().unwrap();
    let ops: Vec<_> = (rows.pop().unwrap())
        .chars()
        .enumerate()
        .filter(|(_, c)| !c.is_whitespace())
        .map(|(i, c)| (i, c == '*'))
        .chain(std::iter::once((max, false)))
        .collect();
    ops.iter()
        .zip(ops[1..].iter())
        .map(|((i, mult), (j, _))| {
            parser(&rows, *i, *j).iter().fold(
                *mult as u64,
                if *mult { |a, b| a * b } else { |a, b| a + b },
            )
        })
        .sum::<u64>()
        .to_string()
}
