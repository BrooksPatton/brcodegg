extern crate crossbeam_channel;
extern crate serde_json;
extern crate serde;

use ggez::graphics::{DrawMode};
use ggez::{graphics};
use ggez::{Context, GameResult};
use rand::{thread_rng, Rng};
use point::Point;
use std::thread;
use self::crossbeam_channel::{bounded, Receiver, Sender};
use std::process::Command;
use std::str;
use bot_move::BotMove;
use game_state::GameState;
use std::fs::File;
use std::io::{Read, Write};

pub struct Bot {
    pub location: Point,
    radius: f32,
    arena_width: f32,
    arena_height: f32,
    thread_receiver: Receiver<Point>,
    thread_sender: Sender<Point>
}

impl Bot {
    pub fn new(arena_width: f32, arena_height: f32) -> Bot {
        let radius = 25.0;
        let (sender, receiver) = bounded(1);
        let thread_sender = sender.clone();
        let thread_receiver = receiver.clone();
        let location = Point::new(0.0, 0.0);
        let game_state_location = Point::new(location.x, location.y);

        thread::spawn(move || {
            let game_state = GameState::new(arena_width, arena_height, game_state_location);
            let stringified_game_state = serde_json::to_string(&game_state).unwrap();

            save_to_file(stringified_game_state.clone()).unwrap();

            let output = Command::new("sh")
                .arg("-c")
                .arg(format!("node bot.js '{}'", stringified_game_state))
                .output()
                .expect("failed to execute process");

            println!("{:?}", output);

            let json: BotMove = serde_json::from_slice(&output.stdout).unwrap();
            let x = &json.x;
            let y = &json.y;
            let bot_location = Point::new(*x, *y);

            loop {
                let point = Point::new(bot_location.x, bot_location.y);

                sender.send(point).unwrap();
            }
        });

        Bot {
            location,
            radius,
            arena_width,
            arena_height,
            thread_receiver,
            thread_sender
        }
    }

    pub fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        let fill = DrawMode::Fill;

        graphics::circle(context, fill, self.location.get_ggez_point2(), self.radius, 0.1)
    }

    pub fn update(&mut self, _context: &mut Context) -> GameResult<()> {
        // let mut rng = thread_rng();
        // let random_x = rng.gen_range(-10.0, 10.0);
        // let random_y = rng.gen_range(-10.0, 10.0);
        // let random_location = Point::new(random_x, random_y);
        self.location = self.thread_receiver.recv().unwrap();

        // self.move_bot(random_location);
        self.keep_in_arena();
        Ok(())
    }

    fn move_bot(&mut self, distance: Point) {
        self.location += distance;
    }

    fn keep_in_arena(&mut self) {
        if (self.location.y - self.radius) < 0.0 {
            self.location.y = self.radius;
        } else if (self.location.y + self.radius) > self.arena_height {
            self.location.y = self.arena_height - self.radius;
        }

        if (self.location.x - self.radius) < 0.0 {
            self.location.x = self.radius;
        } else if (self.location.x + self.radius) > self.arena_width {
            self.location.x = self.arena_width - self.radius;
        }
    }
}

fn save_to_file(content: String) -> Result<(), std::io::Error> {
    let mut file = File::create("game_state.json")?;

    file.write(&content.into_bytes())?;
    Ok(())
}

#[cfg(test)]
#[test]
fn new_bot() {
    let bot = Bot::new(300.0, 300.0);

    assert!(bot.location.x >= 0.0 && bot.location.x <= 300.0);
    assert!(bot.location.y >= 0.0 && bot.location.y <= 300.0);
    assert_eq!(bot.radius, 25.0);
    assert_eq!(bot.arena_width, 300.0);
    assert_eq!(bot.arena_height, 300.0);
}

#[test]
fn move_bot() {
    let mut bot = Bot::new(300.0, 300.0);
    let distance = Point::new(-50.0, 0.0);
    let x = bot.location.x;
    let y = bot.location.y;
    
    bot.move_bot(distance);
    assert_eq!(x - 50.0, bot.location.x);
    assert_eq!(y, bot.location.y);
}

#[test]
fn keep_in_arena() {
    let mut bot = Bot::new(300.0, 300.0);
    let distance_to_move = Point::new(500.0, 0.0);
    
    bot.move_bot(distance_to_move);
    bot.keep_in_arena();
    assert_eq!(bot.location.x, 300.0 - bot.radius);
}

#[test]
fn save_to_file_test() {
    let content = String::from("meow");

    save_to_file(content).unwrap();

    let mut file = File::open("game_state.json").unwrap();
    let mut file_contents = String::new();

    file.read_to_string(&mut file_contents).unwrap();

    assert_eq!(file_contents, "meow");
}