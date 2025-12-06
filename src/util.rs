use std::{any::type_name, cmp, fmt, str::FromStr};

pub trait Partition<T> {
    fn partition<F>(&mut self, indicator: F) -> usize
    where
        F: Fn(&T) -> bool;
}

impl<T> Partition<T> for Vec<T> {
    fn partition<F>(&mut self, indicator: F) -> usize
    where
        F: Fn(&T) -> bool,
    {
        let mut j = self.len() - 1;
        for i in 0..self.len() {
            if !indicator(&self[i]) {
                while !indicator(&self[j]) && j > i {
                    j -= 1;
                }
                if j > i {
                    self.swap(i, j);
                } else {
                    return i;
                }
            }
        }
        return self.len();
    }
}

pub const fn posmod(i: i32, m: i32) -> i32 {
    return ((i % m) + m) % m;
}

pub fn parse<T: FromStr>(s: &str) -> T
where
    <T as FromStr>::Err: fmt::Debug,
{
    s.parse::<T>()
        .expect(&format!("Invalid {}: [{}]", type_name::<T>(), s))
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub struct ClosedInterval(pub i64, pub i64);

impl ClosedInterval {
    pub fn contains(&self, element: i64) -> bool {
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
