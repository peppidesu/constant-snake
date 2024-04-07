// (c) 2024 Pepijn Bakker
// This code is licensed under the AGPL-3.0 license (see LICENSE for details)

use std::io::{self, Stdout};

use crossterm::{style, ExecutableCommand as _};

use crate::{GameConfig, Point, Result, SnakeChange};

pub struct Renderer {
    stdout: Stdout,
    screen_width: u16,
    screen_height: u16,
}

/// Move to the given position, in snake coordinates.
fn move_to_point(pos: Point) -> crossterm::cursor::MoveTo {
    return crossterm::cursor::MoveTo(
        (pos.x as u16 + 1) * 2, 
        pos.y as u16 + 1
    );
}

/// Move to the given position, in screen coordinates.
fn move_to(x: u16, y: u16) -> crossterm::cursor::MoveTo {
    return crossterm::cursor::MoveTo(x, y);
}

impl Renderer {
    /// Create a new renderer with the given configuration.
    pub fn new(config: &GameConfig) -> Self {
        Self {
            stdout: io::stdout(),
            screen_width: config.screen_width as u16,
            screen_height: config.screen_height as u16,
        }
    }

    /// Print text at the given position.
    fn print_at(&mut self, pos: Point, text: &str) -> Result<()> {        
        self.stdout.execute(move_to_point(pos))?;
        print!("{}", text);

        Ok(())
    }

    /// Set the color of the renderer.
    fn set_color(&mut self, color: style::Color) -> Result<()> {
        self.stdout.execute(style::SetForegroundColor(color))?;
    
        Ok(())
    }

    /// Draw the borders of the screen.
    fn draw_borders(&mut self) -> Result<()> {
        for x in 0..self.screen_width {
            self.stdout.execute(move_to(x * 2 + 2, 0))?;            
            print!("--");
            self.stdout.execute(move_to(x * 2 + 2, self.screen_height + 1))?;
            print!("--");            
        }

        for y in 0..self.screen_height {
            self.stdout.execute(move_to(0, y + 1))?;            
            print!("|");
            self.stdout.execute(move_to(self.screen_width * 2 + 2, y + 1))?;            
            print!("|");
        }
        Ok(())
    }

    /// Draw the changes in the snake body to the screen.
    pub fn draw_diff_snake(&mut self, change: SnakeChange) -> Result<()> {        
        self.set_color(style::Color::Green)?;
        self.print_at(change.cell_added, "[]")?;
        
        if let Some(pos) = change.cell_removed {
            self.print_at(pos, "  ")?;
        }                   

        Ok(())
    }

    /// Draw the changes in the apple to the screen.
    pub fn draw_diff_apple(&mut self, change: Point) -> Result<()> {
        self.set_color(style::Color::Red)?;
        self.print_at(change, "()")?;

        Ok(())
    }

    /// Reset the cursor to the top left corner of the screen.
    pub fn reset_cursor(&mut self) -> Result<()> {
        self.stdout.execute(move_to(0, 0))?;
        
        Ok(())
    }

    /// Draw the first frame of the game.
    pub fn draw_first_frame(&mut self, apple: Point) -> Result<()> {
        self.stdout.execute(
            crossterm::terminal::Clear(
                crossterm::terminal::ClearType::All
            )
        ).unwrap();

        self.draw_borders()?;
        self.draw_diff_apple(apple)?;
        self.reset_cursor()?;
        
        Ok(())
    }

    /// Draw the win message to the screen.
    pub fn draw_win(&mut self) -> Result<()> {
        self.set_color(style::Color::Green)?;
        self.stdout.execute(move_to(0, self.screen_height + 2))?;

        println!("You win!");        
        
        Ok(())
    }

    /// Draw the game over message to the screen.
    pub fn draw_game_over(&mut self) -> Result<()> {
        self.set_color(style::Color::Red)?;
        self.stdout.execute(move_to(0, self.screen_height + 2))?;

        println!("Game Over!");        
        
        Ok(())
    }
}






