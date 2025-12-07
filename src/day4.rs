use super::{common_parts, types::Part, util::Partition};

pub static PARTS: &'static [Part<'static>] = &common_parts![single_pass, many_passes];

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

static ADJACENT: &[Point] = &[
    Point::new(1, 0),
    Point::new(0, 1),
    Point::new(-1, 0),
    Point::new(0, -1),
    Point::new(1, 1),
    Point::new(-1, 1),
    Point::new(-1, -1),
    Point::new(1, -1),
];

impl std::ops::Add<&Point> for &Point {
    type Output = Point;

    fn add(self, rhs: &Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Point {
    const fn new(x: i32, y: i32) -> Self {
        Point { x: x, y: y }
    }

    fn get_adjacent(&self) -> impl Iterator<Item = Point> {
        ADJACENT.iter().map(|c| c + self)
    }

    fn in_bounds(&self, width: usize, height: usize) -> bool {
        self.x >= 0 && self.x < (width as i32) && self.y >= 0 && self.y < (height as i32)
    }

    fn key(&self, width: usize) -> usize {
        (self.x + self.y * width as i32) as usize
    }
}

struct Grid {
    points: Vec<Point>,
    width: usize,
    height: usize,
    counts: Vec<usize>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        Grid {
            points: vec![],
            width: width,
            height: height,
            counts: vec![0; width * height],
        }
    }

    fn add(&mut self, p: &Point) {
        self.points.push(*p);
        p.get_adjacent()
            .filter(|p| p.in_bounds(self.width, self.height))
            .for_each(|p| self.counts[p.key(self.width)] += 1);
    }

    fn remove(&mut self, limit: usize) -> usize {
        let new_size = self
            .points
            .partition(|p| self.counts[p.key(self.width)] >= limit);
        let removed = self.points.len() - new_size;
        self.points.drain(new_size..).for_each(|p| {
            p.get_adjacent()
                .filter(|a| a.in_bounds(self.width, self.height))
                .for_each(|a| self.counts[a.key(self.width)] -= 1)
        });
        return removed;
    }
}

fn parse_row(row: &str, j: i32) -> impl Iterator<Item = Point> {
    row.chars()
        .enumerate()
        .filter(|(_, c)| *c != '.')
        .map(move |(i, _)| Point::new(i as i32, j as i32))
}

fn parse_grid(s: &str) -> Grid {
    let lines: Vec<_> = s.split('\n').collect();
    let mut grid = Grid::new(lines[0].len(), lines.len());
    (lines.iter())
        .enumerate()
        .flat_map(|(j, row)| parse_row(row, j as i32))
        .for_each(|p| grid.add(&p));
    return grid;
}

pub fn single_pass(input: &str) -> String {
    let mut grid = parse_grid(input);
    return grid.remove(4).to_string();
}

pub fn many_passes(input: &str) -> String {
    let mut grid = parse_grid(input);
    let old = grid.points.len();
    while grid.remove(4) > 0 {}
    return (old - grid.points.len()).to_string();
}
