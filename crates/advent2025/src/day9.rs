use std::collections::{BinaryHeap, HashMap};

use adventlib::{Part, all_parts, interval::ClosedVolume, util::index_posmod, vec::Vec3};

pub static PARTS: &'static [Part<'static>] =
    &all_parts![find_largest_rect, find_largest_contained_rec];

fn find_largest_rect(input: &str) -> String {
    let mut points: Vec<_> = input.split('\n').map(Vec3::parse).collect();
    let mut leftmost = HashMap::new(); // Track the first point seen at various y values
    // Ascending lexicographic order in order to traverse points from lowst to highest x value
    points.sort();
    points
        .iter()
        .map(|p| {
            if !leftmost.contains_key(&p.1) {
                leftmost.insert(p.1, p);
            }
            leftmost.values().map(|o| p.area(o)).max().unwrap()
        })
        .max()
        .expect("Empty input")
        .to_string()
}

fn ccw(a: &Vec3, b: &Vec3) -> bool {
    &Vec3(a.1, -a.0, a.2) == b
}

fn interpolate(a: &Vec3, b: &Vec3, dist: i64) -> Vec<Vec3> {
    let v = (b - a).signum() * dist;
    let d = (b - a).len();

    (1..(d / dist)).map(|i| a + &(v * i)).collect()
}

fn get_poison_points(points: &Vec<Vec3>) -> Vec<Vec3> {
    let mut poison = vec![];
    for i in 0..points.len() {
        let this = points[i];
        let last_edge = &(&this - &points[index_posmod(i, -1, points.len())]).signum();
        let next_edge = &(&points[index_posmod(i, 1, points.len())] - &this).signum();
        if ccw(last_edge, next_edge) {
            let new = (&this - last_edge) + next_edge;
            if poison.len() > 0 {
                let mut interpolated = interpolate(&poison[poison.len() - 1], &new, 500);
                poison.append(&mut interpolated);
            }
            poison.push(new);
        } else {
            poison.push(&this + last_edge);
            poison.push(&this - next_edge);
        }
    }
    return poison;
}

fn find_largest_contained_rec(input: &str) -> String {
    let points: Vec<_> = input.split('\n').map(Vec3::parse).collect();
    let poison = get_poison_points(&points);

    let mut heap: BinaryHeap<_> = (0..points.len())
        .flat_map(|i| ((i + 1)..points.len()).map(move |j| (i, j)))
        .map(|(i, j)| (points[i].area(&points[j]), i, j))
        .collect();
    let best = (0..heap.len())
        .filter_map(|_| heap.pop())
        .filter_map(|(a, i, j)| {
            let volume = ClosedVolume::between(&points[i], &points[j]);
            if poison.iter().any(|p| volume.contains(p)) {
                return None;
            }
            return Some((a, i, j));
        })
        .next()
        .expect("Empty input");

    return best.0.to_string();
}
