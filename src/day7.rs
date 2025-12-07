use super::{common_parts, types::Part};

pub static PARTS: &'static [Part<'static>] =
    &common_parts![count_classical_splits, count_quantum_splits];

fn get_row_activations(s: &str) -> Vec<u64> {
    s.chars().map(|c| (c != '.') as u64).collect()
}

// TODO: Figure out why I needed mut on combo
pub fn aggregate_beam_and_splitters(
    input: &str,
    mut combo: impl FnMut(u64, u64) -> u64,
) -> Vec<u64> {
    let mut splitters = input.split('\n').map(get_row_activations);
    let initial_beams = splitters.next().expect("Empty sequence");
    splitters.fold(initial_beams, |mut beams, splitter| {
        for i in 0..beams.len() {
            if beams[i] > 0 && splitter[i] > 0 {
                if i > 0 {
                    beams[i - 1] = combo(beams[i - 1], beams[i]);
                }
                if i < beams.len() - 1 {
                    beams[i + 1] = combo(beams[i + 1], beams[i]);
                }
                beams[i] = 0;
            }
        }
        return beams;
    })
}

pub fn count_classical_splits(input: &str) -> String {
    let mut splits = 0;
    aggregate_beam_and_splitters(input, |_, _| {
        splits += 1;
        return 1;
    });
    return (splits / 2).to_string();
}

pub fn count_quantum_splits(input: &str) -> String {
    let result = aggregate_beam_and_splitters(input, |new, old| new + old);
    return result.iter().sum::<u64>().to_string();
}
