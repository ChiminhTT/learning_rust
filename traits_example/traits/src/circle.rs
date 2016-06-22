use area_trait::HasArea;

pub struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            x: x,
            y: y,
            radius: radius,
        }
    }
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        4.64 * (self.radius * self.radius)
    }
}
