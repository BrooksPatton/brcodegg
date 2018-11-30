extern crate ggez;
extern crate rand;

mod bot;

use ggez::event::{EventHandler};
use ggez::{GameResult, Context, graphics};
use bot::Bot;
use rand::{thread_rng, Rng};

pub struct MainState {
    pub width: f32,
    pub height:f32,
    pub bots: Vec<Bot>
}

impl MainState {
    pub fn new(width: f32, height: f32, bots_to_create: u8) -> MainState {
        let mut bots = Vec::new();
        let mut rng = thread_rng();

        for _ in 0..bots_to_create {
            let x: f32 = rng.gen_range(0.0, width);
            let y: f32 = rng.gen_range(0.0, height);
            let bot = Bot::new(x, y);

            bots.push(bot);
        }

        MainState {
            width,
            height,
            bots
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        for bot in &mut self.bots {
            bot.update(ctx)?;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        
        for bot in &mut self.bots {
            bot.draw(ctx)?;
        }

        graphics::present(ctx);
        Ok(())
    }
}

#[cfg(test)]

#[test]
fn main_state_new() {
    let main_state = MainState::new(55.0, 42.0, 5);

    assert_eq!(main_state.width, 55.0);
    assert_eq!(main_state.height, 42.0);
    assert_eq!(main_state.bots.len(), 5);
}