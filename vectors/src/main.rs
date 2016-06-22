fn main() {
    let v = vec![0; 10];

    let v = vec![1, 2, 3, 4, 5]; // v: Vec<
    println!("The third element of v is {}", v[2]);
    // Non-panicking out-of-bounds access
    match v.get(7) {
        Some(x) => println!("Item 7 is {}", x),
        None => println!("Sorry, this vector is too short"),
    }

    // Iterating
    let mut v = vec![1, 2, 3, 4, 5];
    for i in &v {
        println!("A reference to {}", i);
    }

    for i in &mut v {
        println!("A mutable reference to {}", i);
    }

    let v = vec![1, 2, 3, 4, 5];
    for i in v {
        println!("Take ownership of the vector and its element {}", i);
    }

}
