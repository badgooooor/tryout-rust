mod input;
mod operation;

fn main() {
    // Get input
    let a = input::get_int();
    let b = input::get_int();
    
    // Select operation.
    let op = input::get_int();

    let result = match op {
        1 => operation::add(a, b),
        2 => operation::subtract(a, b),
        3 => operation::multiply(a, b),
        4 => operation::divide(a, b),
        _ => 0
    };

    // Print out
    println!("Result : {}", result);
}