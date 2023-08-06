use hw_3::math::vec::{default_vec3, vec3_scalar_sum, vec3_vector_sum};

#[cfg(test)]
mod vec_tests {
    use super::*;
    use hw_3::math::vec::Vec3;

    #[test]
    fn default_vec3_works() {
        let want: Vec3 = [0; 3];
        let got = default_vec3();

        assert_eq!(want, got);
    }

    #[test]
    fn vec3_vector_sum_works() {
        let test_vec_1: Vec3 = [1, 2, 3];
        let test_vec_2: Vec3 = [1, 2, 3];

        let want: Vec3 = [2, 4, 6];
        let got = vec3_vector_sum(test_vec_1, test_vec_2);

        assert_eq!(want, got);
    }

    #[test]
    #[should_panic]
    fn vec3_vector_sum_panics() {
        let test_vec_1: Vec3 = [i32::MAX, i32::MAX, i32::MAX];
        let test_vec_2: Vec3 = [i32::MAX, i32::MAX, i32::MAX];

        vec3_vector_sum(test_vec_1, test_vec_2);
    }

    #[test]
    fn vec3_scalar_sum_works() {
        let test_vec_1: Vec3 = [1, 2, 3];
        let test_vec_2: Vec3 = [1, 2, 3];

        let want = 12;
        let got = vec3_scalar_sum(test_vec_1, test_vec_2);

        assert_eq!(want, got);
    }

    #[test]
    #[should_panic]
    fn vec3_scalar_sum_panics() {
        let test_vec_1: Vec3 = [i32::MAX, i32::MAX, i32::MAX];
        let test_vec_2: Vec3 = [i32::MAX, i32::MAX, i32::MAX];

        vec3_scalar_sum(test_vec_1, test_vec_2);
    }
}
