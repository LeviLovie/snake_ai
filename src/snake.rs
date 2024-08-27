use std::collections::VecDeque;

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
    pub direction: Direction,
    pub growth: u32,
    pub width: u32,
    pub height: u32,

    _args: Args,
}

impl Snake {
    pub fn new(args: Args) -> Snake {
        let mut body = VecDeque::new();
        body.push_back(Point::new(2, 0));
        body.push_back(Point::new(1, 0));
        body.push_back(Point::new(0, 0));

        Snake {
            body,
            direction: Direction::Right,
            growth: 0,
            width: args.width,
            height: args.height,

            _args: args,
        }
    }

    pub fn prepare(&mut self) {
        self.body.clear();
        self.body.push_back(Point::new(2, 0));
        self.body.push_back(Point::new(1, 0));
        self.body.push_back(Point::new(0, 0));
        self.direction = Direction::Right;
        self.growth = 0;
    }

    pub fn head(&self) -> &Point {
        self.body.front().unwrap()
    }

    pub fn grow(&mut self) {
        self.growth += 1;
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
            self.body.pop_back();
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
