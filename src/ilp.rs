use std::{collections::{HashMap, HashSet}};

// Represents a linear equation of the form a_1 * x_1 + a_2 * x_2 + .. a_n * x_n = b
pub struct LinearEquation {
    pub a: Vec<i32>,
    pub b: i32,
}

impl std::fmt::Debug for LinearEquation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write![f, "{:?} * X = {}", self.a, self.b]
    }
}

impl std::ops::Add<&LinearEquation> for &LinearEquation {
    type Output = LinearEquation;

    fn add(self, rhs: &LinearEquation) -> Self::Output {
        LinearEquation {
            a: (self.a.iter())
                .zip(rhs.a.iter())
                .map(|(a, b)| a + b)
                .collect(),
            b: self.b + rhs.b,
        }
    }
}

impl std::ops::Mul<i32> for &LinearEquation {
    type Output = LinearEquation;

    fn mul(self, rhs: i32) -> Self::Output {
        LinearEquation {
            a: self.a.iter().map(|c| c * rhs).collect(),
            b: self.b * rhs,
        }
    }
}

impl LinearEquation {
    fn solve(&self, column: usize, assigned: &Vec<i32>) -> i32 {
        if self.a.len() != assigned.len() || assigned[column] != 0 {
            panic!();
        }
        let rhs = (self.a.iter().zip(assigned.iter()))
            .map(|(a, b)| a * b)
            .fold(self.b, |s, c| s - c);
        rhs / self.a[column]
    }

    fn get_implied_bound(&self, index: usize, bounds: &Vec<Bound>) -> Bound {
        let other_ranges(self.get_vars_except(pivot).iter()).flat_map(|i| {

        });
        if self.a[index] == 0 {
            return Bound(None, None);
        }
        let solve_for: Vec<_> = (0..self.a.len())
            .filter(|i| *i != index && self.a[*i] != 0)
            .collect();
        if solve_for.len() != 1 {
            return Bound(None, None);
        }
        if (self.a[index] * self.a[solve_for[0]]) > 0 {
            return Bound(None, Some(self.b / self.a[index]));
        } else {
            return Bound(Some(self.b / -self.a[index]), None);
        }
    }

    fn get_vars(&self) -> impl Iterator<Item = usize> {
        (self.a.iter().enumerate())
            .filter(|(_, a)| **a != 0)
            .map(|(i, _)| i)
    }

    fn get_vars_except(&self, pivot: usize) -> HashSet<usize> {
        self.get_vars()
            .filter(|i| *i != pivot && self.a[*i] != 0)
            .collect()
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

#[derive(Clone)]
pub struct Bound(pub Option<i32>, pub Option<i32>);

impl Bound {
    fn contains(&self, val: i32) -> bool {
        self.0.map_or(true, |l| l <= val) && self.1.map_or(true, |u| u >= val)
    }
}

impl std::ops::BitAnd<Bound> for &Bound {
    type Output = Option<Bound>;

    fn bitand(self, rhs: Bound) -> Self::Output {
        let low = self.0.max(rhs.0);
        let high = self.1.min(rhs.1);
        if high < low {
            None
        } else {
            Some(Bound(low, high))
        }
    }
}

impl std::fmt::Debug for Bound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}..{}",
            self.0.map_or(String::new(), |l| l.to_string()),
            self.1.map_or(String::new(), |u| u.to_string())
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
                self.rows[i] = &self.rows[i] * (1 / self.rows[i].a[j]);
                // Ensure column j is 0 for all other rows
                for ii in 0..row_count {
                    if ii == i {
                        continue;
                    }
                    self.rows[ii] = &self.rows[ii] + &(&self.rows[i] * -self.rows[ii].a[j]);
                }
                // Swap rows to partition pivots and unused rows
                self.rows.swap(i, pivots.len());
                pivots.insert(j, i);
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
            bounds: vec![Bound(None, None); var_count],
        }
    }
}

impl ReducedRowEcheleon {
    pub const fn get_var_count(&self) -> usize {
        self.bounds.len()
    }

    pub fn apply_bound(&mut self, var: usize, bound: Bound) {
        if let Some(new_bound) = &self.bounds[var] & bound {
            self.bounds[var] = new_bound;
        } else {
            panic!("No solution for {var}");
        }
    }

    pub fn infer_bounds(&mut self) {
        let pivot_free_map: HashMap<_, _> = (self.pivots.iter())
            .map(|(p, r)| (*p, self.system.rows[*r].get_vars_except(*p)))
            .collect();
        let mut pivots: Vec<usize> = self.pivots.keys().cloned().collect();
        pivots.sort_by_key(|p| pivot_free_map[p].len());
        for pivot in pivots {
            let row = &self.system.rows[self.pivots[&pivot]];
            for var in row.get_vars() {
                if let Some(new_bounds) = &self.bounds[var] & row.get_implied_bound(var, &self.bounds) {
                    self.bounds[var] = new_bounds
                } else {
                    panic!("No solution for {var}");
                }
            }
        }
    }

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
