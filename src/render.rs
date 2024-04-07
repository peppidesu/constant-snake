use std::io::{self, Stdout};

use crossterm::{style, ExecutableCommand as _};

use crate::{GameConfig, Point, Result, SnakeChange};

pub struct Renderer {
    stdout: Stdout,
    screen_width: u16,
    screen_height: u16,
}

fn move_to_point(pos: Point) -> crossterm::cursor::MoveTo {
    return crossterm::cursor::MoveTo(
        (pos.x as u16 + 1) * 2, 
        pos.y as u16 + 1
    );
}

fn move_to(x: u16, y: u16) -> crossterm::cursor::MoveTo {
    return crossterm::cursor::MoveTo(x, y);
}

impl Renderer {
    pub fn new(config: &GameConfig) -> Self {
        Self {
            stdout: io::stdout(),
            screen_width: config.screen_width as u16,
            screen_height: config.screen_height as u16,
        }
    }

    fn print_at(&mut self, pos: Point, text: &str) -> Result<()> {        
        self.stdout.execute(move_to_point(pos))?;
        print!("{}", text);

        Ok(())
    }

    fn set_color(&mut self, color: style::Color) -> Result<()> {
        self.stdout.execute(style::SetForegroundColor(color))?;
    
        Ok(())
    }

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

    pub fn draw_diff_snake(&mut self, change: SnakeChange) -> Result<()> {        
        self.set_color(style::Color::Green)?;
        self.print_at(change.cell_added, "[]")?;
        
        if let Some(pos) = change.cell_removed {
            self.print_at(pos, "  ")?;
        }                   

        Ok(())
    }

    pub fn draw_diff_apple(&mut self, change: Point) -> Result<()> {
        self.set_color(style::Color::Red)?;
        self.print_at(change, "()")?;

        Ok(())
    }

    pub fn reset_cursor(&mut self) -> Result<()> {
        self.stdout.execute(move_to(0, 0))?;
        
        Ok(())
    }

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
}






