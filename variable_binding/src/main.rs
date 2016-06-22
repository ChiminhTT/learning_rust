fn main() {
    // Basic binding
    let x = 5;

    // Patterns
    let (x, y) = (1, 2);

    // Type annotation
    let x: i32 = 5; // Explicit declaration

    // Mutability
    let mut x = 5;
    x = 10;

    // Scope and shadowing
    let x: i32 = 17;
    {
        let y: i32 = 3;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y);

}
