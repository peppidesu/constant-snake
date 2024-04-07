// (c) 2024 Pepijn Bakker
// This code is licensed under the AGPL-3.0 license (see LICENSE for details)

/// Stores config parameters for the game
pub struct GameConfig {
    pub screen_width: usize,
    pub screen_height: usize,
    pub snake_speed: u64,
}

impl GameConfig {
    pub fn new(width: usize, height: usize, speed: u64) -> Self {
        GameConfig {
            screen_width: width,
            screen_height: height,
            snake_speed: speed,
        }
    }
}

impl Default for GameConfig {
    fn default() -> Self {
        GameConfig {
            screen_width: 20,
            screen_height: 20,
            snake_speed: 150,
        }
    }
}

impl GameConfig {
    /// Get the total number of cells in the screen.
    pub fn total_cells(&self) -> usize {
        self.screen_width * self.screen_height
    }
}