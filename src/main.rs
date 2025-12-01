use std::collections::HashSet;
use std::fs::read_to_string;

use phf::{phf_map, Map};

mod day1;

type Parts = Map<&'static str, fn(String) -> String>;

pub struct Part<'a> {
    pub name: &'a str,
    pub solver: fn(String) -> String,
}

static PROBLEMS: Map<&'static str, &'static Parts> = phf_map! {
    "day1" => &day1::PARTS,
};

fn run_solvers(day: String, parts: &Parts) {
    println!("{}:", day);
    for (name, solver) in parts {
        match read_to_string(format!("data/{}/{}.txt", day, name)) {
            Ok(content) => println!("  > {}: {}", name, solver(content)),
            Err(err) => println!("  > {}: Error: {}", name, err),
        };
    }
}

fn main() {
    let mut days: HashSet<String> = std::env::args().skip(1).collect();
    if days.len() == 0 {
        days = PROBLEMS.keys().map(|s| String::from(*s)).collect();
    }
    for day in days {
        match PROBLEMS.get(day.as_str()) {
            Some(parts) => run_solvers(day, parts),
            None => println!("No day '{}'!", day),
        }
    }
}
