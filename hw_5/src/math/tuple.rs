#[derive(Default)]
pub struct Pair(i32, i32);

impl Pair {
    pub fn with_values(val_1: i32, val_2: i32) -> Self {
        Self(val_1, val_2)
    }

    pub fn vector_sum(&self, rhs: &Pair) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }

    pub fn scalar_sum(&self, rhs: &Pair) -> i32 {
        self.0 + self.1 + rhs.0 + rhs.1
    }

    pub fn get_fields(&self) -> (i32, i32) {
        (self.0, self.1)
    }
}
