use rand::Rng;
use raylib::prelude::*;
use std::sync::{Arc, Mutex};

use super::{snake, Args, Commands};

pub struct Renderer {
    pub snake: Arc<Mutex<snake::Snake>>,
    pub food: snake::Point,
    pub width: u32,
    pub height: u32,
    pub died: bool,
    
    snake_counter: u32,

    args: Args,
    rl: RaylibHandle,
    thread: RaylibThread,
}

impl Renderer {
    pub fn new(snake: Arc<Mutex<snake::Snake>>, args: Args) -> Renderer {
        let width = args.width;
        let height = args.height;

        let (rl, thread) = raylib::init()
            .size(width as i32 * 20 + 40, height as i32 * 20 + 40)
            .title("Snake")
            .build();

        let food = snake::Point::new(width as i32 / 4 * 3, height as i32 / 2);

        Renderer {
            snake,
            food,
            width,
            height,
            died: false,

            snake_counter: 0,

            args,
            rl,
            thread,
        }
    }

    pub fn run(&mut self) {
        self.rl.set_target_fps(self.args.fps);
        self.prepare();

        while !self.rl.window_should_close() {
            self.update();
            self.draw();
        }
    }

    pub fn prepare(&mut self) {
        self.died = false;

        self.food.x = self.width as i32 / 4 * 3;
        self.food.y = self.height as i32 / 2;

        let mut snake = self.snake.lock().unwrap();
        snake.prepare();
    }

    pub fn update(&mut self) {
        if self.died {
            if self.rl.is_key_pressed(KeyboardKey::KEY_ENTER)
                || self.rl.is_key_pressed(KeyboardKey::KEY_SPACE)
            {
                self.prepare();
            }
        } else {
            match self.args.cmd {
                Commands::Play => {
                    let mut snake = self.snake.lock().unwrap();
                    if self.snake_counter % 5 == 0 {
                        snake.move_forward();
                    }
                    self.snake_counter += 1;

                    if snake.body[0] == self.food {
                        snake.grow();
                        // Randomize new food position from 0 to width and 0 to height
                        self.food = snake::Point::new(
                            rand::thread_rng().gen_range(0..self.width as i32),
                            rand::thread_rng().gen_range(0..self.height as i32),
                        );
                    }

                    if self.rl.is_key_pressed(KeyboardKey::KEY_UP) {
                        snake.turn(snake::Direction::Up);
                    } else if self.rl.is_key_pressed(KeyboardKey::KEY_DOWN) {
                        snake.turn(snake::Direction::Down);
                    } else if self.rl.is_key_pressed(KeyboardKey::KEY_LEFT) {
                        snake.turn(snake::Direction::Left);
                    } else if self.rl.is_key_pressed(KeyboardKey::KEY_RIGHT) {
                        snake.turn(snake::Direction::Right);
                    }

                    if snake.collides_with_wall(self.width, self.height)
                        || snake.collides_with_self()
                    {
                        self.died = true;
                    }
                }
            }
        }
    }

    pub fn draw(&mut self) {
        let mut d = self.rl.begin_drawing(&self.thread);
        d.clear_background(Color::WHITE);

        let wall_color = Color::new(44, 39, 49, 255);
        // Draw walls
        for x in 0..self.width + 2 {
            d.draw_rectangle(x as i32 * 20, 0, 20, 20, wall_color);
            d.draw_rectangle(
                x as i32 * 20,
                (self.height - 1) as i32 * 20 + 40,
                20,
                20,
                wall_color,
            );
        }
        for y in 1..self.height - 1 + 2 {
            d.draw_rectangle(0, y as i32 * 20, 20, 20, wall_color);
            d.draw_rectangle(
                (self.width - 1) as i32 * 20 + 40,
                y as i32 * 20,
                20,
                20,
                wall_color,
            );
        }

        // Draw floor
        let floor_color_1 = Color::new(73, 67, 81, 255);
        let floor_color_2 = Color::new(68, 62, 76, 255);
        for x in 0..self.width {
            for y in 0..self.height {
                let color = if (x + y) % 2 == 0 {
                    floor_color_1
                } else {
                    floor_color_2
                };
                d.draw_rectangle(x as i32 * 20 + 20, y as i32 * 20 + 20, 20, 20, color);
            }
        }

        // Draw snake
        let snake = self.snake.lock().unwrap();
        let body = &snake.body;
        let start_color = Color::new(79, 124, 246, 255);
        let end_color = Color::new(51, 96, 203, 255);
        for (i, point) in body.iter().enumerate() {
            let color = Renderer::lerp_color(start_color, end_color, i as f32 / body.len() as f32);
            d.draw_rectangle(point.x * 20 + 20, point.y * 20 + 20, 20, 20, color);
        }

        // Draw food
        let food_color = Color::new(231, 71, 29, 255);
        d.draw_rectangle(
            self.food.x * 20 + 20,
            self.food.y * 20 + 20,
            20,
            20,
            food_color,
        );

        if self.died {
            d.draw_rectangle(
                0,
                0,
                self.width as i32 * 20 + 40,
                self.height as i32 * 20 + 40,
                Color::new(0, 0, 0, 100),
            );
            d.draw_text("Snake died :(", 40, 40, 20, Color::WHITE);
            d.draw_text("Press Enter to restart", 40, 60, 20, Color::WHITE);
        }
    }

    fn lerp_color(start: Color, end: Color, t: f32) -> Color {
        let r = start.r as f32 + (end.r as f32 - start.r as f32) * t;
        let g = start.g as f32 + (end.g as f32 - start.g as f32) * t;
        let b = start.b as f32 + (end.b as f32 - start.b as f32) * t;
        let a = start.a as f32 + (end.a as f32 - start.a as f32) * t;
        Color::new(r as u8, g as u8, b as u8, a as u8)
    }
}
