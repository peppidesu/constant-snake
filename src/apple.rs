use crate::Point;
pub struct Apple {
    position: Point
}



impl Apple {
    pub fn new(pos: Point) -> Self {
        Apple {
            position: pos
        }
    }

    pub fn move_to(&mut self, point: Point) {
        self.position = point;
    }

    pub fn position(&self) -> Point {
        self.position
    }
}