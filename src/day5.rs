use super::types::Part;
use super::util::{ClosedInterval, ClosedIntervals};

pub static PARTS: &'static [Part<'static>] = &Part::full(count_spoiled, count_total);

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
