use std::collections::{BinaryHeap, HashMap};

use super::types::Part;

pub static PARTS: &'static [Part<'static>] = &[
    Part::new("Example", "example.txt", |s| times_top_circuits(s, 10, 3)),
    Part::new("Part", "input.txt", |s| times_top_circuits(s, 1000, 3)),
    Part::new("Example", "example.txt", full_circuit_last_pair),
    Part::new("Part", "input.txt", full_circuit_last_pair),
];

#[derive(PartialEq, Eq, Hash, Debug)]
struct Point(i64, i64, i64);

impl Point {
    fn parse(s: &str) -> Self {
        let vals: Vec<_> = s.split(',').filter_map(|i| i.parse().ok()).collect();
        return Point(vals[0], vals[1], vals[2]);
    }

    fn sq_dist(&self, other: &Point) -> i64 {
        let dx = self.0 - other.0;
        let dy = self.1 - other.1;
        let dz = self.2 - other.2;
        return dx * dx + dy * dy + dz * dz;
    }
}

struct Graph {
    points: Vec<Point>,
    edge_heap: BinaryHeap<(i64, usize, usize)>,
    point_to_circuit: HashMap<usize, usize>,
    circuit_to_points: HashMap<usize, Vec<usize>>,
}

impl Graph {
    fn read(s: &str) -> Self {
        let points: Vec<_> = s.split('\n').map(Point::parse).collect();
        let edge_heap: BinaryHeap<_> = (0..points.len())
            .flat_map(|i| {
                ((i + 1)..points.len())
                    .map(|j| (i, j))
                    .collect::<Vec<(usize, usize)>>()
            })
            .map(|(i, j)| (-points[i].sq_dist(&points[j]), i, j))
            .collect();
        let mut graph = Graph {
            points: points,
            edge_heap: edge_heap,
            point_to_circuit: HashMap::new(),
            circuit_to_points: HashMap::new(),
        };
        for i in 0..graph.points.len() {
            graph.point_to_circuit.insert(i, i);
            graph.circuit_to_points.insert(i, vec![i]);
        }
        return graph;
    }

    fn connect_shortest(&mut self) -> Option<(usize, usize)> {
        if let Some((_, i, j)) = self.edge_heap.pop() {
            let new_circuit = self.point_to_circuit[&i];
            let old_circuit = self.point_to_circuit[&j];
            if new_circuit == old_circuit {
                return Some((i, j));
            }
            let mut old_points = self.circuit_to_points.remove(&old_circuit).unwrap();
            let mut new_points = self.circuit_to_points.remove(&new_circuit).unwrap();
            for p in old_points.iter().copied() {
                self.point_to_circuit.insert(p, new_circuit);
            }
            new_points.append(&mut old_points);
            self.circuit_to_points.insert(new_circuit, new_points);
            return Some((i, j));
        }
        return None;
    }
}

pub fn times_top_circuits(s: &str, connect: usize, take: usize) -> String {
    let mut graph = Graph::read(s);
    (0..connect).for_each(|_| {
        graph.connect_shortest();
    });
    let mut top_circuits: BinaryHeap<_> =
        graph.circuit_to_points.values().map(|v| v.len()).collect();
    (0..take)
        .map(|_| top_circuits.pop().unwrap())
        .fold(1, |a, b| a * b)
        .to_string()
}

pub fn full_circuit_last_pair(s: &str) -> String {
    let mut graph = Graph::read(s);
    while let Some((i, j)) = graph.connect_shortest() {
        if graph.circuit_to_points.len() == 1 {
            return (graph.points[i].0 * graph.points[j].0).to_string();
        }
    }
    panic!();
}
