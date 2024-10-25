pub fn divide(a: i32, b: i32) -> f32 {
    if b == 0 {
        panic!("Cannot divide by zero");
    }
    (a as f32 / b as f32) as f32
}