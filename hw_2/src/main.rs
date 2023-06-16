fn main() {
    assert_eq!(4 as i32, double_int32(2));
    assert_eq!(4 as i64, double_int64(2));
    assert_eq!(4 as f32, double_float32(2.));
    assert_eq!(4 as f64, double_float64(2.));
    assert_eq!(4 as f64, int_plus_float_to_float(2, 2.));   
    assert_eq!(4 as i64, int_plus_float_to_int(2, 2.));
    assert_eq!(4 as i64, tuple_sum((2,2)));
    assert_eq!(6 as i64, array_sum([2, 2, 2]));
}

fn double_int32(x: i32) -> i32 {
    x + x
}

fn double_int64(x: i32) -> i64 {
    (x + x) as i64
}

fn double_float32(x: f32) -> f32 {
    x + x
}

fn double_float64(x: f32) -> f64 {
    (x + x) as f64
}

fn int_plus_float_to_float(x: i32, y: f32) -> f64 {
    (x as f32 + y) as f64
}

fn int_plus_float_to_int(x: i32, y: f32) -> i64 {
    (x as f32 + y) as i64
}

fn tuple_sum(int_tup: (i32, i32)) -> i64 {
    (int_tup.0 + int_tup.1) as i64
}

fn array_sum(int_arr: [i32; 3]) -> i64 {
    let mut result  = 0;
    for num in int_arr.iter(){
        result += num;
    }

    result as i64
}
