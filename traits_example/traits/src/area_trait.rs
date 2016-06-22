pub trait HasArea {
    fn area(&self) -> f64;
}

pub fn print_area<T: HasArea>(shape: T) {
    println!("This shape has an area of {}", shape.area());
}
