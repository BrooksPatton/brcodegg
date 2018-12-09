extern crate serde_json;
extern crate serde;

use ggez::graphics::{DrawMode};
use ggez::{graphics};
use ggez::{Context, GameResult};
use rand::{thread_rng, Rng};
use point::Point;
use std::thread;
use std::process::Command;
use std::str;
use bot_move::BotMove;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Serialize, Deserialize)]
pub struct Bot {
    pub location: Point,
    radius: f32,
    arena_width: f32,
    arena_height: f32
}

impl Bot {
    pub fn new(arena_width: f32, arena_height: f32) -> Bot {
        let radius = 25.0;
        let location = Point::new(50.0, 50.0);

        Bot {
            location,
            radius,
            arena_width,
            arena_height
        }
    }

    pub fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        let fill = DrawMode::Fill;

        graphics::circle(context, fill, self.location.get_ggez_point2(), self.radius, 0.1)
    }

    pub fn update(&mut self, _context: &mut Context) -> GameResult<()> {
        let serialized_state_for_bot = self.serialize_data()?;
        // let new_location = self.run_bot(serialized_state_for_bot)?;
        // self.location = new_location;
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

    fn serialize_data(&self) -> Result<String, serde_json::Error> {
        let result = serde_json::to_string(self)?;

        Ok(result)
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

#[test]
fn serialize_state() {
    let bot = Bot::new(100.0, 100.0);
    let serialized_data = bot.serialize_data().unwrap();
    let json = "{\"location\":{\"x\":50.0,\"y\":50.0},\"radius\":25.0,\"arena_width\":100.0,\"arena_height\":100.0}";

    assert_eq!(serialized_data, json);
}