use github_actions_demo::calculator;

fn main() {
    println!("GitHub Actions Demo - Rust Calculator");
    println!("2 + 3 = {}", calculator::add(2, 3));
    println!("10 / 3 = {:?}", calculator::divide(10, 3));
    println!("fib(10) = {}", calculator::fibonacci(10));
}
