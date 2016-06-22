use area_trait::HasArea;

pub struct Square {
    x: f64,
    y: f64,
    side: f64,
}

impl Square {
    pub fn new(x: f64, y: f64, side: f64) -> Square {
        Square {
            x: x,
            y: y,
            side: side,
        }
    }
}

impl HasArea for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}
