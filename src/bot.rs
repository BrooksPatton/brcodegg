extern crate rand;

use ggez::graphics::{DrawMode};
use ggez::{graphics};
use ggez::{Context, GameResult};
use rand::{thread_rng, Rng};
use point::Point;

pub struct Bot {
    pub location: Point,
    radius: f32,
    arena_width: f32,
    arena_height: f32
}

impl Bot {
    pub fn new(arena_width: f32, arena_height: f32) -> Bot {
        let mut rng = thread_rng();
        let x: f32 = rng.gen_range(0.0, arena_width);
        let y: f32 = rng.gen_range(0.0, arena_height);
        let location = Point::new(x, y);
        let radius = 25.0;

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
        let mut rng = thread_rng();
        let random_x = rng.gen_range(-10.0, 10.0);
        let random_y = rng.gen_range(-10.0, 10.0);
        let random_location = Point::new(random_x, random_y);

        self.move_bot(random_location);
        Ok(())
    }

    fn move_bot(&mut self, distance: Point) {
        self.location += distance;
    }
}

#[cfg(test)]
#[test]
fn new_bot() {
    let bot = Bot::new(300.0, 300.0);

    assert!(bot.location.x > 0.0 && bot.location.x < 300.0);
    assert!(bot.location.y > 0.0 && bot.location.y < 300.0);
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