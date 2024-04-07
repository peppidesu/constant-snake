// (c) 2024 Pepijn Bakker
// This code is licensed under the AGPL-3.0 license (see LICENSE for details)

use crate::{Bitmask, GameConfig, Point};

/// Represents a cell in the snake. Works similarly to a linked list.
struct Cell {
    pos: Point,
    next: usize, 
    idx: usize,
}

/// A buffer to store the cells of the snake, with a head and tail pointer.
/// This is used to avoid allocating and deallocating memory for each cell.
struct CellBuf {
    buf: Vec<Cell>,
    head: usize,
    tail: usize,
}

impl CellBuf {
    pub fn len(&self) -> usize {
        self.buf.len()
    }
}

/// Used to communicate changes in the snake to the renderer.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SnakeChange {
    pub cell_added: Point,
    pub cell_removed: Option<Point>,
}

/// Used to communicate the result of a simulation step to the game loop.
#[derive(Debug, PartialEq)]
pub enum SnakeStepResult {
    Ok(SnakeChange),
    GameOver,    
    AppleEaten(SnakeChange),
}

/// Represents the snake in the game.
pub struct Snake {
    body: CellBuf,
    mask: Bitmask,
    screen_width: usize,
    direction: Point,
}

impl Snake {
    /// Create a new snake at the given position. 
    /// Snake starts off with length 1.
    pub fn new(pos: Point, config: &GameConfig) -> Self {
        let cell_count = config.screen_width * config.screen_height;
        let mut body = CellBuf {
            buf: Vec::with_capacity(cell_count),
            head: 0,
            tail: 0,
        };

        let cell = Cell {
            pos,
            next: 0,
            idx: 0,
        };
        body.buf.push(cell);        
        

        Snake {
            body,
            mask: Bitmask::new(config),
            screen_width: config.screen_width,
            direction: Point::new(1, 0),
        }
    }

    /// Move the snake forward by one cell. The position of the new cell
    /// is passed as an argument to avoid recalculation.
    fn move_forward(&mut self, pos: Point) {
        self.mask.set(self.body.buf[self.body.tail].pos, false);
        self.mask.set(pos, true);

        self.body.buf[self.body.tail].pos = pos;
        
        self.body.head = self.body.tail;
        self.body.tail = self.body.buf[self.body.tail].next;
    }

    /// Grow the snake by one cell in the current direction. The position of the
    /// new cell is passed as an argument to avoid recalculation.
    fn grow(&mut self, pos: Point) {        
        let new_cell = Cell {
            pos,
            next: self.body.tail,
            idx: self.body.buf.len(),
        };
        self.mask.set(pos, true);
        
        self.body.buf[self.body.head].next = new_cell.idx;
        self.body.head = new_cell.idx;
        self.body.buf.push(new_cell);
    }

    /// Check if the snake overlaps with the given position.
    pub fn overlaps(&self, pos: Point) -> bool {
        self.mask.get(pos)
    }

    /// Perform a simulation step for the snake. Returns a SnakeStepResult
    pub fn step(&mut self, apple: Point) -> SnakeStepResult {
        let mut new_pos = self.body.buf[self.body.head].pos + self.direction;
        let old_tail = self.body.buf[self.body.tail].pos;
        new_pos.x = (new_pos.x + self.screen_width as i32) % self.screen_width as i32;
        new_pos.y = (new_pos.y + self.screen_width as i32) % self.screen_width as i32;
        
        if self.mask.get(new_pos) {
            return SnakeStepResult::GameOver;
        }

        if new_pos == apple {
            self.grow(new_pos);
            return SnakeStepResult::AppleEaten(SnakeChange {
                cell_added: new_pos,
                cell_removed: None,
            });
        } 

        self.move_forward(new_pos);
        return SnakeStepResult::Ok(SnakeChange {
            cell_added: new_pos,
            cell_removed: Some(old_tail),
        });
    }

    /// Set the direction of the snake. The snake cannot move in the opposite
    /// direction of its current direction.
    pub fn set_direction(&mut self, direction: Point) {
        if self.direction != -direction {
            self.direction = direction;
        }
    }
    
    /// Get the current length of the snake.
    pub fn len(&self) -> usize {
        self.body.len()
    }
}

