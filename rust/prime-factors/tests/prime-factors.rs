use prime_factors::factors;

#[test]
fn test_no_factors() {
    assert_eq!(factors(1), vec![]);
}

#[test]
fn test_prime_number() {
    assert_eq!(factors(2), vec![2]);
}

#[test]
fn test_square_of_a_prime() {
    assert_eq!(factors(9), vec![3, 3]);
}
