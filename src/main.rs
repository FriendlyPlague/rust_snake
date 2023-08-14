use std::{thread,time::Duration};
use std::collections::VecDeque;
use macroquad::{window,shapes,color, input, time, text,rand};
use indexmap::IndexMap;

const GAME_WIDTH: i32 = 32;
const GAME_HEIGHT: i32 = 18;
const SPEED: f64 = 0.1;
const Y_OFFSET: f32 = 50.0;
const X_OFFSET: f32 = 30.0;

fn window_conf() -> window::Conf {
    window::Conf {
        window_title: "Snake".to_owned(),
        window_width: 1280,
        window_height: 720,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game_objects: IndexMap<(i32,i32),ObjectName> = IndexMap::new();
    let mut snake: Snake = Snake {
        head: (15,8), // starting location for snake
        direction: Direction::Up,
        body: VecDeque::new(),
    };
    game_objects.insert((15,8), ObjectName::SnakeHead);
    // used to scale up game to better fit screen size
    let scale = {
        let x_scale: f32 = window::screen_width()/(GAME_WIDTH+1) as f32;
        let y_scale: f32 = window::screen_height()/(GAME_HEIGHT+2) as f32;
        let scale = {
            if x_scale < y_scale {x_scale}
            else {y_scale}};
        if scale < 10.0 {10.0}
        else {scale}
    };

    let mut score: i32 = 1;
    let mut last_time = time::get_time();
    let mut new_direction = Direction::Up;
    let mut food_exists = false;
    let mut game_over = false;
    loop {
        //Proccess input
        match input::get_last_key_pressed() {
            Some(input::KeyCode::Left) => new_direction = Direction::Left,
            Some(input::KeyCode::Right) => new_direction = Direction::Right,
            Some(input::KeyCode::Up) => new_direction = Direction::Up,
            Some(input::KeyCode::Down) => new_direction = Direction::Down,
            Some(input::KeyCode::Escape) => break,
            _ => (),
        }
        if !food_exists {
            loop {
                let rnd_num = rand::gen_range(0,GAME_WIDTH*GAME_HEIGHT) as i32;
                let x = rnd_num%GAME_WIDTH;
                let y = rnd_num/GAME_WIDTH;
                if !game_objects.contains_key(&(x, y)) {
                    game_objects.insert((x,y), ObjectName::Food);
                    break;
                }
            }
            food_exists = true;
        }
        //update game objects
        if !game_over && (time::get_time() - last_time) > SPEED {
            last_time = time::get_time();
            snake.set_direction(new_direction.clone());

            let result = snake.move_forward(&mut game_objects);
            if result == 1 {
                println!("Food eaten!");
                food_exists = false;
                score += 1;
            }
            else if result == 2 {
                println!("YOU DIED!");
                game_objects.clear();
                game_over = true;
            }
        }

        // render
        window::clear_background(color::BEIGE);
        if !game_over {
            text::draw_text(&score.to_string(), window::screen_width()/2.0, 35.0, 60.0, color::BLACK);
            for y in 0..GAME_HEIGHT {
                for x in 0..GAME_WIDTH {
                    shapes::draw_rectangle_lines((x as f32)*scale+X_OFFSET, (y as f32)*scale+Y_OFFSET, scale, scale,2.0,color::BLACK);
                }
            }
        }
        else {
            text::draw_text("YOU DIED!", window::screen_width()/2.0-150.0, 250.0, 100.0, color::RED);
            text::draw_text(&format!("Score: {}",score.to_string()), window::screen_width()/2.0-150.0, 350.0, 100.0, color::BLACK);
        }
        for (k,v) in &game_objects {
            match v {
                ObjectName::SnakeHead | ObjectName::SnakeSeg => {shapes::draw_rectangle((k.0 as f32)*scale+X_OFFSET, (k.1 as f32)*scale+Y_OFFSET, scale,scale, color::GREEN);},
                ObjectName::Food => {shapes::draw_rectangle((k.0 as f32)*scale+X_OFFSET, (k.1 as f32)*scale+Y_OFFSET, scale,scale, color::YELLOW);},
            }
        }
        window::next_frame().await;
        // sleeps
        thread::sleep(Duration::from_millis(12));
    }
}

#[derive(Copy,Clone)]
enum ObjectName {
    Food,
    SnakeHead,
    SnakeSeg,
}

struct Snake {
    head: (i32,i32),
    direction: Direction,
    body: VecDeque<(i32,i32)>,
}

impl Snake {
    fn set_direction(&mut self,dir: Direction) {
        let opposite_dir = match self.direction {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up
        };
        if opposite_dir == dir {
            println!("Can't change to opposite direction!");
        } else {
            self.direction = dir;
        }
    }
    fn move_forward (&mut self, game: &mut IndexMap<(i32,i32),ObjectName>) -> u16 {
        // 0 = moving freely
        // 1 = eating food and growing
        // 2 = dying
        // add segment with previous pos
        self.body.push_back(self.head);
        game.remove(&self.head);
        game.insert(self.head, ObjectName::SnakeSeg);
        // move forward
        match self.direction {
            Direction::Left => {self.head.0 -= 1;},
            Direction::Right => {self.head.0 += 1;},
            Direction::Up => {self.head.1 -= 1;},
            Direction::Down => {self.head.1 += 1;},
        }
        if (self.head.0 < 0 || self.head.0 >= GAME_WIDTH ||
            self.head.1 < 0 || self.head.1 >= GAME_HEIGHT) {return 2;}
        match game.get_key_value(&self.head) {
            Some((_, ObjectName::Food)) => {
                game.insert(self.head, ObjectName::SnakeHead);
                1},
            Some(_) => 2,
            None => {// deletes end seg
                    game.insert(self.head, ObjectName::SnakeHead);
                    game.remove(self.body.front().unwrap());
                    self.body.pop_front();
                    0}
        }

    }

}

#[derive(PartialEq,Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}