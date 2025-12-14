use std::{
    collections::{BinaryHeap, HashMap},
    hash::Hash,
};

use super::{
    types::Part,
    util::{mid, parse, set_bits, zip},
};

pub static PARTS: &'static [Part<'static>] = &super::all_parts![sum_joltage_presses];

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

fn pq_state_search<T, S>(
    start: T,
    target: T,
    options: &Vec<T>,
    edge_fn: fn(&T, &T) -> T,
    valid_fn: fn(&T, &T) -> bool,
    score_fn: fn(usize, &T, &T) -> S,
) -> usize
where
    T: PartialEq + Eq + Hash + PartialOrd + Ord + std::fmt::Debug,
    S: PartialOrd + Ord + std::fmt::Debug,
{
    let mut seen: HashMap<T, usize> = HashMap::new();
    let mut edge: BinaryHeap<(S, usize, T)> = BinaryHeap::new();
    let mut best = 0;
    edge.push((score_fn(0, &start, &target), 0, start));
    while let Some((_, presses, value)) = edge.pop() {
        if value == target {
            if best == 0 || presses < best {
                println!("Found new best: {presses}");
                best = presses;
            }
            continue;
        }
        if best > 0 && presses >= best {
            continue;
        }
        for option in options {
            let new = edge_fn(&value, option);
            if let Some(old_presses) = seen.get(&new)
                && *old_presses <= (presses + 1)
            {
                continue;
            }
            if !valid_fn(&new, &target) {
                continue;
            }
            let score = score_fn(presses + 1, &new, &target);
            // println!("{new:?} has score {score:?} vs target {target:?}");
            edge.push((score, presses + 1, new))
        }
        seen.insert(value, presses);
    }
    return best;
}

fn sum_lights_presses(input: &str) -> String {
    (input.split('\n').map(parse_machine))
        .map(|m| {
            pq_state_search(
                0,
                m.lights,
                &m.buttons.iter().map(set_bits).collect(),
                |a, b| a ^ b,
                |_, _| true,
                |c, _, _| -(c as i32),
            )
        })
        .sum::<usize>()
        .to_string()
}

fn sum_joltage_presses(input: &str) -> String {
    (input.split('\n').map(parse_machine))
        .map(|m| {
            pq_state_search(
                vec![0; m.joltage.len()],
                m.joltage,
                &m.buttons,
                |v, o| {
                    let mut new: Vec<_> = v.iter().map(|vv| *vv).collect();
                    for idx in o {
                        new[*idx] += 1;
                    }
                    new
                },
                |v, t| zip(v, t).all(|(vv, tt)| vv <= tt),
                |_, v, t| -zip(v, t).map(|(vv, tt)| (tt - vv) as i32).sum::<i32>(),
            )
        })
        .sum::<usize>()
        .to_string()
}
