use crate::vec::Vec3;

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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
            let low = latest.min(*interval);
            let high = latest.max(*interval);
            if low.1 < high.0 - 1 {
                new_intervals.push(low);
                new_intervals.push(high);
            } else {
                new_intervals.push(ClosedInterval(low.0, low.1.max(high.1)));
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
