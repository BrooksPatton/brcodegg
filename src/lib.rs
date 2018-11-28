extern crate ggez;

use ggez::event::{EventHandler};
use ggez::{GameResult, Context, graphics};
use ggez::graphics::{DrawMode, Point2};

pub struct MainState {
    pub width: f32,
    pub height:f32
}

impl MainState {
    pub fn new(width: f32, height: f32) -> MainState {
        MainState {
            width,
            height
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let location = Point2::new(150.0, 150.0);

        graphics::circle(ctx, DrawMode::Fill, location, 100.0, 0.1)?;
        graphics::present(ctx);
        Ok(())
    }
}

#[cfg(test)]

#[test]
fn main_state_new() {
    let main_state = MainState::new(55.0, 42.0);

    assert_eq!(main_state.width, 55.0);
    assert_eq!(main_state.height, 42.0);
}