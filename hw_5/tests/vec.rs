use hw_5::math::vec::Vec3;

#[test]
fn default_works() {
    let expected = [0, 0, 0];
    let actual = Vec3::default();

    assert_eq!(expected, actual.get_vec());
}

#[test]
fn vector_sum_works() {
    let vec_1 = Vec3::with_value([1, 2, 3]);
    let vec_2 = Vec3::with_value([1, 2, 3]);

    let expected = [2, 4, 6];
    let actual = vec_1.vector_sum(&vec_2);

    assert_eq!(expected, actual.get_vec());
}

#[test]
fn scalar_sum_works() {
    let vec_1 = Vec3::with_value([1, 2, 3]);
    let vec_2 = Vec3::with_value([1, 2, 3]);

    let expected = 12;
    let actual = vec_1.scalar_sum(&vec_2);

    assert_eq!(expected, actual);
}
