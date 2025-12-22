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
    fn get_implied_bound(&self, index: usize, bounds: &Vec<Bound>) -> Bound {
        if self.a[index] == 0 {
            return Bound::DEFAULT;
        }
        let other_bound = self
            .get_vars_except(index)
            .map(|i| &bounds[i] * self.a[i])
            .fold(Bound::point(0), |a, b| a + b);
        &((&other_bound * -1) + self.b) * (1 / self.a[index])
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

#[derive(Clone)]
pub struct Bound(pub Option<i32>, pub Option<i32>);

impl Bound {
    const DEFAULT: Bound = Bound(None, None);

    pub const fn point(value: i32) -> Self {
        Bound(Some(value), Some(value))
    }

    pub fn closed_low(low: i32) -> Self {
        Bound(Some(low), None)
    }

    pub fn range(&self, max: i32) -> std::ops::Range<i32> {
        self.0.unwrap_or(-max)..self.1.unwrap_or(max)
    }

    pub fn contains(&self, val: i32) -> bool {
        self.0.map_or(true, |l| l <= val) && self.1.map_or(true, |u| u >= val)
    }
}

impl std::ops::Add<i32> for Bound {
    type Output = Bound;

    fn add(self, rhs: i32) -> Self::Output {
        Bound(self.0.map(|l| l + rhs), self.1.map(|u| u + rhs))
    }
}

impl std::ops::Add<Bound> for Bound {
    type Output = Bound;

    fn add(self, rhs: Bound) -> Self::Output {
        Bound(
            self.0.map(|l| rhs.0.map(|ll| l + ll)).unwrap_or(None),
            self.1.map(|u| rhs.1.map(|uu| u + uu)).unwrap_or(None),
        )
    }
}

impl std::ops::Mul<i32> for &Bound {
    type Output = Bound;

    fn mul(self, rhs: i32) -> Self::Output {
        if rhs == 0 {
            Bound::point(0)
        } else if rhs > 0 {
            Bound(self.0.map(|l| l * rhs), self.1.map(|u| u * rhs))
        } else {
            Bound(self.1.map(|u| u * rhs), self.0.map(|l| l * rhs))
        }
    }
}

impl std::ops::BitAnd<Bound> for &Bound {
    type Output = Option<Bound>;

    fn bitand(self, rhs: Bound) -> Self::Output {
        let low = self.0.max(rhs.0);
        let high = [self.1, rhs.1].iter().filter_map(|v| v.clone()).min();
        if high < low {
            None
        } else {
            Some(Bound(low, high))
        }
    }
}

impl std::ops::BitOr<Bound> for &Bound {
    type Output = Bound;

    fn bitor(self, rhs: Bound) -> Self::Output {
        let low = [self.0, rhs.0].iter().filter_map(|v| v.clone()).min();
        Bound(low, self.1.max(rhs.1))
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
            bounds: vec![Bound::DEFAULT; var_count],
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
        let mut pivots: Vec<&usize> = self.pivots.keys().collect();
        pivots.sort_by_key(|p| self.system.rows[self.pivots[p]].get_vars().count());
        for pivot in pivots {
            let row = &self.system.rows[self.pivots[&pivot]];
            for var in row.get_vars() {
                if let Some(new_bounds) =
                    &self.bounds[var] & row.get_implied_bound(var, &self.bounds)
                {
                    self.bounds[var] = new_bounds
                } else {
                    panic!("No solution for {var}");
                }
            }
        }
    }

    fn accumulate_free_options(&self, free: usize, options: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        self.bounds[free]
            .range(100)
            .flat_map(|value| {
                options.iter().map(move |r| {
                    let mut new = r.clone();
                    new[free] = value;
                    new
                })
            })
            .collect()
    }

    pub fn get_solutions(&self) -> Vec<Vec<i32>> {
        (self.free.iter()).fold(vec![vec![0; self.get_var_count()]], |options, free| {
            self.accumulate_free_options(*free, options)
        })
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
