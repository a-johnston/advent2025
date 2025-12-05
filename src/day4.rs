pub static PARTS: &'static [super::Part<'static>] = &[
    super::Part::new("Example 1", "example.txt", single_pass),
    super::Part::new("Part 1", "input.txt", single_pass),
    super::Part::new("Example 2", "example.txt", many_passes),
    super::Part::new("Part 2", "input.txt", many_passes),
];

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

    fn key(&self, width: usize) -> usize {
        (self.x + self.y * width as i32) as usize
    }
}

struct Grid {
    points: Vec<Point>,
    width: usize,
    height: usize,
}

impl Grid {
    fn in_bounds(&self, p: &Point) -> bool {
        p.x >= 0 && p.x < (self.width as i32) && p.y >= 0 && p.y < (self.height as i32)
    }

    fn get_counts(&self) -> Vec<usize> {
        let mut counts: Vec<usize> = vec![0; self.width * self.height];
        self.points
            .iter()
            .flat_map(|p| p.get_adjacent())
            .filter(|p| self.in_bounds(p))
            .map(|p| p.key(self.width))
            .for_each(|i| counts[i] += 1);
        return counts;
    }

    fn remove(&mut self, counts: &[usize], limit: usize) -> usize {
        let old_size = self.points.len();
        self.points.retain(|p| counts[p.key(self.width)] >= limit);
        return old_size - self.points.len();
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
    return Grid {
        points: lines
            .iter()
            .enumerate()
            .flat_map(|(j, row)| parse_row(row, j as i32))
            .collect(),
        width: lines[0].len(),
        height: lines.len(),
    };
}

pub fn single_pass(input: &str) -> String {
    let mut grid = parse_grid(input);
    return grid.remove(&grid.get_counts(), 4).to_string();
}

pub fn many_passes(input: &str) -> String {
    let mut grid = parse_grid(input);
    let old = grid.points.len();
    let mut counts = grid.get_counts();
    while grid.remove(&counts, 4) > 0 {
        counts = grid.get_counts();
    }
    return (old - grid.points.len()).to_string();
}
