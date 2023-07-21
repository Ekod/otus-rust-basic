const VEC3_LEN: usize = 3;

#[derive(Default)]
pub struct Vec3 {
    arr: [i32; VEC3_LEN],
}

impl Vec3 {
    pub fn with_value(val: [i32; VEC3_LEN]) -> Self {
        Self { arr: val }
    }

    pub fn vector_sum(&self, rhs: &Vec3) -> Self {
        let mut result = Self::default();

        for i in 0..3 {
            result.arr[i] = self.arr[i] + rhs.arr[i];
        }

        result
    }

    pub fn scalar_sum(&self, rhs: &Vec3) -> i32 {
        let mut result = 0;

        for i in 0..VEC3_LEN {
            result += self.arr[i] + rhs.arr[i];
        }

        result
    }

    pub fn get_vec(&self) -> [i32; VEC3_LEN] {
        self.arr
    }
}
