use std::env;

mod operations;


mod calc;
mod test;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <num1> <operator> <num2>", args[0]);
        std::process::exit(1);
    }

    let num1: i32 = args[1].parse().expect("Invalid number");
    let operator: String = args[2].parse().expect("Invalid operator");
    let num2: i32 = args[3].parse().expect("Invalid number");

    calc::calculate(num1, operator, num2);
    
}
