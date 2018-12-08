extern crate ggez;
extern crate rand;

#[macro_use]
extern crate serde_derive;

mod bot;
mod point;
mod bot_move;
mod game_state;

use ggez::event::{EventHandler};
use ggez::{GameResult, Context, graphics};
use bot::Bot;

pub struct MainState {
    pub width: f32,
    pub height:f32,
    pub bots: Vec<Bot>
}

impl MainState {
    pub fn new(width: f32, height: f32, bots_to_create: u8) -> MainState {
        let mut bots = Vec::new();

        for _ in 0..bots_to_create {
            
            let bot = Bot::new(width, height);

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