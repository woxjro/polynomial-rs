use polynomial_rs::polynomial::Polynomial;

fn main() {
    // adding two polynomials
    // x^2 + 1
    let lhs: Polynomial<i32, u32> = Polynomial::from(vec![1, 0, 1]);
    // x + 1
    let rhs: Polynomial<i32, u32> = Polynomial::from(vec![1, 1]);

    // x^2 + x + 2
    println!("{}", lhs + rhs);

    // subtracting two polynomials
    // x^2 - 1
    let lhs: Polynomial<i32, u32> = Polynomial::from(vec![1, 0, 1]);
    // x - 1
    let rhs: Polynomial<i32, u32> = Polynomial::from(vec![1, -1]);

    // x^2 - x + 2
    println!("{}", lhs - rhs);

    // multiplying two polynomials
    // x^99 + x^98 + ... + x^1 + x^0
    let lhs: Polynomial<i32, u32> = Polynomial::from((0..100).map(|_| 1).collect::<Vec<i32>>());
    // x - 1
    let rhs: Polynomial<i32, u32> = Polynomial::from(vec![1, -1]);

    // x^100 - 1
    println!("{}", lhs * rhs);
}
