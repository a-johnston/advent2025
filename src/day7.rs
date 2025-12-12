use std::iter;

use super::{all_parts, types::Part};

pub static PARTS: &'static [Part<'static>] =
    &all_parts![count_classical_splits, count_quantum_splits];

struct Row {
    split_count: u64,
    data: Vec<u64>,
}

impl Row {
    fn parse(s: &str) -> Self {
        let data = iter::once(0)
            .chain(s.chars().map(|c| (c != '.') as u64))
            .chain(iter::once(0))
            .collect();
        return Row {
            split_count: 0,
            data: data,
        };
    }

    fn split_on(&self, splits: &Row) -> Row {
        let mut new_count = self.split_count;
        let mut new_data = vec![0; self.data.len()];
        for i in 1..(self.data.len() - 1) {
            if self.data[i] > 0 {
                if splits.data[i] > 0 {
                    new_count += 1;
                    new_data[i - 1] += self.data[i];
                    new_data[i + 1] += self.data[i];
                } else {
                    new_data[i] += self.data[i];
                }
            }
        }
        return Row {
            split_count: new_count,
            data: new_data,
        };
    }

    fn get_path_count(&self) -> u64 {
        self.data[1..self.data.len() - 1].iter().sum::<u64>()
    }
}

fn process_rows(input: &str) -> Row {
    let mut rows = input.split('\n').map(Row::parse);
    let beams = rows.next().expect("Empty sequence");
    return rows.fold(beams, |a, b| a.split_on(&b));
}

pub fn count_classical_splits(input: &str) -> String {
    process_rows(input).split_count.to_string()
}

pub fn count_quantum_splits(input: &str) -> String {
    process_rows(input).get_path_count().to_string()
}
