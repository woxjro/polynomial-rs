use polynomial_rs::polynomial::Polynomial;

#[test]
fn test_polynomial() {
    // x^2 + 2x + 3
    let polynomial = Polynomial::from(vec![1, 2, 3]);
    println!("{}", polynomial);
    assert_eq!(polynomial.terms[0].coefficient, 1);
    assert_eq!(polynomial.terms[1].coefficient, 2);
    assert_eq!(polynomial.terms[2].coefficient, 3);
    assert_eq!(polynomial.terms[0].degree, 2);
    assert_eq!(polynomial.terms[1].degree, 1);
    assert_eq!(polynomial.terms[2].degree, 0);
    assert_eq!(polynomial.substitute(2), 11);
}

#[test]
fn test_polynomial_add() {
    //        x^2 + 2x + 3
    let polynomial1 = Polynomial::from(vec![1, 2, 3]);
    // x^3 + 2x^2 + 3x + 4
    let polynomial2 = Polynomial::from(vec![1, 2, 3, 4]);
    let polynomial3 = polynomial1 + polynomial2;

    assert_eq!(polynomial3.terms[0].coefficient, 1);
    assert_eq!(polynomial3.terms[1].coefficient, 3);
    assert_eq!(polynomial3.terms[2].coefficient, 5);
    assert_eq!(polynomial3.terms[3].coefficient, 7);
    assert_eq!(polynomial3.terms[0].degree, 3);
    assert_eq!(polynomial3.terms[1].degree, 2);
    assert_eq!(polynomial3.terms[2].degree, 1);
    assert_eq!(polynomial3.terms[3].degree, 0);
}

#[test]
fn test_polynomial_sub() {
    //        x^2 + 2x + 3
    let polynomial1 = Polynomial::from(vec![1, 2, 3]);
    // x^3 + 2x^2 + 3x + 4
    let polynomial2 = Polynomial::from(vec![1, 2, 3, 4]);
    let polynomial3 = polynomial1 - polynomial2;

    println!("{}", polynomial3);
    assert_eq!(polynomial3.terms[0].coefficient, -1);
    assert_eq!(polynomial3.terms[1].coefficient, -1);
    assert_eq!(polynomial3.terms[2].coefficient, -1);
    assert_eq!(polynomial3.terms[3].coefficient, -1);
    assert_eq!(polynomial3.terms[0].degree, 3);
    assert_eq!(polynomial3.terms[1].degree, 2);
    assert_eq!(polynomial3.terms[2].degree, 1);
    assert_eq!(polynomial3.terms[3].degree, 0);
}

#[test]
fn test_polynomial_mul() {
    // x^1 + 2x^0
    let polynomial1 = Polynomial::from(vec![1, 2]);
    // 3x^1 + 4x^0
    let polynomial2 = Polynomial::from(vec![3, 4]);
    // 3x^2 + 10x^1 + 8x^0
    let polynomial3 = polynomial1 * polynomial2;

    println!("{}", polynomial3);
    assert_eq!(polynomial3.terms[0].coefficient, 3);
    assert_eq!(polynomial3.terms[1].coefficient, 10);
    assert_eq!(polynomial3.terms[2].coefficient, 8);
    assert_eq!(polynomial3.terms[0].degree, 2);
    assert_eq!(polynomial3.terms[1].degree, 1);
    assert_eq!(polynomial3.terms[2].degree, 0);
}
