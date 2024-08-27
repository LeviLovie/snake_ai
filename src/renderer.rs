use raylib::prelude::*;
use std::sync::{Arc, Mutex};

use super::{snake, Args, Commands, SnakeStyle};

pub struct Renderer {
    pub snake: Arc<Mutex<snake::Snake>>,
    pub width: u32,
    pub height: u32,
    pub died: bool,
    pub restart: bool,

    input_queue: Vec<snake::Direction>,
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

        Renderer {
            snake,
            width,
            height,
            died: false,
            restart: false,

            input_queue: Vec::new(),
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

        self.input_queue.clear();

        let mut snake = self.snake.lock().unwrap();
        snake.prepare();
    }

    pub fn update(&mut self) {
        if self.restart {
            self.prepare();
            self.restart = false;
        }
        if self.died {
            if self.rl.is_key_pressed(KeyboardKey::KEY_ENTER)
                || self.rl.is_key_pressed(KeyboardKey::KEY_SPACE)
            {
                self.restart = true;
            }
        } else {
            let mut snake = self.snake.lock().unwrap();
            let snake_update_time;
            if self.args.fps >= 30 {
                snake_update_time = 5 * (self.args.fps / 30);
            } else {
                snake_update_time = 1;
            }
            if self.snake_counter % snake_update_time == 0 {
                if !self.input_queue.is_empty() {
                    snake.turn(self.input_queue.pop().unwrap());
                }

                snake.update();
            }
            self.snake_counter += 1;

            if self.rl.is_key_pressed(KeyboardKey::KEY_R) {
                self.restart = true;
            }

            match self.args.cmd {
                Commands::Play => {
                    if self.rl.is_key_pressed(KeyboardKey::KEY_UP) {
                        self.input_queue.push(snake::Direction::Up);
                        snake.start();
                    } else if self.rl.is_key_pressed(KeyboardKey::KEY_DOWN) {
                        self.input_queue.push(snake::Direction::Down);
                        snake.start();
                    } else if self.rl.is_key_pressed(KeyboardKey::KEY_LEFT) {
                        self.input_queue.push(snake::Direction::Left);
                        snake.start();
                    } else if self.rl.is_key_pressed(KeyboardKey::KEY_RIGHT) {
                        self.input_queue.push(snake::Direction::Right);
                        snake.start();
                    }
                }
            }

            if snake.collides_with_wall(self.width, self.height) || snake.collides_with_self() {
                self.died = true;
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

        match self.args.style {
            SnakeStyle::Block => {
                for (i, point) in body.iter().enumerate() {
                    let color =
                        Renderer::lerp_color(start_color, end_color, i as f32 / body.len() as f32);
                    d.draw_rectangle(point.x * 20 + 20, point.y * 20 + 20, 20, 20, color);
                }
            }
            SnakeStyle::Line => {
                // Draw Snake body
                for i in 0..snake.body.len() {
                    let color =
                        Renderer::lerp_color(start_color, end_color, i as f32 / body.len() as f32);

                    let prev;
                    let curr = body[i].clone();
                    let next;
                    if i == 0 {
                        next = body[i + 1].clone();
                        match snake.direction {
                            snake::Direction::Up => prev = snake::Point::new(curr.x, curr.y - 1),
                            snake::Direction::Down => prev = snake::Point::new(curr.x, curr.y + 1),
                            snake::Direction::Left => prev = snake::Point::new(curr.x - 1, curr.y),
                            snake::Direction::Right => prev = snake::Point::new(curr.x + 1, curr.y),
                        }
                    } else if i == snake.body.len() - 1 {
                        next = snake.last_tail.clone();
                        prev = body[i - 1].clone();
                    } else {
                        prev = body[i - 1].clone();
                        next = body[i + 1].clone();
                    }

                    if prev.x == next.x {
                        // Draw Vertical Line
                        d.draw_rectangle(curr.x * 20 + 20 + 5, curr.y * 20 + 20, 10, 20, color);
                    } else if prev.y == next.y {
                        // Draw Horizontal Line
                        d.draw_rectangle(curr.x * 20 + 20, curr.y * 20 + 20 + 5, 20, 10, color);
                    } else {
                        if (prev.x < curr.x && next.y < curr.y)
                            || (next.x < curr.x && prev.y < curr.y)
                        {
                            // Top left corner
                            d.draw_rectangle(curr.x * 20 + 20 + 5, curr.y * 20 + 20, 10, 15, color);
                            d.draw_rectangle(curr.x * 20 + 20, curr.y * 20 + 20 + 5, 15, 10, color);
                        } else if (prev.x < curr.x && next.y > curr.y)
                            || (next.x < curr.x && prev.y > curr.y)
                        {
                            // Bottom left corner
                            d.draw_rectangle(
                                curr.x * 20 + 20 + 5,
                                curr.y * 20 + 20 + 5,
                                10,
                                15,
                                color,
                            );
                            d.draw_rectangle(curr.x * 20 + 20, curr.y * 20 + 20 + 5, 15, 10, color);
                        } else if (prev.x > curr.x && next.y < curr.y)
                            || (next.x > curr.x && prev.y < curr.y)
                        {
                            // Top right corner
                            d.draw_rectangle(curr.x * 20 + 20 + 5, curr.y * 20 + 20, 10, 15, color);
                            d.draw_rectangle(
                                curr.x * 20 + 20 + 5,
                                curr.y * 20 + 20 + 5,
                                15,
                                10,
                                color,
                            );
                        } else {
                            // Bottom right corner
                            d.draw_rectangle(
                                curr.x * 20 + 20 + 5,
                                curr.y * 20 + 20 + 5,
                                10,
                                15,
                                color,
                            );
                            d.draw_rectangle(
                                curr.x * 20 + 20 + 5,
                                curr.y * 20 + 20 + 5,
                                15,
                                10,
                                color,
                            );
                        }
                    }
                }
            }
        }

        // Draw food
        let food_color = Color::new(231, 71, 29, 255);
        d.draw_rectangle(
            snake.food.x * 20 + 20,
            snake.food.y * 20 + 20,
            20,
            20,
            food_color,
        );
        let next_food_color = Color::new(231, 71, 29, 100);
        d.draw_rectangle(snake.next_food.x * 20 + 20, snake.next_food.y * 20 + 20, 20, 20, next_food_color);

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
