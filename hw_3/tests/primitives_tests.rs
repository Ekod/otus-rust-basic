use hw_3::math::primitives::{
    default_signed_counter, default_unsigned_counter, next_signed, next_unsigned, prev_signed,
};

#[cfg(test)]
mod primitives_tests {
    use super::*;

    #[test]
    fn default_signed_counter_works() {
        let want = 0;
        let got = default_signed_counter();

        assert_eq!(want, got);
    }

    #[test]
    fn default_unsigned_counter_works() {
        let want = 0;
        let got = default_unsigned_counter();

        assert_eq!(want, got);
    }

    #[test]
    fn next_signed_works() {
        let want = 2;
        let got = next_signed(1);

        assert_eq!(want, got);
    }

    #[test]
    fn next_unsigned_works() {
        let want = 2;
        let got = next_unsigned(1);

        assert_eq!(want, got);
    }

    #[test]
    fn prev_signed_works() {
        let want = 0;
        let got = prev_signed(1);

        assert_eq!(want, got);
    }
}
