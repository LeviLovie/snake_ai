pub mod renderer;
pub mod snake;

use clap::{Parser, Subcommand};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: Commands,

    #[arg(long, default_value = "20")]
    pub width: u32,
    #[arg(long, default_value = "20")]
    pub height: u32,

    #[arg(short, long, default_value = "line")]
    pub style: SnakeStyle,

    #[arg(long, default_value = "30")]
    pub fps: u32,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Play,
}

#[derive(clap::ValueEnum, Default, Debug, Clone)]
pub enum SnakeStyle {
    Block,

    #[default]
    Line,
}

fn main() {
    let args = Args::parse();
    pretty_env_logger::init();

    let snake = Arc::new(Mutex::new(snake::Snake::new(args.clone())));

    let snake_for_thread = Arc::clone(&snake);
    thread::spawn(move || loop {
        {
            let _snake = snake.lock().unwrap();
            // snake.move_forward();
        }
        std::thread::sleep(std::time::Duration::from_millis(200));
    });

    let mut renderer = renderer::Renderer::new(snake_for_thread, args.clone());
    renderer.run();

    // loop {
    //     let mut input = String::new();
    //     std::io::stdin().read_line(&mut input).unwrap();
    //
    //     let mut snake = snake.lock().unwrap();
    //     match input.trim() {
    //         "w" => snake.direction = snake.direction.opposite(),
    //         "a" => snake.direction = snake.direction.opposite(),
    //         "s" => snake.direction = snake.direction.opposite(),
    //         "d" => snake.direction = snake.direction.opposite(),
    //         _ => continue,
    //     }
    // }

    // match args.cmd {
    //     Commands::Play() => {
    //         println!("Booting up snake for playing");
    //     }
    // }
}
