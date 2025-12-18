use std::{
    collections::{HashMap, HashSet, VecDeque},
    usize,
};

use super::{
    types::Part,
    util::{mid, parse},
};

#[rustfmt::skip]
pub static PARTS: &'static [Part<'static>] = &super::all_parts![
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
    cache: HashMap<usize, Vec<usize>>,
}

impl Machine {
    fn parse(s: &str) -> Self {
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
            cache: HashMap::new(),
        }
    }

    fn get_solutions(&mut self, target: usize) -> Vec<usize> {
        if !self.cache.contains_key(&target) {
            let mut edge = VecDeque::from([(0, 0)]);
            let mut seen = HashSet::new();
            let mut result = vec![];

            while let Some((state, buttons)) = edge.pop_front() {
                if !seen.insert(buttons) {
                    continue;
                }
                self.buttons.iter().for_each(|b| {
                    if buttons & b.index == 0 {
                        edge.push_back((state ^ b.wires, buttons ^ b.index));
                    }
                });
                if state == target {
                    result.push(buttons);
                }
            }
            self.cache.insert(target, result);
        }
        self.cache.get(&target).unwrap().clone()
    }

    fn get_joltage_change(&self, buttons: impl Iterator<Item = usize>) -> Vec<usize> {
        let mut presses = vec![0_usize; self.joltage.len()];
        for button in buttons {
            for wire in unpack_bit_indexes(self.buttons[button].wires) {
                presses[wire] += 1;
            }
        }
        presses
    }
}

pub fn pack_bit_indexes(bits: impl Iterator<Item = usize>) -> usize {
    bits.fold(0, |a, b| a + (1 << b))
}

pub fn unpack_bit_indexes(x: usize) -> impl Iterator<Item = usize> {
    (0..(size_of::<usize>() * 8)).filter(move |i| ((x >> i) & 1) != 0)
}

fn sum_fewest_presses(input: &str, solver: fn(Machine) -> usize) -> String {
    input
        .split('\n')
        .map(Machine::parse)
        .map(solver)
        .sum::<usize>()
        .to_string()
}

fn fewest_light_presses(mut machine: Machine) -> usize {
    machine
        .get_solutions(machine.lights)
        .iter()
        .map(|b| b.count_ones() as usize)
        .next()
        .expect("No solution")
}

fn get_odds(joltage: &Vec<usize>) -> usize {
    joltage
        .iter()
        .rev()
        .map(|v| v % 2)
        .fold(0, |a, b| (a << 1) + b)
}

fn joltage_helper(machine: &mut Machine, joltage: &Vec<usize>) -> Option<usize> {
    if joltage.iter().all(|x| *x == 0) {
        return Some(0);
    }
    let odds = get_odds(joltage);
    machine
        .get_solutions(odds)
        .iter()
        .filter_map(|buttons| {
            let change = machine.get_joltage_change(unpack_bit_indexes(*buttons));
            if change.iter().zip(joltage.iter()).any(|(c, j)| c > j) {
                return None;
            }
            let new_joltage: Vec<_> = change
                .iter()
                .zip(joltage.iter())
                .map(|(c, j)| (j - c) / 2)
                .collect();
            if let Some(sub_joltage) = joltage_helper(machine, &new_joltage) {
                Some(buttons.count_ones() as usize + 2 * sub_joltage)
            } else {
                None
            }
        })
        .min()
}

fn fewest_joltage_presses(mut machine: Machine) -> usize {
    let joltage = machine.joltage.clone();
    joltage_helper(&mut machine, &joltage).expect("no solution")
}
