pub trait DefaultForMul: Sized {
    fn default() -> Self;
}

macro_rules! default_impl_for_mul {
    ($($t:ty)*) => ($(
        impl DefaultForMul for $t {
            fn default() -> $t {
                1 as $t
            }
        }
    )*)
}

default_impl_for_mul! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }
