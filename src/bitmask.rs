// (c) 2024 Pepijn Bakker
// This code is licensed under the AGPL-3.0 license (see LICENSE for details)

use crate::{Point, GameConfig};

/// A bitmask to store the state of each cell in the game.
///
/// Used for collision detection.
pub struct Bitmask {
    mask: Vec<u8>,
    width: usize,    
}

impl Bitmask {
    /// Create a new bitmask with the given configuration.
    pub fn new(config: &GameConfig) -> Self {
        let width = config.screen_width;
        let height = config.screen_height;

        let cell_count = width * height;
        Bitmask {
            mask: vec![0; (cell_count >> 3) + 1],
            width,            
        }
    }

    /// Set the value of the cell at the given point.
    pub fn set(&mut self, point: Point, value: bool) {
        let idx = (point.y as usize) * self.width + point.x as usize;
        let byte_idx = idx >> 3;
        let bit_idx = idx & 0b111;

        if value {
            self.mask[byte_idx] |= 1 << bit_idx;
        } else {
            self.mask[byte_idx] &= !(1 << bit_idx);
        }
    }

    /// Get the value of the cell at the given point.
    pub fn get(&self, point: Point) -> bool {
        let idx = (point.y as usize) * self.width + point.x as usize;
        let byte_idx = idx >> 3;
        let bit_idx = idx & 0b111;

        (self.mask[byte_idx] & (1 << bit_idx)) != 0
    }
}