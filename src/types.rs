use std::{cmp, fmt};

type Solver = fn(&str) -> String;

pub struct Part<'a> {
    pub name: &'a str,
    pub file: &'a str,
    pub solver: Solver,
}

impl<'a> Part<'a> {
    pub const fn new(name: &'a str, file: &'a str, solver: Solver) -> Self {
        Self {
            name: name,
            file: file,
            solver: solver,
        }
    }
}

#[macro_export]
macro_rules! all_parts {
    ( $( $solver:expr ),* ) => {
        [ $( Part::new("Example", "example.txt", $solver), Part::new("Input", "input.txt", $solver) ),* ]
    };
}

#[macro_export]
macro_rules! example_parts {
    ( $( $solver:expr ),* ) => {
        [ $( Part::new("Example", "example.txt", $solver) ),* ]
    };
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, PartialOrd, Ord)]
pub struct Vec3(pub i64, pub i64, pub i64);

impl Vec3 {
    pub fn parse(s: &str) -> Self {
        let mut vals: Vec<_> = s.split(',').filter_map(|i| i.parse().ok()).collect();
        vals.resize(3, 0);
        return Vec3(vals[0], vals[1], vals[2]);
    }

    pub const fn sq_dist(&self, other: &Vec3) -> i64 {
        let dx = self.0 - other.0;
        let dy = self.1 - other.1;
        let dz = self.2 - other.2;
        return dx * dx + dy * dy + dz * dz;
    }

    pub const fn area(&self, other: &Vec3) -> i64 {
        ((self.0 - other.0).abs() + 1)
            * ((self.1 - other.1).abs() + 1)
            * ((self.2 - other.2).abs() + 1)
    }

    pub fn get_adjacent_2d(&self) -> impl Iterator<Item = Vec3> {
        (-1..2)
            .flat_map(|i| (-1..2).map(|j| Vec3(i, j, 0)).collect::<Vec<Vec3>>())
            .filter(|p| p.0 != 0 || p.1 != 0 || p.2 != 0)
            .map(|p| &p + self)
    }

    pub const fn in_bounds(&self, width: usize, height: usize) -> bool {
        self.0 >= 0 && self.0 < (width as i64) && self.1 >= 0 && self.1 < (height as i64)
    }

    pub const fn key(&self, width: usize) -> usize {
        (self.0 + self.1 * width as i64) as usize
    }

    pub const fn signum(self) -> Vec3 {
        Vec3(self.0.signum(), self.1.signum(), self.2.signum())
    }

    pub const fn len(self) -> i64 {
        self.0 + self.1 + self.2
    }
}

impl std::ops::Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Vec3 {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl std::ops::Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl std::ops::Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl std::ops::Mul<i64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: i64) -> Self::Output {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub struct ClosedInterval(pub i64, pub i64);

impl ClosedInterval {
    pub fn new(a: i64, b: i64) -> Self {
        if a < b {
            ClosedInterval(a, b)
        } else {
            ClosedInterval(b, a)
        }
    }

    pub const fn contains(&self, element: i64) -> bool {
        return self.0 <= element && self.1 >= element;
    }

    pub const fn count(&self) -> usize {
        return (self.1 - self.0 + 1) as usize;
    }

    pub fn parse(s: &str) -> Option<Self> {
        if let Some((low, high)) = s.split_once('-') {
            match (low.parse::<i64>(), high.parse::<i64>()) {
                (Ok(start), Ok(end)) => return Some(ClosedInterval(start, end)),
                _ => {}
            }
        }
        println!("Bad interval string: '{}'", s);
        return None;
    }
}

impl std::ops::Add<i64> for ClosedInterval {
    type Output = ClosedInterval;

    fn add(self, rhs: i64) -> Self::Output {
        ClosedInterval(self.0 - rhs, self.1 + rhs)
    }
}

impl std::fmt::Display for ClosedInterval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}-{}", self.0.to_string(), self.1.to_string());
    }
}

pub struct ClosedIntervals {
    intervals: Vec<ClosedInterval>,
}

impl ClosedIntervals {
    pub fn new() -> ClosedIntervals {
        return ClosedIntervals { intervals: vec![] };
    }

    pub fn add(&mut self, new_interval: ClosedInterval) {
        let mut new_intervals = vec![new_interval];
        for interval in &self.intervals {
            let latest = new_intervals.pop().unwrap();
            let low = cmp::min(latest, *interval);
            let high = cmp::max(latest, *interval);
            if low.1 < high.0 - 1 {
                new_intervals.push(low);
                new_intervals.push(high);
            } else {
                new_intervals.push(ClosedInterval(low.0, cmp::max(low.1, high.1)));
            }
        }
        self.intervals = new_intervals;
    }

    pub fn count(&self) -> usize {
        self.intervals.iter().map(|i| i.count()).sum()
    }
}

pub struct ClosedVolume(pub ClosedInterval, pub ClosedInterval, pub ClosedInterval);

impl ClosedVolume {
    pub fn between(a: &Vec3, b: &Vec3) -> Self {
        ClosedVolume(
            ClosedInterval::new(a.0, b.0),
            ClosedInterval::new(a.1, b.1),
            ClosedInterval::new(a.2, b.2),
        )
    }

    pub fn contains(&self, point: &Vec3) -> bool {
        ([self.0, self.1, self.2])
            .iter()
            .zip(([point.0, point.1, point.2]).iter())
            .all(|(i, v)| i.contains(*v))
    }
}
