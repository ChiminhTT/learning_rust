fn main() {
    let str = frob("this is ", "a test");
    println!("{}", str);
}

fn frob<'a, 'b>(s: &'a str, t: &'a str) -> &'a str {
    "Test ok"
}
