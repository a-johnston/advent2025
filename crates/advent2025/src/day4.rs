use adventlib::{Part, all_parts, util::Partition, vec::Vec3};

pub static PARTS: &'static [Part<'static>] = &all_parts![single_pass, many_passes];

struct Grid {
    points: Vec<Vec3>,
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

    fn add(&mut self, p: &Vec3) {
        self.points.push(*p);
        p.get_adjacent_2d()
            .filter(|p| p.in_bounds(self.width, self.height))
            .for_each(|p| self.counts[p.key(self.width)] += 1);
    }

    fn remove(&mut self, limit: usize) -> usize {
        let new_size = self
            .points
            .partition(|p| self.counts[p.key(self.width)] >= limit);
        let removed = self.points.len() - new_size;
        self.points.drain(new_size..).for_each(|p| {
            p.get_adjacent_2d()
                .filter(|a| a.in_bounds(self.width, self.height))
                .for_each(|a| self.counts[a.key(self.width)] -= 1)
        });
        return removed;
    }
}

fn parse_row(row: &str, j: i32) -> impl Iterator<Item = Vec3> {
    row.chars()
        .enumerate()
        .filter(|(_, c)| *c != '.')
        .map(move |(i, _)| Vec3(i as i64, j as i64, 0))
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
