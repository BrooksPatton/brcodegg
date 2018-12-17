extern crate serde;
extern crate serde_json;

use crate::point::Point;
use ggez::graphics;
use ggez::graphics::DrawMode;
use ggez::{Context, GameResult};
use std::fs::File;
use std::io::Write;
use std::process::Command;

#[derive(Serialize, Deserialize)]
pub struct Bot {
    pub location: Point<u16>,
    radius: u16,
    arena_width: u16,
    arena_height: u16,
}

impl Bot {
    pub fn new(arena_width: u16, arena_height: u16) -> Bot {
        let radius = 25;
        let mut rng = rand::thread_rng();
        let x = 55;
        let y = 62;
        let location = Point::<u16>::new(x, y);

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
            self.location.to_ggez_point2(),
            self.radius.into(),
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

    fn keep_in_arena(&mut self) {
        if (self.location.y - self.radius) < 0 {
            self.location.y = self.radius;
        } else if (self.location.y + self.radius) > self.arena_height {
            self.location.y = self.arena_height - self.radius;
        }

        if (self.location.x - self.radius) < 0 {
            self.location.x = self.radius;
        } else if (self.location.x + self.radius) > self.arena_width {
            self.location.x = self.arena_width - self.radius;
        }
    }

    fn serialize_data(&self) -> Result<String, serde_json::Error> {
        let result = serde_json::to_string(self)?;

        Ok(result)
    }

    fn run_bot(&self, json: String) -> Result<Point<u16>, serde_json::Error> {
        let result = Command::new("sh")
            .arg("-c")
            .arg(format!("node test_bot.js '{}'", json))
            .output()
            .unwrap();

        let location: Point<u16> = serde_json::from_slice(&result.stdout)?;

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
    let bot = Bot::new(300, 300);

    assert!(bot.location.x >= 0 && bot.location.x <= 300);
    assert!(bot.location.y >= 0 && bot.location.y <= 300);
    assert_eq!(bot.radius, 25);
    assert_eq!(bot.arena_width, 300);
    assert_eq!(bot.arena_height, 300);
}

#[test]
fn serialize_state() {
    let mut bot = Bot::new(100, 100);

    bot.location.x = 50;
    bot.location.y = 55;

    let serialized_data = bot.serialize_data().unwrap();
    let json =
        "{\"location\":{\"x\":50,\"y\":55},\"radius\":25,\"arena_width\":100,\"arena_height\":100}";

    assert_eq!(serialized_data, json);
}

#[test]
fn run_bot() {
    let bot = Bot::new(100, 100);
    let mut expected_point = bot.location.clone();

    expected_point += Point::<u16>::new(1, 1);

    let serialized_data = bot.serialize_data().unwrap();
    let new_location = bot.run_bot(serialized_data).unwrap();

    assert_eq!(new_location, expected_point);
}
