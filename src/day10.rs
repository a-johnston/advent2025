use std::collections::{HashSet, VecDeque};

use z3::{Solver, ast::Int};

use super::{
    types::Part,
    util::{mid, parse, set_bits, zip},
};

pub static PARTS: &'static [Part<'static>] =
    &super::all_parts![|i| solve_fewest_presses(i, fewest_light_presses), |i| {
        solve_fewest_presses(i, fewest_joltage_presses)
    }];

struct Machine {
    lights: usize,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

fn parse_machine(s: &str) -> Machine {
    let parts: Vec<_> = s.split_ascii_whitespace().collect();
    Machine {
        lights: set_bits(
            &mid(parts[0], 1)
                .chars()
                .enumerate()
                .filter_map(|(i, c)| if c == '#' { Some(i) } else { None })
                .collect(),
        ),
        buttons: parts[1..(parts.len() - 1)]
            .iter()
            .map(|bs| mid(bs, 1).split(',').map(parse).collect())
            .collect(),
        joltage: mid(parts[parts.len() - 1], 1)
            .split(',')
            .map(parse)
            .collect(),
    }
}

fn solve_fewest_presses(input: &str, solver: fn(Machine) -> usize) -> String {
    input
        .split('\n')
        .map(parse_machine)
        .map(solver)
        .sum::<usize>()
        .to_string()
}

fn fewest_light_presses(machine: Machine) -> usize {
    let mut edge: VecDeque<_> = VecDeque::from([(0, 0)]);
    let mut seen: HashSet<_> = HashSet::new();
    let options: Vec<_> = machine.buttons.iter().map(set_bits).collect();
    while let Some((presses, value)) = edge.pop_front() {
        if !seen.insert(value) {
            continue;
        }
        if value == machine.lights {
            return presses;
        }
        edge.extend(options.iter().map(|option| (presses + 1, value ^ option)));
    }
    panic!("Could not reach target state");
}

fn fewest_joltage_presses(machine: Machine) -> usize {
    let solver = Solver::new();
    let joltage_buttons: Vec<_> = (0..machine.joltage.len())
        .map(|i| {
            machine
                .buttons
                .iter()
                .enumerate()
                .filter_map(|(j, b)| if b.contains(&i) { Some(j) } else { None })
                .collect::<Vec<usize>>()
        })
        .collect();
    let buttons: Vec<_> = (0..machine.buttons.len())
        .map(|i| Int::new_const(format!("Button {i}").as_str()))
        .collect();
    buttons.iter().for_each(|b| solver.assert(b.ge(0)));
    for (total, inputs) in zip(&machine.joltage, &joltage_buttons) {
        let sum = inputs.iter().fold(Int::from(0), |s, i| s + &buttons[*i]);
        solver.assert(sum.eq(*total as u64));
    }
    solver
        .solutions(buttons, false)
        .map(|v| {
            v.iter()
                .map(|i| i.as_u64().unwrap() as usize)
                .sum::<usize>()
        })
        .min().expect("No solutions")
}
