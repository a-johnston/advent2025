use std::{
    any::type_name,
    fmt,
    ops::{Add, Rem},
    str::FromStr,
};

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

pub fn index_posmod(index: usize, change: i64, m: usize) -> usize {
    posmod(index as i64 + change, m as i64) as usize
}

pub fn posmod<T: Copy + Add<Output = T> + Rem<Output = T>>(i: T, m: T) -> T {
    return ((i % m) + m) % m;
}

pub fn parse<T: FromStr>(s: &str) -> T
where
    <T as FromStr>::Err: fmt::Debug,
{
    s.parse::<T>()
        .expect(&format!("Invalid {}: [{}]", type_name::<T>(), s))
}
