fn main() {
    assert_eq!(4_u32, double_unt32(2));
    assert_eq!(4_u64, double_unt64(2));
    assert_eq!(4_f32, double_float32(2.));
    assert_eq!(4_f64, double_float64(2.));
    assert_eq!(4_f64, int_plus_float_to_float(2, 2.));
    assert_eq!(4_u64, int_plus_float_to_int(2, 2.));
    assert_eq!(4_i64, tuple_sum((2,2)));
    assert_eq!(6_i64, array_sum([2, 2, 2]));
    assert_eq!(8_589_934_590_u64, double_unt64(4_294_967_295_u32));
    assert_eq!(int_plus_float_to_int(10_u32, -5_f32), 5_u64)
}

fn double_unt32(x: u32) -> u32 {
    x + x
}

fn double_unt64(x: u32) -> u64 {
    x as u64 + x as u64
}

fn double_float32(x: f32) -> f32 {
    x + x
}

fn double_float64(x: f32) -> f64 {
    x as f64 + x as f64
}

fn int_plus_float_to_float(x: u32, y: f32) -> f64 {
    x as f64 + y as f64
}

fn int_plus_float_to_int(x: u32, y: f32) -> u64 {
    (x as f32 + y ) as u64
}

fn tuple_sum(int_tup: (i32, i32)) -> i64 {
    int_tup.0 as i64 + int_tup.1 as i64
}

fn array_sum(int_arr: [i64; 3]) -> i64 {
    let mut result: i64  = 0;
    for num in int_arr.iter(){
        result += num;
    }

    result
}
