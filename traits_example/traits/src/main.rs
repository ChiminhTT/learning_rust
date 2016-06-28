mod area_trait;
mod circle;
mod square;
mod rectangle;

use circle::Circle;
use square::Square;
use area_trait::print_area;
use rectangle::Rectangle;
use std::fmt::Debug;

fn foo<T: Clone, K: Clone + Debug>(x: T, y: K) {
    x.clone();
    y.clone();
    println!("{:?}", y);
}

fn bar<T, K>(x: T, y: K)
    where T: Clone,
          K: Clone + Debug
{
    x.clone();
    y.clone();
    println!("{:?}", y);
}

trait ConvertTo<Output> {
    fn convert(&self) -> Output;
}

impl ConvertTo<i64> for i32 {
    fn convert(&self) -> i64 {
        *self as i64
    }
}

// can be called with T == i32
fn normal<T: ConvertTo<i64>>(x: &T) -> i64 {
    x.convert()
}

fn inverse<T>() -> T
    where i32: ConvertTo<T>
{
    42.convert()
}

trait Foo {
    fn is_valid(&self) -> bool;

    fn is_invalid(&self) -> bool {
        !self.is_valid()
    }
}

struct UseDefault;

impl Foo for UseDefault {
    fn is_valid(&self) -> bool {
        println!("Called UseDefault.is_valid.");
        true
    }
}

struct OverrideDefault;

impl Foo for OverrideDefault {
    fn is_valid(&self) -> bool {
        println!("Called OverrideDefault.is_valid.");
        true
    }

    fn is_invalid(&self) -> bool {
        println!("Called OverrideDefault.is_invalid!");
        true // overrides the expected value of is_invalid
    }
}

trait ParentTrait {
    fn foo(&self);
}

trait ChildTrait: ParentTrait {
    fn foobar(&self);
}

struct Baz;
impl ParentTrait for Baz {
    fn foo(&self) {
        println!("Parent");
    }
}

impl ChildTrait for Baz {
    fn foobar(&self) {
        println!("foobar");
    }
}

#[derive(Debug)]
struct Derived;

fn main() {

    let c = Circle::new(0.0f64, 0.0f64, 1.0f64);
    let s = Square::new(0.0f64, 0.0f64, 1.0f64);

    print_area(c);
    print_area(s);

    // Trait bounds on generic struct
    let mut r = Rectangle::new(5, 5, 47, 47);
    assert!(r.is_square());
    let oldWidth = r.get_position().0;
    r.resize(oldWidth, 42);

    assert!(!r.is_square());
    assert!(r.get_dimensions() == (&5, &42));

    // Where clause
    foo("Hello", "world");
    bar("Hello", "world");

    // Default behavior
    let default = UseDefault;
    assert!(!default.is_invalid()); // prints "Called UseDefault.is_valid."

    let over = OverrideDefault;
    assert!(over.is_invalid()); // prints "Called UseDefault.is_valid."

    // Deriving
    println!("{:?}", Derived);
}
