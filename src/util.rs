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

#[allow(dead_code)]
pub fn zip<'a, A, B>(a: &'a Vec<A>, b: &'a Vec<B>) -> impl Iterator<Item = (&'a A, &'a B)> {
    a.iter().zip(b.iter())
}

#[allow(dead_code)]
pub fn zop<'a, T>(a: &'a Vec<T>, b: &'a Vec<T>, f: fn(&'a T, &'a T) -> T) -> Vec<T> {
    zip(a, b).map(|(aa, bb)| f(aa, bb)).collect()
}

pub fn set_bits(bits: &Vec<usize>) -> usize {
    bits.iter().fold(0, |a, b| a + (1 << b))
}

pub fn mid(s: &str, i: usize) -> &str {
    &s[i..(s.len() - i)]
}
