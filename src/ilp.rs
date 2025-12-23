use std::{collections::HashMap, fmt::Debug};

use super::util::gcd;

// Represents a fraction with 0=numerator 1=denominator
#[derive(Clone, Copy)]
pub struct Fraction(i32, i32);

impl Fraction {
    const fn signum(&self) -> i32 {
        return self.0.signum() * self.1.signum();
    }

    const fn is_integer(&self) -> bool {
        (self.0 % self.1) == 0
    }

    const fn ceil(&self) -> i32 {
        (self.0 / self.1) + (!self.is_integer() as i32)
    }

    const fn floor(&self) -> i32 {
        self.0 / self.1
    }

    const fn reduce(&self) -> Self {
        if self.0 == 0 || self.1 == 0 {
            Fraction(0, 1)
        } else {
            let x = gcd(self.0.abs(), self.1.abs()) * self.1.signum();
            Fraction(self.0 / x, self.1 / x)
        }
    }
}

impl PartialEq for Fraction {
    fn eq(&self, other: &Self) -> bool {
        self.0 * other.1 == other.0 * self.1
    }
}

impl Eq for Fraction {}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Fraction {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let sign = (self - other).signum();
        if sign > 0 {
            std::cmp::Ordering::Greater
        } else if sign < 0 {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Equal
        }
    }
}

impl From<i32> for Fraction {
    fn from(value: i32) -> Self {
        Fraction(value, 1)
    }
}

impl Debug for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.1 == 1 {
            write![f, "{}", self.0]
        } else {
            write![f, "{}/{}", self.0, self.1]
        }
    }
}

macro_rules! fraction_i32_ops {
    ($lhs:ty) => {
        impl std::ops::Mul<i32> for $lhs {
            type Output = Fraction;

            fn mul(self, rhs: i32) -> Self::Output {
                Fraction(self.0 * rhs, self.1)
            }
        }

        impl std::ops::Div<i32> for $lhs {
            type Output = Fraction;

            fn div(self, rhs: i32) -> Self::Output {
                Fraction(self.0, self.1 * rhs)
            }
        }
    };
}

impl std::cmp::PartialEq<i32> for Fraction {
    fn eq(&self, rhs: &i32) -> bool {
        self.0 == self.1 * rhs
    }
}

fraction_i32_ops!(Fraction);
fraction_i32_ops!(&Fraction);

macro_rules! fraction_fraction_ops {
    ($lhs:ty, $rhs:ty) => {
        impl std::ops::Mul<$rhs> for $lhs {
            type Output = Fraction;

            fn mul(self, rhs: $rhs) -> Self::Output {
                Fraction(self.0 * rhs.0, self.1 * rhs.1)
            }
        }

        impl std::ops::Div<$rhs> for $lhs {
            type Output = Fraction;

            fn div(self, rhs: $rhs) -> Self::Output {
                Fraction(self.0 * rhs.1, self.1 * rhs.0)
            }
        }

        impl std::ops::Add<$rhs> for $lhs {
            type Output = Fraction;

            fn add(self, rhs: $rhs) -> Self::Output {
                Fraction(self.0 * rhs.1 + rhs.0 * self.1, self.1 * rhs.1)
            }
        }

        impl std::ops::Sub<$rhs> for $lhs {
            type Output = Fraction;

            fn sub(self, rhs: $rhs) -> Self::Output {
                Fraction(self.0 * rhs.1 - rhs.0 * self.1, self.1 * rhs.1)
            }
        }
    };
}

impl std::iter::Sum for Fraction {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Fraction::from(0), |a, b| a + b)
    }
}

fraction_fraction_ops!(Fraction, Fraction);
fraction_fraction_ops!(&Fraction, Fraction);
fraction_fraction_ops!(Fraction, &Fraction);
fraction_fraction_ops!(&Fraction, &Fraction);

// Represents a linear equation of the form a_1 * x_1 + a_2 * x_2 + .. a_n * x_n = b
pub struct LinearEquation {
    pub a: Vec<Fraction>,
    pub b: Fraction,
}

impl Debug for LinearEquation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write![f, "{:?} * X = {:?}", self.a, self.b]
    }
}

impl std::ops::Add<&LinearEquation> for &LinearEquation {
    type Output = LinearEquation;

    fn add(self, rhs: &LinearEquation) -> Self::Output {
        LinearEquation {
            a: (self.a.iter())
                .zip(rhs.a.iter())
                .map(|(a, b)| (a + b).reduce())
                .collect(),
            b: (self.b + rhs.b).reduce(),
        }
    }
}

impl std::ops::Mul<Fraction> for &LinearEquation {
    type Output = LinearEquation;

