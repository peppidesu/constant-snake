use crate::{Bitmask, GameConfig, Point};

struct Cell {
    pos: Point,
    next: usize,
    idx: usize,
}

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



#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SnakeChange {
    pub cell_added: Point,
    pub cell_removed: Option<Point>,
}

#[derive(Debug, PartialEq)]
pub enum SnakeStepResult {
    Ok(SnakeChange),
    GameOver,    
    AppleEaten(SnakeChange),
}

pub struct Snake {
    body: CellBuf,
    mask: Bitmask,
    screen_width: usize,
    direction: Point,
}

impl Snake {
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

    fn move_forward(&mut self, pos: Point) {   

        self.mask.set(self.body.buf[self.body.tail].pos, false);
        self.mask.set(pos, true);

        self.body.buf[self.body.tail].pos = pos;
        
        self.body.head = self.body.tail;
        self.body.tail = self.body.buf[self.body.tail].next;
    }

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
    pub fn overlaps(&self, pos: Point) -> bool {
        self.mask.get(pos)
    }

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

    pub fn set_direction(&mut self, direction: Point) {
        if self.direction != -direction {
            self.direction = direction;
        }
    }
    
    pub fn len(&self) -> usize {
        self.body.len()
    }
}

