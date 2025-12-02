use std::fs::read_to_string;

mod day1;

struct Part<'a> {
    name: &'a str,
    file: &'a str,
    solver: fn(String) -> String,
}

static PROBLEMS: &'static [&'static [Part<'static>]] = &[&day1::PARTS];

fn get_day_num(arg: String) -> Option<usize> {
    let lower = arg.to_lowercase();
    let s = lower.strip_prefix("day").unwrap_or(lower.as_str());
    if let Ok(i) = s.parse::<usize>() {
        if i > 0 && i <= PROBLEMS.len() {
            return Some(i);
        }
    }
    println!("Can't handle arg {}", arg);
    return None;
}

fn run_solvers(day: &usize) {
    println!("Day {}:", day);
    let parts = PROBLEMS[day - 1];
    for part in parts {
        match read_to_string(format!("data/{}/{}", day, part.file)) {
            Ok(content) => println!("  > {}: {}", part.name, (part.solver)(content)),
            Err(err) => println!("  > {}: Error: {}", part.name, err),
        };
    }
}

fn main() {
    let mut days: Vec<usize> = std::env::args().skip(1).filter_map(get_day_num).collect();
    if days.len() == 0 {
        days.push(PROBLEMS.len());
    }
    days.iter().for_each(run_solvers);
}
