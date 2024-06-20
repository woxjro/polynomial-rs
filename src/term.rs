use std::ops::{Add, Div, Mul, Neg, Sub};
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Term<T = i32, U = i32>
where
    T: Copy
        + Clone
        + Add<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Sub<Output = T>
        + Neg<Output = T>
        + std::cmp::PartialEq
        + std::fmt::Debug,
    U: Copy
        + Clone
        + Add<Output = U>
        + Mul<Output = U>
        + Div<Output = U>
        + Sub<Output = U>
        + Neg<Output = U>
        + std::cmp::PartialEq
        + std::fmt::Debug,
{
    pub coefficient: T,
    pub degree: U,
}

impl Add for Term {
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

impl Sub for Term {
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

impl Mul for Term {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            coefficient: self.coefficient * other.coefficient,
            degree: self.degree + other.degree,
        }
    }
}

impl Div for Term {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            coefficient: self.coefficient / other.coefficient,
            degree: self.degree - other.degree,
        }
    }
}

impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = if self.degree == 0 {
            if self.coefficient == 1 {
                "1".to_string()
            } else if self.coefficient == -1 {
                "-1".to_string()
            } else {
                format!("{}", self.coefficient)
            }
        } else if self.degree == 1 {
            if self.coefficient == 1 {
                "x".to_string()
            } else if self.coefficient == -1 {
                "-x".to_string()
            } else {
                format!("{}x", self.coefficient)
            }
        } else {
            format!(
                "{}x^{}",
                if self.coefficient == 1 {
                    "".to_string()
                } else if self.coefficient == -1 {
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
