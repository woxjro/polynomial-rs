use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::term::Term;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Polynomial<T = i32, U = i32>
where
    T: Copy
        + Clone
        + Add<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Sub<Output = T>
        + Neg<Output = T>
        + std::cmp::PartialEq
        + std::cmp::PartialOrd
        + std::fmt::Debug,
    U: Copy
        + Clone
        + Add<Output = U>
        + Mul<Output = U>
        + Div<Output = U>
        + Sub<Output = U>
        + Neg<Output = U>
        + std::cmp::PartialEq
        + std::cmp::PartialOrd
        + std::fmt::Debug,
{
    pub terms: Vec<Term<T, U>>,
}

impl<T> From<Vec<T>> for Polynomial<T, i32>
where
    T: Copy
        + Clone
        + Add<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Sub<Output = T>
        + Neg<Output = T>
        + std::cmp::PartialEq
        + std::cmp::PartialOrd
        + std::fmt::Debug,
{
    fn from(coefficients: Vec<T>) -> Self {
        let mut terms = vec![];
        let length = coefficients.len();
        for (i, coefficient) in coefficients.iter().enumerate() {
            terms.push(Term {
                coefficient: coefficient.to_owned(),
                degree: (length - i - 1) as i32,
            });
        }
        Self { terms }
    }
}

impl<T, U> Polynomial<T, U>
where
    T: Copy
        + Clone
        + Add<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Sub<Output = T>
        + Neg<Output = T>
        + std::cmp::PartialEq
        + std::cmp::Ord
        + std::fmt::Debug,
    U: Copy
        + Clone
        + Add<Output = U>
        + Mul<Output = U>
        + Div<Output = U>
        + Sub<Output = U>
        + Neg<Output = U>
        + std::cmp::PartialEq
        + std::cmp::Ord
        + std::fmt::Debug,
{
    pub fn sort(&mut self) {
        self.terms.sort_by(|a, b| b.degree.cmp(&a.degree));
    }
}

impl Polynomial {
    pub fn substitute(&self, x: i32) -> i32 {
        let mut result = 0;
        for term in self.terms.iter() {
            result += term.coefficient * x.pow(term.degree as u32);
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
        self.sort();
    }
}

impl std::fmt::Display for Polynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut sorted_polynomial = self.clone();
        sorted_polynomial.uniqify();

        let mut s = String::new();

        let mut iter = sorted_polynomial
            .terms
            .iter()
            .filter(|term| term.coefficient != 0)
            .peekable();

        while let Some(term) = iter.next() {
            if let Some(next_term) = iter.peek() {
                s.push_str(&format!(
                    "{term}{}",
                    if next_term.coefficient > 0 { "+" } else { "" }
                ));
            } else {
                s.push_str(&format!("{term}"));
            }
        }
        write!(f, "{}", s)
    }
}

impl Add for Polynomial {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut terms = self.terms.clone();
        for term in other.terms {
            let mut found = false;
            for t in terms.iter_mut() {
                if t.degree == term.degree {
                    *t = *t + term;
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

impl Sub for Polynomial {
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
                    coefficient: -term.coefficient,
                    degree: term.degree,
                });
            }
        }
        let mut polynomial = Self { terms };
        polynomial.sort();
        polynomial
    }
}

impl Mul for Polynomial {
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
