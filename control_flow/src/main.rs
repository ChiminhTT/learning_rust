fn main() {

    // If
    let x = 5;
    if x == 5 {
        println!("x is five");
    } else if x == 6 {
        println!("x is six");
    } else {
        println!("x is not five nor six");
    }

    let y = if x == 5 {
        10
    } else {
        15
    }; // y: i32

    // Infinite Loop
    // loop {
    //     println!("Loop forever")
    // }

    // While loop {
    let mut x = 5;
    let mut done = false;

    while !done {
        x += x - 3;

        println!("{}", x);

        if x % 5 == 0 {
            done = true;
        }
    }

    // Enumerate on range
    for (i, j) in (5..10).enumerate() {
        println!("i = {} and j = {}", i, j);
    }

    // Enumerate on iterator
    let lines = "hello\nworld".lines();
    for (linenumber, line) in lines.enumerate() {
        println!("{} : {}", linenumber, line);
    }

    // Ending iterion early
    let mut x = 5;
    loop {
        x += x - 3;

        println!("{}", x);

        if x % 5 == 0 {
            break;
        }
    }

    // Loop labels
    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
            if x % 2 ==0 {continue 'outer;} // continues the loop over x
            if y % 2 ==0 {continue 'inner;} // continues the loop over y
            println!("x : {}, y : {}", x, y);
        }
    } 
}
