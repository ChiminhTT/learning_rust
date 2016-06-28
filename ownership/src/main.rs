fn main() {
    // Move semantics
    let v = vec![1, 2, 3];
    let v2 = v;

    // Copy type
    let v = 1; // i32 implement 'Copy' trait
    let v2 = v;
    println!("v is: {}", v); // You'll note that v still has ownership to smth; due to the copy trait v did not have to transfert its ownership
}
