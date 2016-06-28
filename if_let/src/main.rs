fn main() {

    let opt = None;

    // If let
    if let Some(unwrapped_opt) = opt {
        foo(unwrapped_opt);
    } else {
        bar();
    }
    // While let
    let mut v = vec![1, 3, 5, 7, 11];
    loop {
        match v.pop() {
            Some(x) => println!("{}", x),
            None => break,
        }
    }

    let mut v = vec![1, 3, 5, 7, 11];
    while let Some(x) = v.pop() {
        println!("{}", x);
    }
}

fn foo(x: i32) {}
fn bar() {
    println!("Optional was empty");
}
