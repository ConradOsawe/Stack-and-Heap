// stack and heap
// The size and location of every data is known at compile time, and the data is fixed


fn main() {
    let result = do_math(3, 4);
    println!("Result: {}", result);
}

fn do_math(a:i32, b:i32) -> i32 {
    let sum = add(a, b);
    let diffrence = substract(a, b);
    let product = mutltiply(a, b);
    let quotient = divide(a, b);
    sum + diffrence + product + quotient
}

fn add(a: i32, b:i32) -> i32 {
    a + b
}

fn substract(a: i32, b:i32) -> i32 {
    a - b
}

fn mutltiply (a: i32, b:i32) -> i32 {
    a * b
}

fn divide(a: i32, b:i32) -> i32 {
a / b
}