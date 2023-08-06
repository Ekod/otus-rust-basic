use hw_3::math::tuple::{default_pair, pair_vector_sum, pair_scalar_sum};

#[cfg(test)]
mod tuple_tests {
    use super::*;

    #[test]
    fn default_pair_works() {
        let want = (0, 0);
        let got = default_pair();

        assert_eq!(want, got);
    }

    #[test]
    fn pair_vector_sum_works() {
        let test_pair_1 = (1, 2);
        let test_pair_2 = (3, 4);

        let want = (4, 6);
        let got = pair_vector_sum(test_pair_1, test_pair_2);

        assert_eq!(want, got);
    }

    #[test]
    #[should_panic]
    fn pair_vector_sum_overflows() {
        let test_pair_1 = (i32::MAX, i32::MAX);
        let test_pair_2 = (i32::MAX, i32::MAX);

        pair_vector_sum(test_pair_1, test_pair_2);
    }

    #[test]
    fn pair_scalar_sum_works() {
        let test_pair_1 = (1, 2);
        let test_pair_2 = (3, 4);

        let want = 10;
        let got = pair_scalar_sum(test_pair_1, test_pair_2);

        assert_eq!(want, got);
    }

    #[test]
    #[should_panic]
    fn pair_scalar_sum_overflows() {
        let test_pair_1 = (i32::MAX, i32::MAX);
        let test_pair_2 = (i32::MAX, i32::MAX);

        pair_scalar_sum(test_pair_1, test_pair_2);
    }
}
