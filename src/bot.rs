extern crate serde;
extern crate serde_json;

use crate::point::Point;
use ggez::{Context, GameResult};
use std::fs::File;
use std::io::Write;
use std::process::Command;

#[derive(Serialize, Deserialize, Debug)]
pub struct Bot {
    pub location: Point,
    game_grid_width: u16,
    game_grid_height: u16,
    turn_number: u32,
    pub index: u16
}

impl Bot {
    pub fn new(game_grid_width: u16, game_grid_height: u16, index: u16) -> Bot {
        let x = 3;
        let y = 3;
        let location = Point::new(x, y);
        let turn_number = 0;

        Bot {
            location,
            game_grid_width,
            game_grid_height,
            turn_number,
            index
        }
    }

    pub fn update(&mut self, new_turn_number: u32) -> GameResult<Point> {
        self.turn_number = new_turn_number;
        let serialized_state_for_bot = self.serialize_data().unwrap();
        
        Ok(self.run_bot(serialized_state_for_bot).unwrap())
    }

    fn serialize_data(&self) -> Result<String, serde_json::Error> {
        let result = serde_json::to_string(self)?;

        Ok(result)
    }

    fn run_bot(&self, json: String) -> Result<Point, serde_json::Error> {
        let result = Command::new("sh")
            .arg("-c")
            .arg(format!("node bot_examples/chooses_start_location.js '{}'", json))
            .output()
            .expect("Error running Node bot");

        let location: Point = serde_json::from_slice(&result.stdout)?;

        Ok(location)
    }
}

fn _save_to_file(content: String) -> Result<(), std::io::Error> {
    let mut file = File::create("game_state.json")?;

    file.write(&content.into_bytes())?;
    Ok(())
}

#[cfg(test)]
#[test]
fn new_bot() {
    let bot = Bot::new(300, 300, 0);

    assert!(bot.location.x ==3);
    assert!(bot.location.y == 3);
    assert_eq!(bot.game_grid_width, 300);
    assert_eq!(bot.game_grid_height, 300);
    assert_eq!(bot.turn_number, 0);
    assert_eq!(bot.index, 0);
}

#[test]
fn serialize_state() {
    let mut bot = Bot::new(100, 100, 0);

    bot.location.x = 50;
    bot.location.y = 55;

    let serialized_data = bot.serialize_data().unwrap();
    let json =
        "{\"location\":{\"x\":50,\"y\":55},\"game_grid_width\":100,\"game_grid_height\":100,\"turn_number\":0,\"index\":0}";

    assert_eq!(serialized_data, json);
}

#[test]
fn run_bot() {
    let bot = Bot::new(100, 100, 0);
    let expected_point = Point::new(5, 5);

    let serialized_data = bot.serialize_data().unwrap();
    let new_location = bot.run_bot(serialized_data).unwrap();

    assert_eq!(new_location, expected_point);
    assert_eq!(bot.turn_number, 0);
}

#[test]
fn update() {
    let mut bot = Bot::new(100, 100, 0);

    let location = bot.update(0).unwrap();
    let expected_location = Point::new(5, 5);

    assert_eq!(location, expected_location);
}