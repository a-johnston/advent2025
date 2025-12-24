use super::util::gcd;

// Represents a fraction with 0=numerator 1=denominator
#[derive(Clone, Copy)]
pub struct Fraction(i32, i32);

impl Fraction {
    pub const fn signum(&self) -> i32 {
        return self.0.signum() * self.1.signum();
    }

    pub const fn is_integer(&self) -> bool {
        (self.0 % self.1) == 0
    }

    pub const fn ceil(&self) -> i32 {
        (self.0 / self.1) + (!self.is_integer() as i32)
    }

    pub const fn floor(&self) -> i32 {
        self.0 / self.1
    }

    pub const fn reduce(&self) -> Self {
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

impl std::fmt::Debug for Fraction {
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
