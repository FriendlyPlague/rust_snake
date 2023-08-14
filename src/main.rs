/// Snake written in Rust
// TODO!: Fix rendering scaling


use std::{thread,time::Duration};
use macroquad::{window,shapes,color, input, time, text,rand};

const GAME_WIDTH: i32 = 20;
const GAME_HEIGHT: i32 = 15;
const SPEED: f64 = 0.1;
const Y_OFFSET: f32 = 50.0;
const X_OFFSET: f32 = 50.0;

fn window_conf() -> window::Conf {
    window::Conf {
        window_title: "Snake".to_owned(),
        window_width: 1200,
        window_height: 900,
        window_resizable: false,
        ..Default::default()
    }
}

enum State {
    StartMenu,
    SnakeLoop,
    GameOver,
    ExitGame,
}
#[macroquad::main(window_conf)]
async fn main() {
    let mut state = State::SnakeLoop;
    let mut score: i32 = 0;
    let scale: f32 = {
        let x_scale: f32 = window::screen_width()/GAME_WIDTH as f32 - X_OFFSET*2.0/GAME_WIDTH as f32;
        let y_scale: f32 = window::screen_height()/GAME_HEIGHT as f32 - Y_OFFSET*2.0/GAME_HEIGHT as f32;
        let scale = {
            if x_scale < y_scale {x_scale}
            else {y_scale}};
        scale
    };
    loop {
        match state {
            State::SnakeLoop => {
                let mut snake: Snake = Snake {..Default::default()};
                let mut food: Food = Food {..Default::default()};
                let mut last_time = time::get_time();
                let mut new_direction: Direction = Direction::Left;
                loop {
                    //Proccess input
                    match input::get_last_key_pressed() {
                        Some(input::KeyCode::Left) => new_direction = Direction::Left,
                        Some(input::KeyCode::Right) => new_direction = Direction::Right,
                        Some(input::KeyCode::Up) => new_direction = Direction::Up,
                        Some(input::KeyCode::Down) => new_direction = Direction::Down,
                        Some(input::KeyCode::Escape) => {state = State::ExitGame; break;},
                        _ => (),
                    };
                    if (time::get_time() - last_time) > SPEED {
                        snake.set_direction(&new_direction);
                        match snake.move_forward((food.x, food.y)) {
                            0 => (),
                            1 => {
                                food = Food {..Default::default()};
                                score += 1;},
                            2 => { println!("went out of bounds");
                                state = State::GameOver;
                                break;},
                            _ => panic!("move_forward returned unexpected number"),
                        }
                        last_time = time::get_time();
                    }
                    // render game
                    window::clear_background(color::BEIGE);
                    text::draw_text(&score.to_string(), window::screen_width()/2.0, 35.0, 60.0, color::BLACK);
                    for y in 0..GAME_HEIGHT {
                        for x in 0..GAME_WIDTH {
                            shapes::draw_rectangle_lines((x as f32)*scale+X_OFFSET, (y as f32)*scale+Y_OFFSET, scale, scale,2.0,color::BLACK);
                        }
                    }
                    snake.draw(scale);
                    shapes::draw_rectangle((food.x as f32)*scale+X_OFFSET, (food.y as f32)*scale+Y_OFFSET, scale, scale,color::YELLOW);
                    window::next_frame().await;
                    thread::sleep(Duration::from_millis(12));
                }
            },

            State::GameOver => {
                loop {
                    //Proccess input
                    match input::get_last_key_pressed() {
                        Some(input::KeyCode::Escape) => {state = State::ExitGame; break;},
                        Some(input::KeyCode::R) => {state = State::SnakeLoop; break;},
                        _ => (),
                    };
                    window::clear_background(color::BEIGE);
                    text::draw_text("YOU DIED!", window::screen_width()/2.0-150.0, 250.0, 100.0, color::RED);
                    text::draw_text(&format!("Score: {}",score), window::screen_width()/2.0-150.0, 350.0, 100.0, color::BLACK);
                    text::draw_text("Press R to restart", window::screen_width()/2.0-350.0, 500.0, 100.0, color::BLACK);
                    window::next_frame().await;
                    thread::sleep(Duration::from_millis(12));
                }
            }
            _ => break,
        }
    }

}
struct Food {
    x: i32,
    y: i32,
}

impl Default for Food {
    fn default() -> Self {
        let rnd_num = rand::gen_range(0,GAME_WIDTH*GAME_HEIGHT);
        let rx = rnd_num%GAME_WIDTH;
        let ry = rnd_num/GAME_WIDTH;
        Food {x: rx, y: ry}
    }
}
struct Snake {
    head: (i32,i32), // X and Y
    direction: Direction,
    body: Vec<(i32,i32)>,
}
impl Default for Snake {
    fn default() -> Self {
        Snake {
            head: (GAME_WIDTH/2,GAME_HEIGHT/2),
            direction: Direction::Right,
            body: Vec::new()
        }
    }
}
impl Snake {
    fn set_direction(&mut self,dir: &Direction) {
        let opposite_dir = match self.direction {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up
        };
        if opposite_dir != *dir {
            self.direction = dir.clone();
        }

    }
    fn move_forward (&mut self, food_pos: (i32, i32)) -> u8 {
        // return 0 if moved
        // return 1 if ate food
        // return 2 if dead
        self.body.insert(0,self.head);
        match self.direction {
            Direction::Left => {self.head.0 -= 1;},
            Direction::Right => {self.head.0 += 1;},
            Direction::Up => {self.head.1 -= 1;},
            Direction::Down => {self.head.1 += 1;},
        }
        if self.head.0 < 0 || self.head.0 >= GAME_WIDTH ||
            self.head.1 < 0 || self.head.1 >= GAME_HEIGHT {return 2;}
        if self.head != food_pos {
            self.body.pop();
            return 0;
        }
        1
    }
    fn draw(&self, scale: f32) {
        shapes::draw_rectangle((self.head.0 as f32)*scale+X_OFFSET, (self.head.1 as f32)*scale+Y_OFFSET, scale,scale, color::GREEN);
        for (x, y) in &self.body {
            shapes::draw_rectangle((*x as f32)*scale+X_OFFSET, (*y as f32)*scale+Y_OFFSET, scale,scale, color::GREEN);
        }
    }

}

#[derive(PartialEq,Clone, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}