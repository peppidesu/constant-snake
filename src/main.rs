// (c) 2024 Pepijn Bakker
// This code is licensed under the AGPL-3.0 license (see LICENSE for details)

use constant_snake::*;

use crossterm::{
    event::{poll, read, Event, KeyCode}, 
    terminal::{disable_raw_mode, enable_raw_mode}
};
use rand::Rng;
use std::time::Duration;

struct Game {
    snake: Snake,
    apple: Apple,
    rng: rand::rngs::ThreadRng,
    renderer: Renderer,
    config: GameConfig,
}

enum GameStepResult {
    Ok(SnakeChange),
    AppleEaten(SnakeChange, Point),
    GameOver,
    Win,
}

impl Game {
    fn new() -> Self {      
        let config = GameConfig::default();

        let result = Game {
            snake: Snake::new(Point::new(0, 0), &config),
            apple: Apple::new(Point::new(1, 0)),
            rng: rand::thread_rng(),
            renderer: Renderer::new(&config),
            config,
        };        
        result
    }

    fn step(&mut self) -> GameStepResult {
        let result = self.snake.step(self.apple.position());
    
        match result {
            SnakeStepResult::AppleEaten(change) => {
                if self.snake.len() == self.config.total_cells() {                    
                    GameStepResult::Win
                } else {
                    self.randomize_apple_pos();
                    self.apple.move_to(self.apple.position());                

                    GameStepResult::AppleEaten(change, self.apple.position())                
                }            
            }
            SnakeStepResult::GameOver => GameStepResult::GameOver,            
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
    
}



fn main() -> Result<()> {
    enable_raw_mode()?;
    
    let mut game = Game::new();    

    game.renderer.draw_first_frame(game.apple.position())?;    
    
    let sleep_duration = Duration::from_millis(game.config.snake_speed);
    
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
                game.renderer.draw_diff_snake(change)?;
            }
            GameStepResult::AppleEaten(change, apple_pos) => {
                game.renderer.draw_diff_snake(change)?;
                game.renderer.draw_diff_apple(apple_pos)?;
            }
            GameStepResult::GameOver => {
                game.renderer.draw_game_over()?;
                break;
            }
            GameStepResult::Win => {
                game.renderer.draw_win()?;
                break;
            }
        }

        game.renderer.reset_cursor()?;
    }

    disable_raw_mode()?;
    
    Ok(())
}
