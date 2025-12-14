use std::collections::{HashMap, HashSet};

use super::types::Part;

pub static PARTS: &'static [Part<'static>] = &[
    Part::new("Example", "example.txt", count_all_paths),
    Part::new("Part", "input.txt", count_all_paths),
    Part::new("Example", "example2.txt", count_fft_dac_paths),
    Part::new("Part", "input.txt", count_fft_dac_paths),
];

struct Dag<'a> {
    edge_out: HashMap<&'a str, Vec<&'a str>>,
    edge_in: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> Dag<'a> {
    fn parse(s: &'a str) -> Self {
        let edge_out: HashMap<&str, Vec<_>> = s
            .split('\n')
            .map(|line| {
                let (k, v) = line.split_once(": ").unwrap();
                (k, v.split_whitespace().collect())
            })
            .collect();

        let edge_in = edge_out
            .iter()
            .flat_map(|(k, v)| v.iter().map(move |vv| (vv, k)))
            .fold(HashMap::<&str, Vec<&str>>::new(), |mut h, (v, k)| {
                if !h.contains_key(v) {
                    h.insert(v, vec![]);
                }
                let ks = h.get_mut(v).unwrap();
                ks.push(k);
                h
            });
        Dag {
            edge_in: edge_in,
            edge_out: edge_out,
        }
    }

    fn get_upstream(&'a self, start: &'a str) -> HashSet<&'a str> {
        let mut result = HashSet::new();
        let mut edge = self.edge_in.get(start).unwrap_or(&vec![]).clone();
        while let Some(node) = edge.pop() {
            if result.contains(node) {
                continue;
            }
            result.insert(node);
            edge.extend(self.edge_in.get(node).unwrap_or(&vec![]));
        }
        return result;
    }

    fn count_kahn(&self, start: &str, end: &str, ignore: &str) -> usize {
        let upstream = self.get_upstream(end);
        let mut nodes = upstream.clone();
        nodes.remove(ignore);
        let mut count: HashMap<&str, usize> = HashMap::from([(end, 1)]);
        while !nodes.is_empty() && !count.contains_key(start) {
            let edge: Vec<_> = nodes
                .extract_if(|node| {
                    (self.edge_out[*node].iter())
                        .all(|e| !upstream.contains(e) || count.contains_key(e))
                })
                .collect();
            if edge.len() == 0 {
                break; // Can happen due to ignore
            }
            for node in edge {
                let sum = (self.edge_out[node].iter())
                    .map(|e| count.get(e).unwrap_or(&0))
                    .sum::<usize>();
                count.insert(node, sum);
            }
        }
        *count.get(start).unwrap_or(&0)
    }
}

fn count_all_paths(input: &str) -> String {
    Dag::parse(input).count_kahn("you", "out", "").to_string()
}

fn count_fft_dac_paths(input: &str) -> String {
    let dag = Dag::parse(input);
    let fft_dac = dag.count_kahn("svr", "fft", "dac")
        * dag.count_kahn("fft", "dac", "")
        * dag.count_kahn("dac", "out", "fft");
    let dac_fft = dag.count_kahn("svr", "dac", "fft")
        * dag.count_kahn("dac", "fft", "")
        * dag.count_kahn("fft", "out", "dac");
    (fft_dac + dac_fft).to_string()
}
