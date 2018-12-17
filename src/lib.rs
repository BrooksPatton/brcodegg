extern crate ggez;
extern crate rand;

#[macro_use]
extern crate serde_derive;

mod bot;
mod bot_move;
mod game_grid;
mod point;

use crate::bot::Bot;
use crate::point::Point;
use ggez::event::EventHandler;
use ggez::{graphics, Context, GameResult};

pub struct MainState {
    pub width: u16,
    pub height: u16,
    pub bots: Vec<Bot>,
    grid_cell_size: Point<u16>,
}

impl MainState {
    pub fn new(width: u16, height: u16, bots_to_create: u8, grid_cells_count: u16) -> MainState {
        let mut bots = Vec::new();

        for _ in 0..bots_to_create {
            let bot = Bot::new(width, height);

            bots.push(bot);
        }

        let grid_cell_size_x = width / grid_cells_count;
        let grid_cell_size_y = height / grid_cells_count;

        MainState {
            width,
            height,
            bots,
            grid_cell_size: Point::new(grid_cell_size_x, grid_cell_size_y),
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
    let main_state = MainState::new(55, 42, 5, 25);

    assert_eq!(main_state.width, 55);
    assert_eq!(main_state.height, 42);
    assert_eq!(main_state.bots.len(), 5);
}