    fn mul(self, rhs: Fraction) -> Self::Output {
        LinearEquation {
            a: self.a.iter().map(|c| (c * rhs).reduce()).collect(),
            b: (self.b * rhs).reduce(),
        }
    }
}

impl LinearEquation {
    fn solve(&self, index: usize, values: &Vec<Fraction>) -> Fraction {
        if self.a[index] == 0 {
            return Fraction::from(0);
        }
        let others: Fraction = self
            .get_vars_except(index)
            .map(|i| &self.a[i] * values[i])
            .sum::<Fraction>();
        return ((self.b - others) / self.a[index]).reduce();
    }

    fn get_implied_bound(&self, index: usize, bounds: &Vec<Bound>) -> Bound {
        if self.a[index] == 0 {
            return Bound::DEFAULT;
        }
        let other_bound = self
            .get_vars_except(index)
            .map(|i| bounds[i] * self.a[i])
            .fold(Bound::point(0.into()), |a, b| a + b);
        (other_bound * (-1).into() + self.b) * (Fraction::from(1) / self.a[index])
    }

    fn get_vars(&self) -> impl Iterator<Item = usize> {
        (self.a.iter().enumerate())
            .filter(|(_, a)| **a != 0)
            .map(|(i, _)| i)
    }

    fn get_vars_except(&self, pivot: usize) -> impl Iterator<Item = usize> {
        self.get_vars()
            .filter(move |i| *i != pivot && self.a[*i] != 0)
    }
}

pub struct LinearSystem {
    rows: Vec<LinearEquation>,
}

impl FromIterator<LinearEquation> for LinearSystem {
    fn from_iter<T: IntoIterator<Item = LinearEquation>>(iter: T) -> Self {
        let rows = iter.into_iter().collect();
        LinearSystem { rows: rows }
    }
}

#[derive(Clone, Copy)]
pub struct Bound(pub Option<Fraction>, pub Option<Fraction>);

impl Bound {
    const DEFAULT: Bound = Bound(None, None);

    pub const fn point(value: Fraction) -> Self {
        Bound(Some(value), Some(value))
    }

    pub fn closed_low(low: Fraction) -> Self {
        Bound(Some(low), None)
    }

    pub fn integer_range(&self, max: i32) -> impl Iterator<Item = Fraction> {
        let low = self.0.map(|f| f.ceil()).unwrap_or(-max);
        let high = self.1.map(|f| f.floor()).unwrap_or(max);
        (low..(high + 1)).map(|i| Fraction::from(i))
    }

    pub fn contains(&self, val: Fraction) -> bool {
        self.0.map_or(true, |l| l <= val) && self.1.map_or(true, |u| u >= val)
    }
}

macro_rules! bound_ops {
    ($bound:ty) => {
        impl std::ops::Add<Fraction> for $bound {
            type Output = Bound;

            fn add(self, rhs: Fraction) -> Self::Output {
                Bound(self.0.map(|l| (l + rhs).reduce()), self.1.map(|u| (u + rhs).reduce()))
            }
        }

        impl std::ops::Add<Bound> for $bound {
            type Output = Bound;

            fn add(self, rhs: Bound) -> Self::Output {
                Bound(
                    self.0.map(|l| rhs.0.map(|ll| (l + ll).reduce())).unwrap_or(None),
                    self.1.map(|u| rhs.1.map(|uu| (u + uu).reduce())).unwrap_or(None),
                )
            }
        }

        impl std::ops::Mul<Fraction> for $bound {
            type Output = Bound;

            fn mul(self, rhs: Fraction) -> Self::Output {
                if rhs == 0 {
                    Bound::point(0.into())
                } else if rhs > Fraction::from(0) {
                    Bound(self.0.map(|l| l * rhs), self.1.map(|u| (u * rhs).reduce()))
                } else {
                    Bound(self.1.map(|u| u * rhs), self.0.map(|l| (l * rhs).reduce()))
                }
            }
        }

        impl std::ops::BitAnd<Bound> for $bound {
            type Output = Option<Bound>;

            fn bitand(self, rhs: Bound) -> Self::Output {
                let low = self.0.max(rhs.0);
                let high = [self.1, rhs.1].iter().filter_map(|v| v.clone()).min();
                match (low, high) {
                    (Some(l), Some(h)) => {
                        if h < l {
                            None
                        } else {
                            Some(Bound(low, high))
                        }
                    }
                    _ => Some(Bound(low, high)),
                }
            }
        }

        impl std::ops::BitOr<Bound> for $bound {
            type Output = Bound;

            fn bitor(self, rhs: Bound) -> Self::Output {
                let low = [self.0, rhs.0].iter().filter_map(|v| v.clone()).min();
                Bound(low, self.1.max(rhs.1))
            }
        }
    };
}

bound_ops!(Bound);
bound_ops!(&Bound);

