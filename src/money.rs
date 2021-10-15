use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Money(i64);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Currency {
    USD(Money),
    JPY(Money),
    RMB(Money),
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Currency::USD(amount) => write!(f, "{} cents", amount.0),
            Currency::JPY(amount) => write!(f, "{} yen", amount.0),
            Currency::RMB(amount) => write!(f, "{} rmb", amount.0),
        }
    }
}

impl Add for Currency {
    type Output = Self;

    fn add(self, other: Currency) -> Currency {
        match (self, other) {
            (Currency::USD(a), Currency::USD(b)) => Currency::USD(a + b),
            (Currency::JPY(a), Currency::JPY(b)) => Currency::JPY(a + b),
            (Currency::RMB(a), Currency::RMB(b)) => Currency::RMB(a + b),
            (_, _) => panic!(),
        }
    }
}

impl Sub for Currency {
    type Output = Self;

    fn sub(self, other: Currency) -> Currency {
        match (self, other) {
            (Currency::USD(a), Currency::USD(b)) => Currency::USD(a - b),
            (Currency::JPY(a), Currency::JPY(b)) => Currency::JPY(a - b),
            (Currency::RMB(a), Currency::RMB(b)) => Currency::RMB(a - b),
            (_, _) => panic!(),
        }
    }
}

impl Mul for Currency {
    type Output = Self;

    fn mul(self, other: Currency) -> Currency {
        match (self, other) {
            (Currency::USD(a), Currency::USD(b)) => Currency::USD(a * b),
            (Currency::JPY(a), Currency::JPY(b)) => Currency::JPY(a * b),
            (Currency::RMB(a), Currency::RMB(b)) => Currency::RMB(a * b),
            (_, _) => panic!(),
        }
    }
}

impl Div for Currency {
    type Output = Self;

    fn div(self, other: Currency) -> Currency {
        match (self, other) {
            (Currency::USD(a), Currency::USD(b)) => Currency::USD(a / b),
            (Currency::JPY(a), Currency::JPY(b)) => Currency::JPY(a / b),
            (Currency::RMB(a), Currency::RMB(b)) => Currency::RMB(a / b),
            (_, _) => panic!(),
        }
    }
}

impl Money {
    pub fn new(amount: i64) -> Self {
        Money(amount)
    }
}

impl Add for Money {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.0 + other.0)
    }
}

impl Sub for Money {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.0 - other.0)
    }
}

impl Mul for Money {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::new(self.0 * other.0)
    }
}

impl Div for Money {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self::new(self.0 / other.0)
    }
}

impl AddAssign for Money {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0
    }
}

impl SubAssign for Money {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0
    }
}

impl MulAssign for Money {
    fn mul_assign(&mut self, other: Self) {
        self.0 *= other.0
    }
}

impl DivAssign for Money {
    fn div_assign(&mut self, other: Self) {
        self.0 /= other.0
    }
}

impl From<i8> for Money {
    fn from(amount: i8) -> Self {
        Self(amount.into())
    }
}

impl From<i16> for Money {
    fn from(amount: i16) -> Self {
        Self(amount.into())
    }
}

impl From<i32> for Money {
    fn from(amount: i32) -> Self {
        Self(amount.into())
    }
}

impl From<i64> for Money {
    fn from(amount: i64) -> Self {
        Self(amount)
    }
}

impl From<u8> for Money {
    fn from(amount: u8) -> Self {
        Self(amount.into())
    }
}

impl From<u16> for Money {
    fn from(amount: u16) -> Self {
        Self(amount.into())
    }
}

impl From<u32> for Money {
    fn from(amount: u32) -> Self {
        Self(amount.into())
    }
}

impl From<u64> for Money {
    fn from(amount: u64) -> Self {
        Self(amount as i64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_u64() {
        let num: u64 = 30;
        let res = Money::from(num);
        assert_eq!(res, Money::new(30));
    }

    #[test]
    #[should_panic]
    fn from_u64_max() {
        let num: u64 = u64::MAX;
        let res = Money::from(num);
        assert_eq!(res, Money::new(30));
    }

    #[test]
    fn add_money() {
        let left = Money::new(10);
        let right = Money::new(10);
        let new = left + right;
        assert_eq!(new, Money::new(20));
    }

    #[test]
    fn add_currencies() {
        let left = Currency::JPY(Money::new(10));
        let right = Currency::JPY(Money::new(10));
        let new = left + right;
        assert_eq!(new, Currency::JPY(Money::new(20)));
    }
}
