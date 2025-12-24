use std::collections::HashSet;

use adventlib::{Part, util::parse};

pub static PARTS: &'static [Part<'static>] = &[Part::new("Input", "input.txt", solve)];

struct Region {
    width: u32,
    height: u32,
    counts: Vec<u32>,
}

impl Region {
    fn parse(s: &str) -> Self {
        let (size, counts) = s.split_once(": ").unwrap();
        let (width, height) = size.split_once('x').unwrap();
        Region {
            width: parse::<u32>(width),
            height: parse::<u32>(height),
            counts: counts.split(' ').map(parse::<u32>).collect(),
        }
    }

    fn try_solve(&self, shapes: &Vec<HashSet<Shape>>) -> Option<usize> {
        let trivial_grid = (self.width / 3) * (self.height / 3);
        let total_boxes = self.counts.iter().sum::<u32>();
        if total_boxes <= trivial_grid {
            return Some(1);
        }
        let units = self
            .counts
            .iter()
            .zip(shapes.iter())
            .map(|(count, shape_set)| count * shape_set.iter().next().unwrap().count())
            .sum::<u32>();
        let area = self.width * self.height;
        if units > area {
            return Some(0);
        }
        None
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Shape {
    id: u16,
    data: u16,
}

const fn make3(a: u16, b: u16, c: u16) -> u16 {
    (a & 1) + ((b & 1) << 1) + ((c & 1) << 2)
}

const fn flip3(x: u16) -> u16 {
    make3(x >> 2, x >> 1, x)
}

impl Shape {
    fn parse(s: &str) -> Self {
        let (id, body) = s.split_once(":\n").unwrap();
        Shape {
            id: id.parse::<u16>().expect("Bad id"),
            data: body
                .split('\n')
                .flat_map(|l| l.chars())
                .fold(0, |s, c| s * 2 + ((c == '#') as u16)),
        }
    }

    fn get_variants(&self) -> HashSet<Shape> {
        [self.clone(), self.fliph()]
            .iter()
            .flat_map(|s| {
                (0..4).scan(s.clone(), |s, _| {
                    let result = Some(s.rotatecw());
                    *s = result.unwrap();
                    result
                })
            })
            .collect()
    }

    const fn compose(id: u16, top: u16, mid: u16, bot: u16) -> Shape {
        Shape {
            id: id,
            data: (top & 7) + ((mid & 7) << 3) + ((bot & 7) << 6),
        }
    }

    const fn rotatecw(&self) -> Shape {
        Shape::compose(
            self.id,
            make3(self.data >> 6, self.data >> 3, self.data),
            make3(self.data >> 7, self.data >> 4, self.data >> 1),
            make3(self.data >> 8, self.data >> 5, self.data >> 2),
        )
    }

    const fn fliph(&self) -> Shape {
        Shape::compose(
            self.id,
            flip3(self.top()),
            flip3(self.mid()),
            flip3(self.bot()),
        )
    }

    const fn top(&self) -> u16 {
        self.data & 7
    }

    const fn mid(&self) -> u16 {
        (self.data >> 3) & 7
    }

    const fn bot(&self) -> u16 {
        (self.data >> 6) & 7
    }

    const fn count(&self) -> u32 {
        self.data.count_ones() as u32
    }
}

fn solve(input: &str) -> String {
    let mut parts: Vec<_> = input.split("\n\n").collect();
    let regions = parts.pop().unwrap().split('\n').map(Region::parse);
    let shapes: Vec<_> = (parts.iter())
        .map(|s| Shape::parse(s))
        .map(|s| s.get_variants())
        .collect();
    regions
        .map(|r| r.try_solve(&shapes))
        .fold(Some(0), |a, b| match (a, b) {
            (Some(aa), Some(bb)) => Some(aa + bb),
            _ => None,
        })
        .map(|c| c.to_string())
        .expect("hello example :)")
}
