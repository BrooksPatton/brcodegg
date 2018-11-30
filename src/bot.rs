extern crate rand;

use ggez::graphics::{Point2, DrawMode};
use ggez::graphics;
use ggez::{Context, GameResult};
use rand::{thread_rng, Rng};

pub struct Bot {
    pub location: Point2,
    radius: f32
}

impl Bot {
    pub fn new(x: f32, y: f32) -> Bot {
        let location = Point2::new(x, y);
        let radius = 25.0;

        Bot {
            location,
            radius
            }
    }

    pub fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        let fill = DrawMode::Fill;

        graphics::circle(context, fill, self.location, self.radius, 0.1)
    }

    pub fn update(&mut self, _context: &mut Context) -> GameResult<()> {
        let mut rng = thread_rng();
        let random_x = rng.gen_range(-10.0, 10.0);
        let random_y = rng.gen_range(-10.0, 10.0);

        self.location.x += random_x;
        self.location.y += random_y;
        Ok(())
    }

    fn move_bot(&mut self, distance: Point2) {
        self.location += distance;
    }
}

#[cfg(test)]
#[test]
fn new_bot() {
    let bot = Bot::new(300.0, 300.0);

    assert_eq!(bot.location.x, 300.0);
    assert_eq!(bot.location.y, 300.0);
    assert_eq!(bot.radius, 25.0);
}

#[test]
fn move_bot() {
    let mut bot = Bot::new(300.0, 300.0);
    let distance = Point2::new(-50.0, 0.0);
    
    bot.move_bot(distance);
    assert_eq!(bot.location.x, 250.0);
    assert_eq!(bot.location.y, 300.0);
}