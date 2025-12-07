use super::types::Part;
use super::util::{ClosedInterval, ClosedIntervals, parse};

pub static PARTS: &'static [Part<'static>] = &super::common_parts![count_spoiled, count_total];

fn read_sections(input: &str) -> (&str, &str) {
    input.split_once("\n\n").expect("Wrong number of sections")
}

fn count_spoiled(input: &str) -> String {
    let (range_section, id_section) = read_sections(input);
    let ranges: Vec<ClosedInterval> = range_section
        .split('\n')
        .filter_map(ClosedInterval::parse)
        .collect();
    return id_section
        .split('\n')
        .map(parse::<i64>)
        .filter(|i| ranges.iter().any(|r| r.contains(*i)))
        .count()
        .to_string();
}

fn count_total(input: &str) -> String {
    let (range_section, _) = read_sections(input);
    let mut intervals = ClosedIntervals::new();
    range_section
        .split('\n')
        .filter_map(ClosedInterval::parse)
        .for_each(|i| intervals.add(i));
    return intervals.count().to_string();
}
