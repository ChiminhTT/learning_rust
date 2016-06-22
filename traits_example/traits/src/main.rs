mod area_trait;
mod circle;
mod square;
mod rectangle;

use circle::Circle;
use square::Square;
use area_trait::print_area;
use rectangle::Rectangle;

fn main() {

    let c = Circle::new(0.0f64, 0.0f64, 1.0f64);
    let s = Square::new(0.0f64, 0.0f64, 1.0f64);

    print_area(c);
    print_area(s);

    // Trait bounds on generic struct
    let mut r = Rectangle::new(5, 5, 47, 47);
    assert!(r.is_square());
    {
        let r2 = r;
        let position = r2.get_position();
        r.resize(*position.0, 42);
    }

    assert!(!r.is_square());
    assert!(r.get_dimensions() == (&5, &42));
}
