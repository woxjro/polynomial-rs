use polynomial_rs::term::Term;

#[test]
fn test_term() {
    let term = Term {
        coefficient: 3,
        degree: 2,
    };
    println!("{}", term);
    assert_eq!(term.coefficient, 3);
}

#[test]
fn test_term_add() {
    let term1 = Term {
        coefficient: 3,
        degree: 2,
    };
    let term2 = Term {
        coefficient: 2,
        degree: 2,
    };
    let term3 = term1 + term2;
    println!("{}", term3);
    assert_eq!(term3.coefficient, 5);
}

#[test]
fn test_term_sub() {
    let term1 = Term {
        coefficient: 3,
        degree: 2,
    };
    let term2 = Term {
        coefficient: 2,
        degree: 2,
    };
    let term3 = term1 - term2;
    println!("{}", term3);
    assert_eq!(term3.coefficient, 1);
}

#[test]
fn test_term_mul() {
    let term1 = Term {
        coefficient: 3,
        degree: 2,
    };
    let term2 = Term {
        coefficient: 2,
        degree: 2,
    };
    let term3 = term1 * term2;
    println!("{}", term3);
    assert_eq!(term3.coefficient, 6);
    assert_eq!(term3.degree, 4);
}
