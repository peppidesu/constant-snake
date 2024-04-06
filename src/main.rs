mod point;
mod apple;
mod snake;
mod config;
mod bitmask;

use point::*;
use apple::*;
use snake::*;
use config::*;
use bitmask::*;

use std::io::Stdout;
use std::{io::stdout, time::Duration};

use crossterm::terminal::enable_raw_mode;

use rand::Rng;

use crossterm::{cursor, style, ExecutableCommand};

use crossterm::event::{poll, read, Event, KeyCode};

struct Game {
    snake: Snake,
    apple: Apple,
    config: GameConfig,
    rng: rand::rngs::ThreadRng,
    stdout: Stdout,
}
fn move_to(pos: Point) -> crossterm::cursor::MoveTo {
    return crossterm::cursor::MoveTo((pos.x as u16 + 1) * 2, pos.y as u16 + 1);
}

enum GameStepResult {
    Ok(SnakeChange),
    AppleEaten(SnakeChange, Point),
    GameOver,
    Win,
}

impl Game {
    fn new(config: GameConfig) -> Self {
        let snake = Snake::new(Point::new(0, 0), &config);
        let apple = Apple::new(Point::new(1, 0));
        let result = Game {
            snake,
            apple,
            config,
            rng: rand::thread_rng(),
            stdout: stdout(),
        };        
        result
    }

    fn step(&mut self) -> GameStepResult {
        let result = self.snake.step(self.apple.position());
    
        match result {
            SnakeStepResult::AppleEaten(change) => {
                if self.snake.len() == (self.config.screen_width as usize * self.config.screen_height as usize) {                    
                    println!("You win!");

                    GameStepResult::Win
                } else {
                    self.randomize_apple_pos();
                    self.apple.move_to(self.apple.position());                

                    GameStepResult::AppleEaten(change, self.apple.position())                
                }            
            }
            SnakeStepResult::GameOver => {
                println!("Game Over!");
                
                GameStepResult::GameOver
            }
            SnakeStepResult::Ok(change) => GameStepResult::Ok(change.clone())                        
        }                
    }
    fn randomize_apple_pos(&mut self) {
        while self.snake.overlaps(self.apple.position()) {
            let x = self.rng.gen_range(0..self.config.screen_width) as i32;
            let y = self.rng.gen_range(0..self.config.screen_height) as i32;
            self.apple.move_to(Point::new(x, y));
        }
    }
    fn draw_first(&mut self) -> std::io::Result<()> {
        self.stdout.execute(
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
        )
        .unwrap();
        

        let pos = self.apple.position();
        
        self.stdout
            .execute(move_to(pos))?
            .execute(style::SetForegroundColor(style::Color::Red))?;            
        
        print!("()");

        self.stdout.execute(style::SetForegroundColor(style::Color::Black))?;
        
        // borders
        for x in 0..self.config.screen_width {
            self.stdout.execute(cursor::MoveTo((x as u16 + 1) * 2, 0))?;                
            print!("--");
            self.stdout.execute(cursor::MoveTo(
                (x as u16 + 1) * 2, 
                self.config.screen_height as u16 + 1
            ))?;
            print!("--");
        }

        for y in 0..self.config.screen_height {
            self.stdout.execute(cursor::MoveTo(0, y as u16 + 1))?;                
            print!("|");
            self.stdout.execute(cursor::MoveTo(
                (self.config.screen_width as u16 + 1) * 2, 
                y as u16 + 1
            ))?;                
            print!("|");
        }

        self.stdout
            .execute(cursor::MoveTo(0, self.config.screen_height as u16 + 2))?;
        
        Ok(())
    }
    fn draw_diff_snake(&mut self, change: SnakeChange) -> std::io::Result<()> {        
        self.stdout
            .execute(cursor::MoveTo((
                change.cell_added.x as u16 + 1) * 2, 
                change.cell_added.y as u16 + 1
            ))?
            .execute(style::SetForegroundColor(style::Color::Green))?;
        print!("[]");        
        if let Some(pos) = change.cell_removed {
            self.stdout.execute(move_to(pos))?;                
            print!("  ");
        }               
    
        Ok(())
    }
    fn draw_diff_apple(&mut self, change: Point) -> std::io::Result<()> {
        self.stdout
            .execute(move_to(change))?             
            .execute(style::SetForegroundColor(style::Color::Red))?;
        print!("()");
    
        Ok(())
    }
    fn reset_cursor(&mut self) -> std::io::Result<()> {
        self.stdout
            .execute(cursor::MoveTo(0, self.config.screen_height as u16 + 2))?;
        
        Ok(())
    }
}



fn main() -> std::io::Result<()> {
    enable_raw_mode()?;
    let config = GameConfig {
        screen_width: 20,
        screen_height: 20,
        snake_speed: 150,
    };

    let sleep_duration = Duration::from_millis(config.snake_speed);

    let mut game = Game::new(config);
    game.draw_first()?;
    loop {
        let t1 = std::time::Instant::now();
            
        if poll(sleep_duration)? {            

            match read()? {
                Event::Key(event) => {                    
                    match event.code {
                        KeyCode::Up => {
                            game.snake.set_direction(Point::new(0, -1));
                        }
                        KeyCode::Down => {
                            game.snake.set_direction(Point::new(0, 1));
                        }
                        KeyCode::Left => {
                            game.snake.set_direction(Point::new(-1, 0));
                        }
                        KeyCode::Right => {
                            game.snake.set_direction(Point::new(1, 0));
                        }
                        KeyCode::Esc => {
                            break;
                        }                        
                        _ => {}
                    }
                },
                _ => {}
            }
        }

        let elapsed = t1.elapsed();
        if elapsed < sleep_duration {
            std::thread::sleep(sleep_duration - elapsed);
        }        

        match game.step() {
            GameStepResult::Ok(change) => {
                game.draw_diff_snake(change)?;
            }
            GameStepResult::AppleEaten(change, apple_pos) => {
                game.draw_diff_snake(change)?;
                game.draw_diff_apple(apple_pos)?;
            }
            GameStepResult::GameOver => {
                break;
            }
            GameStepResult::Win => {
                break;
            }
        }

        game.reset_cursor()?;
    }

    Ok(())
}
