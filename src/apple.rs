// (c) 2024 Pepijn Bakker
// This code is licensed under the AGPL-3.0 license (see LICENSE for details)
use crate::Point;

/// Represents an apple in the game
pub struct Apple {
    position: Point
}

impl Apple {
    pub fn new(pos: Point) -> Self {
        Apple { position: pos }
    }
    
    /// Move the apple to a new position
    pub fn move_to(&mut self, point: Point) {
        self.position = point;
    }

    /// Get the position of the apple
    pub fn position(&self) -> Point {
        self.position
    }
}