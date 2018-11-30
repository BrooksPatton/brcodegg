use ggez::graphics::{Point2, DrawMode};
use ggez::graphics;
use ggez::{Context, GameResult};

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
}

#[cfg(test)]
#[test]
fn new_bot() {
    let bot = Bot::new(300.0, 300.0);

    assert_eq!(bot.location.x, 300.0);
    assert_eq!(bot.location.y, 300.0);
    assert_eq!(bot.radius, 25.0);
}