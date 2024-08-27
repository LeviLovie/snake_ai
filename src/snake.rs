use rand::Rng;
use std::{collections::VecDeque, process::exit};

use super::Args;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

#[derive(Clone)]
pub struct Snake {
    pub body: VecDeque<Point>,
    pub last_tail: Point,
    pub direction: Direction,
    pub growth: u32,
    pub width: u32,
    pub height: u32,
    pub food: Point,
    pub next_food: Point,
    pub started: bool,

    _args: Args,
}

impl Snake {
    pub fn new(args: Args) -> Snake {
        Snake {
            body: VecDeque::new(),
            last_tail: Point::new(0, 0),
            direction: Direction::Right,
            growth: 0,
            width: args.width,
            height: args.height,
            food: Point::new(0, 0),
            next_food: Point::new(0, 0),
            started: false,

            _args: args,
        }
    }

    pub fn prepare(&mut self) {
        self.body.clear();
        self.started = false;
        let x = self.width as i32 / 4;
        let y = self.height as i32 / 2;
        self.body.push_back(Point::new(x + 2, y));
        self.body.push_back(Point::new(x + 1, y));
        self.body.push_back(Point::new(x, y));
        self.last_tail = Point::new(x + 3, y);
        self.direction = Direction::Right;
        self.growth = 0;
        self.food = Point::new(x * 3, y);
        self.next_food = self.gen_next_food();
    }

    pub fn start(&mut self) {
        self.started = true;
    }

    pub fn head(&self) -> &Point {
        self.body.front().unwrap()
    }

    pub fn grow(&mut self) {
        self.growth += 1;
    }
    
    fn gen_next_food(&self) -> Point {
            let mut possible_food_locations = Vec::new();
            for x in 0..self.width as i32 {
                for y in 0..self.height as i32 {
                    let p = Point::new(x, y);
                    if !self.body.contains(&p) {
                        possible_food_locations.push(p);
                    }
                }
            }
            if possible_food_locations.is_empty() {
                exit(0);
            }
            possible_food_locations
                [rand::thread_rng().gen_range(0..possible_food_locations.len())]
    }

    pub fn update(&mut self) {
        if !self.started {
            return;
        }

        self.move_forward();

        if self.head() == &self.food {
            self.grow();
            self.food = self.next_food;
            self.next_food = self.gen_next_food();
        }
    }

    pub fn move_forward(&mut self) {
        let mut new_head = self.head().clone();
        match self.direction {
            Direction::Up => new_head.y -= 1,
            Direction::Down => new_head.y += 1,
            Direction::Left => new_head.x -= 1,
            Direction::Right => new_head.x += 1,
        }
        self.body.push_front(new_head);
        if self.growth == 0 {
            self.last_tail = self.body.pop_back().unwrap();
        } else {
            self.growth -= 1;
        }

        log::info!("Moved snake to {:?}", new_head);
    }

    pub fn turn(&mut self, direction: Direction) {
        if direction != self.direction.opposite() {
            self.direction = direction;
        }
    }

    pub fn collides_with_self(&self) -> bool {
        let head = self.head();
        self.body.iter().skip(1).any(|&p| p == *head)
    }

    pub fn collides_with_wall(&self, width: u32, height: u32) -> bool {
        let head = self.head();
        head.x >= width as i32 || head.y >= height as i32 || head.x < 0 || head.y < 0
    }
}
