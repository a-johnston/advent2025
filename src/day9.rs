use std::collections::{BinaryHeap, HashMap};

use svg::{
    Document,
    node::element::{Path, path::Data},
};

use super::{
    types::{ClosedVolume, Part, Vec3},
    util::index_posmod,
};

pub static PARTS: &'static [Part<'static>] =
    &super::all_parts![find_largest_rect, find_largest_contained_rec];

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
    let v = (b - a).signum();
    let d = (b - a).len();

    (1..(d / dist)).map(|i| a + &(v * (i * dist))).collect()
}

fn get_poison_points(points: &Vec<Vec3>) -> Vec<Vec3> {
    let mut poison = vec![];
    for i in 0..points.len() {
        let this = points[i];
        let last_edge = &(&this - &points[index_posmod(i, -1, points.len())]).signum();
        let next_edge = &(&points[index_posmod(i, 1, points.len())] - &this).signum();
        if ccw(last_edge, next_edge) {
            let new = &(&this - &last_edge) + &next_edge;
            if poison.len() > 0 {
                let mut interpolated = interpolate(&poison[poison.len() - 1], &new, 500);
                poison.append(&mut interpolated);
            }
            poison.push(new);
        } else {
            poison.push(&this + &last_edge);
            poison.push(&this - &next_edge);
        }
    }
    return poison;
}

fn svg_point(p: &Vec3, scale: f32) -> (f32, f32) {
    (p.0 as f32 * scale, p.1 as f32 * scale)
}

fn svg_marker(p: &Vec3, rad: f32, scale: f32, color: &str) -> Path {
    let data = Data::new()
        .move_to(svg_point(p, scale))
        .line_by((-rad * scale, rad * scale))
        .line_by((-rad * scale, -rad * scale))
        .line_by((rad * scale, -rad * scale))
        .close();
    Path::new()
        .set("fill", "none")
        .set("stroke", color)
        .set("stroke-width", 0.25)
        .set("d", data)
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

    let scale = 0.001;

    let min_x = points.iter().map(|p| p.0).min().unwrap() as usize - 1;
    let max_x = points.iter().map(|p| p.0).max().unwrap() as usize + 1;

    let min_y = points.iter().map(|p| p.1).min().unwrap() as usize - 1;
    let max_y = points.iter().map(|p| p.1).max().unwrap() as usize + 1;

    let data = points[1..]
        .iter()
        .fold(
            Data::new().move_to((points[0].0 as f32 * scale, points[0].1 as f32 * scale)),
            |d, p| d.line_to(((p.0 as f32) * scale, (p.1 as f32) * scale)),
        )
        .close();

    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 0.25)
        .set("d", data);

    let ap = points[best.1];
    let bp = points[best.2];

    let best_data = Data::new()
        .move_to(svg_point(&ap, scale))
        .line_to(svg_point(&Vec3(ap.0, bp.1, 0), scale))
        .line_to(svg_point(&bp, scale))
        .line_to(svg_point(&Vec3(bp.0, ap.1, 0), scale))
        .close();

    let best_path = Path::new()
        .set("fill", "none")
        .set("stroke", "blue")
        .set("stroke-width", 0.25)
        .set("d", best_data);

    let document = poison.iter().fold(
        Document::new()
            .set(
                "viewBox",
                (
                    min_x as f32 * scale,
                    min_y as f32 * scale,
                    max_x as f32 * scale,
                    max_y as f32 * scale,
                ),
            )
            .add(path)
            .add(best_path)
            .add(svg_marker(&ap, 0.5, scale, "blue"))
            .add(svg_marker(&bp, 0.5, scale, "blue")),
        |d, p| d.add(svg_marker(p, 0.5, scale, "red")),
    );

    println!("{}: {:?}, {}: {:?}", best.1, ap, best.2, bp);
    svg::save("image.svg", &document).unwrap();

    return best.0.to_string();
}
