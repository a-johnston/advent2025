use crate::util::ClosedIntervals;

use super::util::ClosedInterval;

pub static PARTS: &'static [super::Part<'static>] = &[
    super::Part::new("Example 1", "example.txt", count_spoiled),
    super::Part::new("Part 1", "input.txt", count_spoiled),
    super::Part::new("Example 2", "example.txt", count_total),
    super::Part::new("Part 2", "input.txt", count_total),
];

fn count_spoiled(input: &str) -> String {
    let (range_section, id_section) = input.split_once("\n\n").unwrap();
    let ranges: Vec<ClosedInterval> = range_section
        .split('\n')
        .filter_map(ClosedInterval::parse)
        .collect();
    return id_section
        .split('\n')
        .map(|s| s.parse::<i64>().unwrap())
        .filter(|i| ranges.iter().any(|r| r.contains(*i)))
        .count()
        .to_string();
}

fn count_total(input: &str) -> String {
    let (range_section, _) = input.split_once("\n\n").unwrap();
    let mut intervals = ClosedIntervals::new();
    range_section
        .split('\n')
        .filter_map(ClosedInterval::parse)
        .for_each(|i| intervals.add(i));
    return intervals.count().to_string();
}
