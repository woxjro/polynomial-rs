use crate::term::Term;
use num_traits::{
    identities::{One, Zero},
    NumAssignOps, NumCast, NumOps, Pow,
};
use std::collections::HashMap;
use std::ops::{Add, Mul, Neg, Sub};

#[derive(Debug, Clone, PartialEq)]
pub struct Polynomial<T, U> {
    pub terms: Vec<Term<T, U>>,
}

impl<T, U> From<Vec<T>> for Polynomial<T, U>
where
    T: Clone + NumOps + std::fmt::Debug,
    U: NumOps + NumCast + std::fmt::Debug,
{
    fn from(coefficients: Vec<T>) -> Self {
        let mut terms = vec![];
        let length = coefficients.len();
        for (i, coefficient) in coefficients.iter().enumerate() {
            terms.push(Term {
                coefficient: coefficient.to_owned(),
                degree: NumCast::from(length - i - 1).unwrap(),
            });
        }
        Self { terms }
    }
}

impl<T, U> Polynomial<T, U>
where
    U: std::cmp::Ord + std::fmt::Debug,
{
    pub fn sort(&mut self) {
        self.terms.sort_by(|a, b| b.degree.cmp(&a.degree));
    }
}

impl<T, U> Polynomial<T, U>
where
    T: Copy + NumOps + NumAssignOps + Zero + Pow<U, Output = T>,
    U: Copy + std::hash::Hash + std::cmp::Eq,
{
    pub fn substitute(&self, x: T) -> T {
        let mut result = T::zero();
        for term in self.terms.iter() {
            result += term.coefficient * Pow::pow(x, term.degree);
        }
        result
    }

    pub fn uniqify(&mut self) {
        let mut hp = HashMap::new();
        let mut terms = vec![];
        for term in self.terms.clone() {
            hp.get_mut(&term.degree)
                .map(|t| *t += term.coefficient)
                .unwrap_or_else(|| {
                    hp.insert(term.degree, term.coefficient);
                });
        }

        for (degree, coefficient) in hp {
            terms.push(Term {
                coefficient,
                degree,
            });
        }

        self.terms = terms;
    }
}

impl<T, U> std::fmt::Display for Polynomial<T, U>
where
    T: Copy
        + Neg
        + NumAssignOps
        + NumCast
        + NumOps
        + One
        + PartialEq
        + Pow<U, Output = T>
        + Zero
        + std::cmp::PartialOrd
        + std::fmt::Display,
    U: Copy
        + One
        + PartialEq
        + Zero
        + std::cmp::Eq
        + std::cmp::Ord
        + std::fmt::Debug
        + std::fmt::Display
        + std::hash::Hash,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut sorted_polynomial = self.clone();
        sorted_polynomial.uniqify();
        sorted_polynomial.sort();

        let mut s = String::new();

        let mut iter = sorted_polynomial
            .terms
            .iter()
            .filter(|term| term.coefficient != T::zero())
            .peekable();

        while let Some(term) = iter.next() {
            if let Some(next_term) = iter.peek() {
                s.push_str(&format!(
                    "{term}{}",
                    if next_term.coefficient > T::zero() {
                        "+"
                    } else {
                        ""
                    }
                ));
            } else {
                s.push_str(&format!("{term}"));
            }
        }
        write!(f, "{}", s)
    }
}

impl<T, U> Add for Polynomial<T, U>
where
    T: Clone + Add<Output = T> + std::cmp::PartialEq + std::fmt::Debug,
    U: Clone + Add<Output = U> + std::cmp::Ord + std::cmp::PartialEq + std::fmt::Debug,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut terms = self.terms.clone();
        for term in other.terms {
            let mut found = false;
            for t in terms.iter_mut() {
                if t.degree == term.degree {
                    *t = t.to_owned() + term.to_owned();
                    found = true;
                    break;
                }
            }
            if !found {
                terms.push(term);
            }
        }

        let mut polynomial = Self { terms };
        polynomial.sort();
        polynomial
    }
}

impl<T, U> Sub for Polynomial<T, U>
where
    T: Copy
        + One
        + NumAssignOps
        + NumOps
        + Pow<U, Output = T>
        + Sub<Output = T>
        + Zero
        + std::cmp::PartialEq
        + std::fmt::Debug,
    U: Copy
        + Sub<Output = U>
        + std::cmp::Eq
        + std::cmp::Ord
        + std::cmp::PartialEq
        + std::fmt::Debug
        + std::hash::Hash,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut lhs = self.clone();
        let mut rhs = other.clone();
        lhs.uniqify();
        rhs.uniqify();

        let mut terms = self.terms.clone();
        for term in other.terms {
            let mut found = false;
            for t in terms.iter_mut() {
                if t.degree == term.degree {
                    *t = *t - term;
                    found = true;
                    break;
                }
            }
            if !found {
                terms.push(Term {
                    coefficient: term.coefficient * (T::zero() - T::one()),
                    degree: term.degree,
                });
            }
        }
        let mut polynomial = Self { terms };
        polynomial.sort();
        polynomial
    }
}

impl<T, U> Mul for Polynomial<T, U>
where
    T: Copy
        + Add<Output = T>
        + Mul<Output = T>
        + NumAssignOps
        + NumOps
        + Pow<U, Output = T>
        + Zero
        + std::cmp::PartialEq
        + std::fmt::Debug,
    U: Copy
        + Add<Output = U>
        + Mul<Output = U>
        + std::cmp::Eq
        + std::cmp::Ord
        + std::cmp::PartialEq
        + std::fmt::Debug
        + std::hash::Hash,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut terms = vec![];
        for term1 in self.terms {
            for term2 in other.terms.clone() {
                terms.push(term1 * term2);
            }
        }
        let mut polynomial = Self { terms };
        polynomial.uniqify();
        polynomial.sort();
        polynomial
    }
}
