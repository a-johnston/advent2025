use std::collections::{HashSet, VecDeque};

use super::{
    ilp::{LinearEquation, LinearSystem, ReducedRowEcheleon},
    types::Part,
    util::{mid, parse},
};

#[rustfmt::skip]
pub static PARTS: &'static [Part<'static>] = &super::example_parts![
    |i| sum_fewest_presses(i, fewest_light_presses),
    |i| sum_fewest_presses(i, fewest_joltage_presses)
];

struct Button {
    index: usize,
    wires: usize,
}

impl Button {
    fn parse(index: usize, s: &str) -> Self {
        Button {
            index: 1 << index,
            wires: pack_bit_indexes(mid(s, 1).split(',').map(parse)),
        }
    }
}

struct Machine {
    lights: usize,
    buttons: Vec<Button>,
    joltage: Vec<usize>,
}

impl From<&str> for Machine {
    fn from(s: &str) -> Self {
        let parts: Vec<_> = s.split_ascii_whitespace().collect();
        Machine {
            lights: pack_bit_indexes(
                mid(parts[0], 1)
                    .chars()
                    .enumerate()
                    .filter_map(move |(i, c)| if c == '#' { Some(i) } else { None }),
            ),
            buttons: parts[1..(parts.len() - 1)]
                .iter()
                .enumerate()
                .map(|(i, bs)| Button::parse(i, bs))
                .collect(),
            joltage: mid(parts[parts.len() - 1], 1)
                .split(',')
                .map(parse)
                .collect(),
        }
    }
}

impl Into<LinearSystem> for Machine {
    fn into(self) -> LinearSystem {
        (self.joltage.iter())
            .enumerate()
            .map(|(i, j)| LinearEquation {
                a: self
                    .buttons
                    .iter()
                    .map(|b| ((b.wires >> i) & 1) as i32)
                    .collect(),
                b: *j as i32,
            })
            .collect()
    }
}

impl Into<ReducedRowEcheleon> for Machine {
    fn into(self) -> ReducedRowEcheleon {
        Into::<LinearSystem>::into(self).into()
    }
}

impl Machine {
    fn solve(&self, target: usize) -> usize {
        let mut edge = VecDeque::from([(0, 0)]);
        let mut seen = HashSet::new();

        while let Some((state, buttons)) = edge.pop_front() {
            if state == target {
                return buttons;
            }
            if !seen.insert(buttons) {
                continue;
            }
            self.buttons.iter().for_each(|b| {
                if buttons & b.index == 0 {
                    edge.push_back((state ^ b.wires, buttons ^ b.index));
                }
            });
        }
        panic!("No solution");
    }
}

pub fn pack_bit_indexes(bits: impl Iterator<Item = usize>) -> usize {
    bits.fold(0, |a, b| a + (1 << b))
}

fn sum_fewest_presses(input: &str, solver: fn(Machine) -> usize) -> String {
    input
        .split('\n')
        .map(Machine::from)
        .map(solver)
        .sum::<usize>()
        .to_string()
}

fn fewest_light_presses(machine: Machine) -> usize {
    machine.solve(machine.lights).count_ones() as usize
}

fn fewest_joltage_presses(machine: Machine) -> usize {
    let rre: ReducedRowEcheleon = machine.into();
    println!("\nRRE for Machine:");
    rre.print_info();
    0
}
