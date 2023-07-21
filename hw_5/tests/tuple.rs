use hw_5::math::tuple::Pair;

#[test]
fn default_works() {
    let expected = (0, 0);
    let actual = Pair::default();

    assert_eq!(expected, actual.get_fields());
}

#[test]
fn vector_sum_works() {
    let pair_1 = Pair::with_values(1, 2);
    let pair_2 = Pair::with_values(3, 4);

    let expected = (4, 6);
    let actual = pair_1.vector_sum(&pair_2);

    assert_eq!(expected, actual.get_fields());
}

#[test]
fn scalar_sum_works() {
    let pair_1 = Pair::with_values(1, 2);
    let pair_2 = Pair::with_values(3, 4);

    let expected = 10;
    let actual = pair_1.scalar_sum(&pair_2);

    assert_eq!(expected, actual);
}
