pub fn calculate(a: i32, operator: String, b: i32) {
    use crate::operations::*;
    match operator.as_str() {
        "+" => print!("{}",addition::add(a, b)),
        "-" => print!("{}",subtraction::minus(a, b)),
        "*" => print!("{}",multiplication::multiply(a, b)),
        "/" => print!("{}",division::divide(a, b)),
        _ => panic!("Invalid operator"),
    }
}
