use std::ops::{Add, Div, Mul, Sub};
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

impl std::fmt::Display for Term<i32, i32> {
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
