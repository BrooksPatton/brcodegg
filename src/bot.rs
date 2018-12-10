extern crate serde;
extern crate serde_json;

use bot_move::BotMove;
use ggez::graphics;
use ggez::graphics::DrawMode;
use ggez::{Context, GameResult};
use point::Point;
use rand::{thread_rng, Rng};
use std::fs::File;
use std::io::{Read, Write};
use std::process::Command;
use std::str;
use std::thread;

#[derive(Serialize, Deserialize)]
pub struct Bot {
    pub location: Point,
    radius: f32,
    arena_width: f32,
    arena_height: f32,
}

impl Bot {
    pub fn new(arena_width: f32, arena_height: f32) -> Bot {
        let radius = 25.0;
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0.0, arena_width);
        let y = rng.gen_range(0.0, arena_height);
        let location = Point::new(x, y);

        Bot {
            location,
            radius,
            arena_width,
            arena_height,
        }
    }

    pub fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        let fill = DrawMode::Fill;

        graphics::circle(
            context,
            fill,
            self.location.get_ggez_point2(),
            self.radius,
            0.1,
        )
    }

    pub fn update(&mut self, _context: &mut Context) -> GameResult<()> {
        let serialized_state_for_bot = self.serialize_data().unwrap();
        let new_location = self.run_bot(serialized_state_for_bot).unwrap();

        self.location = new_location;
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

    fn run_bot(&self, json: String) -> Result<Point, serde_json::Error> {
        let result = Command::new("sh")
            .arg("-c")
            .arg(format!("node test_bot.js '{}'", json))
            .output()
            .unwrap();

        let location: Point = serde_json::from_slice(&result.stdout)?;

        Ok(location)
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
    let mut bot = Bot::new(100.0, 100.0);

    bot.location.x = 50.0;
    bot.location.y = 55.0;

    let serialized_data = bot.serialize_data().unwrap();
    let json = "{\"location\":{\"x\":50.0,\"y\":55.0},\"radius\":25.0,\"arena_width\":100.0,\"arena_height\":100.0}";

    assert_eq!(serialized_data, json);
}

#[test]
fn run_bot() {
    let bot = Bot::new(100.0, 100.0);
    let mut expected_point = bot.location.clone();

    expected_point += Point::new(1.0, 1.0);

    let serialized_data = bot.serialize_data().unwrap();
    let new_location = bot.run_bot(serialized_data).unwrap();

    assert_eq!(new_location, expected_point);
}
