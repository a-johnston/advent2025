use std::collections::HashMap;

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

    fn get_implied_bound(&self, index: usize) -> (Option<i32>, Option<i32>) {
        if self.a[index] == 0 {
            return (None, None);
        }
        let solve_for: Vec<_> = (0..self.a.len())
            .filter(|i| *i != index && self.a[*i] != 0)
            .collect();
        if solve_for.len() != 1 {
            return (None, None);
        }
        if (self.a[index] * self.a[solve_for[0]]) > 0 {
            return (None, Some(self.b / self.a[index]));
        } else {
            return (Some(self.b / -self.a[index]), None);
        }
    }
}

pub struct LinearSystem {
    rows: Vec<LinearEquation>,
}

impl FromIterator<LinearEquation> for LinearSystem {
    fn from_iter<T: IntoIterator<Item = LinearEquation>>(iter: T) -> Self {
        LinearSystem {
            rows: iter.into_iter().collect(),
        }
    }
}

#[derive(Clone)]
struct Bound(Option<i32>, Option<i32>);

impl Bound {
    fn contains(&self, val: i32) -> bool {
        self.0.map_or(true, |l| l <= val) && self.1.map_or(true, |u| u >= val)
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
        // Remove any unused vars in reverse order
        let mut reindex: HashMap<usize, usize> = HashMap::new();
        for i in (0..var_count).rev() {
            if self.rows.iter().all(|r| r.a[i] == 0) {
                self.rows.iter_mut().for_each(|r| {
                    r.a.remove(i);
                });
            } else {
                reindex.insert(i, reindex.len());
            }
        }
        // Re-reverse the reversed reindex
        ReducedRowEcheleon {
            system: self,
            pivots: pivots
                .iter()
                .map(|(k, v)| (reindex.len() - 1 - reindex[k], *v))
                .collect(),
            bounds: vec![Bound(None, None); reindex.len()],
        }
    }
}

impl ReducedRowEcheleon {
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
