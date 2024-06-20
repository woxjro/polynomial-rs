# polynomial-rs

A simple polynomial library for Rust.

```rust
use polynomial_rs::polynomial::Polynomial;

fn main() {
    // adding two polynomials
    // x^2 + 1
    let lhs = Polynomial::from(vec![1, 0, 1]);
    // x + 1
    let rhs = Polynomial::from(vec![1, 1]);

    // x^2 + x + 2
    println!("{}", lhs + rhs);

    // subtracting two polynomials
    // x^2 - x
    let lhs = Polynomial::from(vec![1, 0, 1]);
    // x - 1
    let rhs = Polynomial::from(vec![1, -1]);

    // x^2 -1x + 2
    println!("{}", lhs - rhs);

    // multiplying two polynomials
    // x^4 + x^3 + x^2 + x + 1
    let lhs = Polynomial::from((0..100).map(|_| 1).collect::<Vec<i32>>());
    // x + 1
    let rhs = Polynomial::from(vec![1, -1]);

    // x^100 - 1
    println!("{}", lhs * rhs);
}
```
