fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n: u32 = args[1].parse().expect("Please provide a valid number");
    let fact = factorial(n);
    println!("{}", fact);
}

fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}
