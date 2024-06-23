use num_traits::cast::NumCast;
use num_traits::identities::{One, Zero};
use std::ops::{Add, Div, Mul, Neg, Sub};
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Term<T, U> {
    pub coefficient: T,
    pub degree: U,
}

impl<T, U> Add for Term<T, U>
where
    T: Add<Output = T> + std::cmp::PartialEq + std::fmt::Debug,
    U: Add<Output = U> + std::cmp::PartialEq + std::fmt::Debug,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.degree == other.degree {
            Self {
                coefficient: self.coefficient + other.coefficient,
                degree: self.degree,
            }
        } else {
            panic!("Cannot add terms with different degrees");
        }
    }
}

impl<T, U> Sub for Term<T, U>
where
    T: Sub<Output = T> + std::cmp::PartialEq + std::fmt::Debug,
    U: Sub<Output = U> + std::cmp::PartialEq + std::fmt::Debug,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if self.degree == other.degree {
            Self {
                coefficient: self.coefficient - other.coefficient,
                degree: self.degree,
            }
        } else {
            panic!("Cannot subtract terms with different degrees");
        }
    }
}

impl<T, U> Mul for Term<T, U>
where
    T: Add<Output = T> + Mul<Output = T> + std::cmp::PartialEq + std::fmt::Debug,
    U: Add<Output = U> + Mul<Output = U> + std::cmp::PartialEq + std::fmt::Debug,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            coefficient: self.coefficient * other.coefficient,
            degree: self.degree + other.degree,
        }
    }
}

impl<T, U> Div for Term<T, U>
where
    T: Sub<Output = T> + Div<Output = T> + std::cmp::PartialEq + std::fmt::Debug,
    U: Sub<Output = U> + Div<Output = U> + std::cmp::PartialEq + std::fmt::Debug,
{
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            coefficient: self.coefficient / other.coefficient,
            degree: self.degree - other.degree,
        }
    }
}

impl<T, U> std::fmt::Display for Term<T, U>
where
    T: One + Neg + NumCast + PartialEq + std::fmt::Display,
    U: Zero + One + PartialEq + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = if self.degree.is_zero() {
            if self.coefficient.is_one() {
                "1".to_string()
            } else if self.coefficient == NumCast::from(-1).unwrap() {
                "-1".to_string()
            } else {
                format!("{}", self.coefficient)
            }
        } else if self.degree.is_one() {
            if self.coefficient.is_one() {
                "x".to_string()
            } else if self.coefficient == NumCast::from(-1).unwrap() {
                "-x".to_string()
            } else {
                format!("{}x", self.coefficient)
            }
        } else {
            format!(
                "{}x^{}",
                if self.coefficient.is_one() {
                    "".to_string()
                } else if self.coefficient == NumCast::from(-1).unwrap() {
                    "-".to_string()
                } else {
                    self.coefficient.to_string()
                },
                self.degree
            )
        };
        write!(f, "{}", s)
    }
}