impl std::fmt::Debug for Bound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}..{}",
            self.0.map_or(String::new(), |l| format!("{:?}", l)),
            self.1.map_or(String::new(), |u| format!("{:?}", u)),
        )
    }
}

pub struct ReducedRowEcheleon {
    system: LinearSystem,
    pivots: HashMap<usize, usize>, // Column/variable index to row index
    bounds: Vec<Bound>,
    free: Vec<usize>,
}

impl Into<ReducedRowEcheleon> for LinearSystem {
    fn into(mut self) -> ReducedRowEcheleon {
        let row_count = self.rows.len();
        let var_count = self.rows[0].a.len();
        if self.rows.iter().any(|r| r.a.len() != var_count) {
            panic!("Mismatched coefficient vectors");
        }
        let mut pivots: HashMap<usize, usize> = HashMap::new(); // Column index to pivot row index
        for j in 0..var_count {
            // Pick the first row which is nonzero for column j and is not already a pivot row
            if let Some(i) = (pivots.len()..row_count)
                .filter(|ii| self.rows[*ii].a[j] != 0)
                .next()
            {
                // Ensure column j has a value of 1
                self.rows[i] = &self.rows[i] * (Fraction::from(1) / self.rows[i].a[j]);
                // Ensure column j is 0 for all other rows
                for ii in 0..row_count {
                    if ii == i {
                        continue;
                    }
                    self.rows[ii] = &self.rows[ii]
                        + &(&self.rows[i] * (Fraction::from(-1) * self.rows[ii].a[j]));
                }
                // Swap rows to partition pivots and unused rows
                self.rows.swap(i, pivots.len());
                pivots.insert(j, pivots.len());
            }
        }
        self.rows.retain(|r| {
            let zero = r.a.iter().all(|c| *c == 0);
            if zero && r.b != 0 {
                panic!()
            }
            !zero
        });
        ReducedRowEcheleon {
            system: self,
            free: (0..var_count)
                .filter(|i| !pivots.contains_key(&i))
                .collect(),
            pivots: pivots,
            bounds: vec![Bound::DEFAULT; var_count],
        }
    }
}

impl ReducedRowEcheleon {
    pub const fn get_var_count(&self) -> usize {
        self.bounds.len()
    }

    pub fn restrict_bound(&mut self, var: usize, bound: Bound) {
        if let Some(new_bound) = &self.bounds[var] & bound {
            self.bounds[var] = new_bound;
        } else {
            panic!("No solution for {var}");
        }
    }

    pub fn infer_bounds(&mut self) {
        let mut pivots: Vec<&usize> = self.pivots.keys().collect();
        pivots.sort_by_key(|p| self.system.rows[self.pivots[p]].get_vars().count());
        for pivot in pivots {
            let row = &self.system.rows[self.pivots[&pivot]];
            for var in row.get_vars() {
                let new_bound = row.get_implied_bound(var, &self.bounds);
                if let Some(new_bounds) = &self.bounds[var] & new_bound {
                    self.bounds[var] = new_bounds
                } else {
                    panic!("infer_bounds: No solution for {var}");
                }
            }
        }
    }

    fn accumulate_free_options<'a>(
        &'a self,
        vars: usize,
        free: &mut Vec<usize>,
    ) -> Box<dyn Iterator<Item = Vec<Fraction>> + 'a> {
        if let Some(i) = free.pop() {
            let base = self.accumulate_free_options(vars, free);
            let bounds = &self.bounds[i];
            Box::new(base.flat_map(move |r| {
                bounds.integer_range(15).map(move |value| {
                    let mut new = r.clone();
                    new[i] = value;
                    new
                })
            }))
        } else {
            Box::new(std::iter::once(vec![Fraction::from(0); vars]))
        }
    }

    pub fn get_solutions(&self) -> impl Iterator<Item = Vec<i32>> {
        self.accumulate_free_options(self.get_var_count(), &mut self.free.clone())
            .filter_map(|mut r| {
                self.pivots.iter().for_each(|(p, rr)| {
                    r[*p] = self.system.rows[*rr].solve(*p, &r).into();
                });
                if (self.pivots.iter())
                    .all(|(p, _)| self.bounds[*p].contains(r[*p]) && r[*p].is_integer())
                {
                    return Some(r.iter().map(|f| f.floor()).collect());
                }
                None
            })
    }

    #[allow(dead_code)]
    pub fn print_info(&self) {
        for row in &self.system.rows {
            println!("{:?}", row);
        }
        for (i, b) in self.bounds.iter().enumerate() {
            println!(
                "x{i} {b:?} {}",
                if self.pivots.contains_key(&i) {
                    ""
                } else {
                    "(free)"
                }
            );
        }
    }
}
