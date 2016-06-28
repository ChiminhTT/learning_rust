#[derive(Clone, Copy)]
pub struct Rectangle<T> {
    x: T,
    y: T,
    width: T,
    height: T,
}

impl<T> Rectangle<T> {
    pub fn new(x: T, y: T, width: T, height: T) -> Rectangle<T> {
        return Rectangle {
            x: x,
            y: y,
            width: width,
            height: height,
        };
    }

    pub fn translate(&mut self, x: T, y: T) {
        self.x = x;
        self.y = y;
    }

    pub fn resize(&mut self, width: T, height: T) {
        self.width = width;
        self.height = height;
    }

    pub fn get_position(self) -> (T, T) {
        (self.x, self.y)
    }

    pub fn get_dimensions(&self) -> (&T, &T) {
        (&self.width, &self.height)
    }
}

impl<T: PartialEq> Rectangle<T> {
    pub fn is_square(&self) -> bool {
        self.width == self.height
    }
}
