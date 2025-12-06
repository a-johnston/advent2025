use std::{collections::HashSet, fs::read_to_string, time::SystemTime};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod types;
mod util;

static DELIMS: &'static [&'static str] = &["-", ".."];
static PROBLEMS: &'static [&'static [types::Part<'static>]] = &[
    &day1::PARTS,
    &day2::PARTS,
    &day3::PARTS,
    &day4::PARTS,
    &day5::PARTS,
    &day6::PARTS,
];

fn is_valid_day(day: &usize) -> bool {
    return *day > 0 && *day <= PROBLEMS.len();
}

fn get_arg_days(mut arg: &str) -> HashSet<usize> {
    arg = arg.trim();
    if DELIMS.contains(&arg) {
        return (1..(PROBLEMS.len() + 1)).collect();
    }
    if arg.contains(',') {
        return arg.split(',').map(get_arg_days).flatten().collect();
    }
    for delim in DELIMS {
        if arg.contains(delim) {
            let bounds: Vec<_> = arg
                .split(delim)
                .map(util::parse::<i32>)
                .map(|i| util::posmod(i, PROBLEMS.len() as i32) as usize)
                .filter(is_valid_day)
                .collect();
            let min = *(bounds.iter().max().unwrap());
            let max = bounds.iter().min().unwrap() + 1;
            return (min..max).collect();
        }
    }
    if let Ok(i) = arg.parse::<usize>() {
        if is_valid_day(&i) {
            return (i..(i + 1)).collect();
        } else {
            println!("Invalid day {}", arg)
        }
    }
    return HashSet::new();
}

fn ms_since(time: SystemTime) -> f64 {
    if let Ok(duration) = SystemTime::now().duration_since(time) {
        return (duration.as_micros() as f64) / 1000_f64;
    }
    return -1_f64;
}

fn run_solvers(day: &usize) {
    let start = SystemTime::now();
    println!("Day {}:", day);
    let parts = PROBLEMS[day - 1];
    for part in parts {
        match read_to_string(format!("data/{}/{}", day, part.file)) {
            Ok(content) => println!("  > {}:\t{}", part.name, (part.solver)(content.trim())),
            Err(err) => println!("  > {}:\tError: {}", part.name, err),
        };
    }
    println!("  [{:0.2}ms]", ms_since(start));
}

fn main() {
    let args = std::env::args();
    let start = SystemTime::now();
    if args.len() == 1 {
        run_solvers(&PROBLEMS.len());
    } else {
        let day_set: HashSet<usize> = args
            .skip(1)
            .flat_map(|s| get_arg_days(s.as_str()))
            .collect();
        let mut days: Vec<&usize> = day_set.iter().collect();
        days.sort();
        days.iter().for_each(|d| run_solvers(*d));
    }
    println!("[{:0.2}ms]", ms_since(start));
}
