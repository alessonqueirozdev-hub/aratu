//! Exact fractions for musical positions.
//!
//! Beats are divided into twos, threes, fives and so on, and a triplet must
//! land exactly where three equal parts of a beat land. Positions are kept as
//! reduced fractions of `i64` so that no rounding ever accumulates; only the
//! final conversion to seconds uses floating point.

use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

/// A reduced fraction with a positive denominator.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Ratio {
    num: i64,
    den: i64,
}

fn gcd(mut a: i128, mut b: i128) -> i128 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

impl Ratio {
    pub const ZERO: Ratio = Ratio { num: 0, den: 1 };
    pub const ONE: Ratio = Ratio { num: 1, den: 1 };

    /// `num / den`, reduced. Panics if `den` is zero.
    pub fn new(num: i64, den: i64) -> Ratio {
        Ratio::from_i128(num as i128, den as i128)
    }

    fn from_i128(num: i128, den: i128) -> Ratio {
        assert!(den != 0, "fraction with a zero denominator");
        let g = gcd(num, den).max(1);
        let (mut n, mut d) = (num / g, den / g);
        if d < 0 {
            n = -n;
            d = -d;
        }
        Ratio {
            num: i64::try_from(n).expect("fraction numerator overflow"),
            den: i64::try_from(d).expect("fraction denominator overflow"),
        }
    }

    pub fn int(n: i64) -> Ratio {
        Ratio { num: n, den: 1 }
    }

    pub fn num(self) -> i64 {
        self.num
    }

    pub fn den(self) -> i64 {
        self.den
    }

    pub fn is_zero(self) -> bool {
        self.num == 0
    }

    pub fn to_f64(self) -> f64 {
        self.num as f64 / self.den as f64
    }

    /// The largest integer not greater than the fraction.
    pub fn floor(self) -> i64 {
        self.num.div_euclid(self.den)
    }

    pub fn is_integer(self) -> bool {
        self.den == 1
    }

    pub fn min(self, other: Ratio) -> Ratio {
        if self <= other { self } else { other }
    }

    pub fn max(self, other: Ratio) -> Ratio {
        if self >= other { self } else { other }
    }
}

impl Default for Ratio {
    fn default() -> Ratio {
        Ratio::ZERO
    }
}

impl From<i64> for Ratio {
    fn from(n: i64) -> Ratio {
        Ratio::int(n)
    }
}

impl Add for Ratio {
    type Output = Ratio;
    fn add(self, o: Ratio) -> Ratio {
        Ratio::from_i128(
            self.num as i128 * o.den as i128 + o.num as i128 * self.den as i128,
            self.den as i128 * o.den as i128,
        )
    }
}

impl Sub for Ratio {
    type Output = Ratio;
    fn sub(self, o: Ratio) -> Ratio {
        self + (-o)
    }
}

impl Mul for Ratio {
    type Output = Ratio;
    fn mul(self, o: Ratio) -> Ratio {
        Ratio::from_i128(
            self.num as i128 * o.num as i128,
            self.den as i128 * o.den as i128,
        )
    }
}

impl Div for Ratio {
    type Output = Ratio;
    fn div(self, o: Ratio) -> Ratio {
        Ratio::from_i128(
            self.num as i128 * o.den as i128,
            self.den as i128 * o.num as i128,
        )
    }
}

impl Neg for Ratio {
    type Output = Ratio;
    fn neg(self) -> Ratio {
        Ratio {
            num: -self.num,
            den: self.den,
        }
    }
}

impl AddAssign for Ratio {
    fn add_assign(&mut self, o: Ratio) {
        *self = *self + o;
    }
}

impl SubAssign for Ratio {
    fn sub_assign(&mut self, o: Ratio) {
        *self = *self - o;
    }
}

impl PartialOrd for Ratio {
    fn partial_cmp(&self, o: &Ratio) -> Option<Ordering> {
        Some(self.cmp(o))
    }
}

impl Ord for Ratio {
    fn cmp(&self, o: &Ratio) -> Ordering {
        (self.num as i128 * o.den as i128).cmp(&(o.num as i128 * self.den as i128))
    }
}

impl fmt::Display for Ratio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.den == 1 {
            write!(f, "{}", self.num)
        } else {
            write!(f, "{}/{}", self.num, self.den)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reduces_and_normalises_sign() {
        assert_eq!(Ratio::new(2, 4), Ratio::new(1, 2));
        assert_eq!(Ratio::new(1, -3), Ratio::new(-1, 3));
        assert_eq!(Ratio::new(0, 5), Ratio::ZERO);
        assert_eq!(Ratio::new(6, 3).to_string(), "2");
        assert_eq!(Ratio::new(-3, 6).to_string(), "-1/2");
    }

    #[test]
    fn three_triplet_eighths_make_exactly_one_beat() {
        let third = Ratio::new(1, 3);
        assert_eq!(third + third + third, Ratio::ONE);
        let fifth = Ratio::new(1, 5);
        let mut sum = Ratio::ZERO;
        for _ in 0..5 {
            sum += fifth;
        }
        assert_eq!(sum, Ratio::ONE);
    }

    #[test]
    fn arithmetic_and_order() {
        let a = Ratio::new(3, 4);
        let b = Ratio::new(2, 3);
        assert_eq!(a + b, Ratio::new(17, 12));
        assert_eq!(a - b, Ratio::new(1, 12));
        assert_eq!(a * b, Ratio::new(1, 2));
        assert_eq!(a / b, Ratio::new(9, 8));
        assert!(b < a);
        assert_eq!(Ratio::new(7, 2).floor(), 3);
        assert_eq!(Ratio::new(-1, 2).floor(), -1);
        assert_eq!(a.max(b), a);
        assert_eq!(a.min(b), b);
    }
}
