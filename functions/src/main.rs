fn main() {
    let num = add_one(12);
    print_number(num);
    print_sum(3, 5);

    // Expressions vs Statements
    let mut y = 5;
    let x = (y = 6); // x has the value '()', and not '6'

    // Function pointers
    let f: fn(i32) -> i32 = add_one;
    let six = f(5);

    let x: i32 = diverges();
}

fn print_number(x: i32) {
    println!("x is: {}", x);
}

fn print_sum(x: i32, y: i32) {
    println!("sum is: {}", x + y);
}

fn add_one(number: i32) -> i32 {
    return number + 1;
}

// Diverging functions (look for ! in founction's return)
fn diverges() -> ! {
    panic!("This function never returns");
}
