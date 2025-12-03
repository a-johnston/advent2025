use std::{collections::HashSet, fs::read_to_string};

mod day1;
mod day2;

type Solver = fn(&str) -> String;

struct Part<'a> {
    name: &'a str,
    file: &'a str,
    solver: Solver,
}

impl<'a> Part<'a> {
    pub const fn new(name: &'a str, file: &'a str, solver: Solver) -> Self {
        Self {
            name: name,
            file: file,
            solver: solver,
        }
    }
}

static DELIMS: &'static [&'static str] = &["-", ".."];
static PROBLEMS: &'static [&'static [Part<'static>]] = &[&day1::PARTS, &day2::PARTS];

fn is_valid_day(day: usize) -> bool {
    return day > 0 && day <= PROBLEMS.len();
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
        if let Some((a, b)) = arg.split_once(delim) {
            match (a.parse::<usize>(), b.parse::<usize>()) {
                (Ok(a), Ok(b)) => {
                    if is_valid_day(a) && is_valid_day(b) && a <= b {
                        return (a..(b + 1)).collect();
                    } else {
                        println!("Invalid range {}", arg);
                    }
                }
                _ => {}
            }
        }
    }
    if let Ok(i) = arg.parse::<usize>() {
        if is_valid_day(i) {
            return (i..(i + 1)).collect();
        } else {
            println!("Invalid day {}", arg)
        }
    }
    return HashSet::new();
}

fn run_solvers(day: &usize) {
    println!("Day {}:", day);
    let parts = PROBLEMS[day - 1];
    for part in parts {
        match read_to_string(format!("data/{}/{}", day, part.file)) {
            Ok(content) => println!("  > {}: {}", part.name, (part.solver)(content.trim())),
            Err(err) => println!("  > {}: Error: {}", part.name, err),
        };
    }
}

fn main() {
    let args = std::env::args();
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
}
