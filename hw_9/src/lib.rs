mod macros;

use macros::DefaultForMul;
use std::ops::{Add, AddAssign, Mul, MulAssign};

#[derive(PartialEq, Debug)]
struct Matrix<T, const SIZE: usize> {
    elements: [T; SIZE],
}

impl<T: Copy + Default + AddAssign, const SIZE: usize> Matrix<T, SIZE> {
    #[allow(dead_code)]
    fn sum_all_elements(&self) -> T {
        let mut sum = T::default();

        for val in self.elements.iter() {
            sum += *val
        }

        sum
    }
}

impl<T: Copy + DefaultForMul + MulAssign, const SIZE: usize> Matrix<T, SIZE> {
    #[allow(dead_code)]
    fn mul_all_elements(&self) -> T {
        let mut sum = T::default();

        for val in self.elements.iter() {
            sum *= *val
        }

        sum
    }
}

impl<T: Add<Output = T> + Copy, const SIZE: usize> Matrix<T, SIZE> {
    #[allow(dead_code)]
    fn add_to_elements(&mut self, element: T) {
        for i in 0..SIZE {
            self.elements[i] = self.elements[i] + element
        }
    }
}

impl<T: Mul<Output = T> + Copy, const SIZE: usize> Matrix<T, SIZE> {
    #[allow(dead_code)]
    fn mul_with_elements(&mut self, element: T) {
        for i in 0..SIZE {
            self.elements[i] = self.elements[i] * element
        }
    }
}

struct Matrices<'a, 'b, T, const SET_SIZE: usize, const SIZE: usize> {
    elements: &'a [&'b Matrix<T, SIZE>; SET_SIZE],
}

impl<'a, 'b, T: Copy, const SET_SIZE: usize, const SIZE: usize>
    Matrices<'a, 'b, T, SET_SIZE, SIZE>
{
    #[allow(dead_code)]
    fn get_by_idx(&self, idx: usize) -> &'b Matrix<T, SIZE> {
        self.elements.get(idx).unwrap()
    }
}

impl<
        'a,
        'b,
        T: Add<Output = T> + Copy + Default + AddAssign,
        const SET_SIZE: usize,
        const SIZE: usize,
    > Matrices<'a, 'b, T, SIZE, SET_SIZE>
{
    #[allow(dead_code)]
    fn sum_all_values(&self) -> T {
        let mut sum = T::default();
        for val in self.elements {
            sum += val.sum_all_elements();
        }

        sum
    }
}

impl<
        'a,
        'b,
        T: Add<Output = T> + Copy + DefaultForMul + MulAssign,
        const SET_SIZE: usize,
        const SIZE: usize,
    > Matrices<'a, 'b, T, SIZE, SET_SIZE>
{
    #[allow(dead_code)]
    fn mul_all_values(&self) -> T {
        let mut sum = T::default();
        for val in self.elements {
            sum *= val.mul_all_elements()
        }

        sum
    }
}

#[cfg(test)]
mod matrix_tests {
    use super::*;

    #[test]
    fn add_to_elements_works() {
        let mut m = Matrix::<i32, 3> {
            elements: [1, 2, 3],
        };

        let expected = [2, 3, 4];
        m.add_to_elements(1);
        let actual = m.elements;

        assert_eq!(actual, expected)
    }

    #[test]
    fn mul_with_elements_works() {
        let mut m = Matrix::<i32, 3> {
            elements: [1, 2, 3],
        };

        let expected = [2, 4, 6];
        m.mul_with_elements(2);
        let actual = m.elements;

        assert_eq!(actual, expected)
    }

    #[test]
    fn sum_all_elements_works() {
        let m = Matrix::<i32, 3> {
            elements: [1, 2, 3],
        };

        let expected = 6;
        let actual = m.sum_all_elements();

        assert_eq!(actual, expected)
    }

    #[test]
    fn mul_all_elements_works() {
        let m = Matrix::<i32, 3> {
            elements: [1, 2, 3],
        };

        let expected = 6;
        let actual = m.mul_all_elements();

        assert_eq!(actual, expected)
    }
}

#[cfg(test)]
mod matrices_tests {
    use crate::{Matrices, Matrix};

    #[test]
    fn get_by_idx_works() {
        let m = Matrix::<i32, 3> {
            elements: [1, 2, 3],
        };

        let m2 = Matrix::<i32, 3> {
            elements: [2, 3, 4],
        };

        let mut actual: Option<&Matrix<i32, 3>> = None;
        let mut actual2: Option<&Matrix<i32, 3>> = None;

        {
            let ms = Matrices::<i32, 2, 3> { elements: &[&m, &m2] };
            actual = Some(ms.get_by_idx(0));
            actual2 = Some(ms.get_by_idx(1));
        }

        assert_eq!(actual, Some(&m));
        assert_eq!(actual2, Some(&m2));
    }

    #[test]
    fn sum_all_values_works() {
        let m = Matrix::<i32, 3> {
            elements: [1, 2, 3],
        };

        let m2 = Matrix::<i32, 3> {
            elements: [1, 2, 3],
        };

        let ms = Matrices::<i32, 2, 3> {
            elements: &[&m, &m2],
        };

        let expected = 12;
        let actual = ms.sum_all_values();

        assert_eq!(actual, expected)
    }

    #[test]
    fn mul_all_values_works() {
        let m = Matrix::<i32, 3> {
            elements: [1, 2, 3],
        };

        let m2 = Matrix::<i32, 3> {
            elements: [1, 2, 3],
        };

        let ms = Matrices::<i32, 2, 3> {
            elements: &[&m, &m2],
        };

        let expected = 36;
        let actual = ms.mul_all_values();

        assert_eq!(actual, expected)
    }
}
