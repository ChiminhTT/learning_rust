fn main() {
    // Move semantics
    let v = vec![1, 2, 3];
    let v2 = v;

    // Copy type
    let v = 1; // i32 implement 'Copy' trait
    let v2 = v;
    println!("v is: {}", v);
}
