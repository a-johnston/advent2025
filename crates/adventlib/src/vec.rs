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
            .map(move |p| p + self)
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

macro_rules! vec_ops {
    ($A:ty) => {
        impl std::ops::Neg for $A {
            type Output = Vec3;

            fn neg(self) -> Self::Output {
                Vec3(-self.0, -self.1, -self.2)
            }
        }

        impl std::ops::Mul<i64> for $A {
            type Output = Vec3;

            fn mul(self, rhs: i64) -> Self::Output {
                Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
            }
        }
    };
}

macro_rules! vec_vec_ops {
    ($A:ty, $B:ty) => {
        impl std::ops::Add<$B> for $A {
            type Output = Vec3;

            fn add(self, rhs: $B) -> Vec3 {
                Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
            }
        }

        impl std::ops::Sub<$B> for $A {
            type Output = Vec3;

            fn sub(self, rhs: $B) -> Self::Output {
                Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
            }
        }
    };
}

vec_ops!(Vec3);
vec_ops!(&Vec3);

vec_vec_ops!(Vec3, Vec3);
vec_vec_ops!(&Vec3, Vec3);
vec_vec_ops!(Vec3, &Vec3);
vec_vec_ops!(&Vec3, &Vec3);
