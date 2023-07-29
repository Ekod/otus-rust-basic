use hw_5::math::primitives::{SignedCounter, UnsignedCounter};

#[test]
fn default_signed_counter_works() {
    let signed_counter = SignedCounter::default();
    let expected = 0;
    let actual = signed_counter.get_counter();

    assert_eq!(expected, actual);
}

#[test]
fn next_signed_works() {
    let mut signed_counter = SignedCounter::default();
    let mut expected = 1;
    let mut actual = signed_counter.next_counter();

    assert_eq!(expected, actual);

    expected = 2;
    actual = signed_counter.next_counter();

    assert_eq!(expected, actual);
}

#[test]
fn prev_signed_works() {
    let mut signed_counter = SignedCounter::default();
    let mut expected = 0;
    let mut actual = signed_counter.prev_counter();

    assert_eq!(expected, actual);

    signed_counter.next_counter();
    signed_counter.next_counter();

    expected = 1;
    actual = signed_counter.prev_counter();

    assert_eq!(expected, actual);
}

#[test]
fn default_unsigned_counter_works() {
    let unsigned_counter = UnsignedCounter::default();
    let expected = 0;
    let actual = unsigned_counter.get_counter();

    assert_eq!(expected, actual);
}

#[test]
fn next_unsigned_works() {
    let mut unsigned_counter = UnsignedCounter::default();
    let mut expected = 1;
    let mut actual = unsigned_counter.next_counter();

    assert_eq!(expected, actual);

    expected = 2;
    actual = unsigned_counter.next_counter();

    assert_eq!(expected, actual);
}
