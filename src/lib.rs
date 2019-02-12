extern crate ggez;
extern crate rand;

#[macro_use]
extern crate serde_derive;

mod bot;
mod bot_move;
mod game_grid;
mod point;

use crate::bot::Bot;
use crate::game_grid::{GameGrid, GridError};
use crate::point::Point;
use ggez::event::EventHandler;
use ggez::{graphics, Context, GameResult};

#[derive(PartialEq, Debug)]
pub enum BotFileName {
    Local(String),
    Url(String),
}

pub struct MainState {
    pub width: u16,
    pub height: u16,
    pub bots: Vec<Bot>,
    game_grid: GameGrid,
    turn_number: u32
}

impl MainState {
    pub fn new(
        width: u16,
        height: u16,
        grid_cells_count: u16,
        bot_file_names: Vec<BotFileName>,
    ) -> MainState {
        let game_grid = GameGrid::new(grid_cells_count, grid_cells_count);
        let mut bots = Vec::new();
        let turn_number = 0;
        let mut index = 0;

        for bot_file_name in bot_file_names {
            let bot = match bot_file_name {
                BotFileName::Local(_path) => Bot::new(game_grid.width, game_grid.height, index),
                BotFileName::Url(_url) => Bot::new(game_grid.width, game_grid.height, index)
            };

            bots.push(bot);
            index += 1;
        }

        MainState {
            width,
            height,
            bots,
            game_grid,
            turn_number
        }
    }

    fn draw_grid(&self, context: &mut Context) -> GameResult<()> {
        let cell_width = self.width / self.game_grid.width;
        let cell_height = self.height / self.game_grid.height;
        
        for count in 0..self.game_grid.width {
            let start = Point::new(cell_width * count, 0).to_ggez_point2();
            let end = Point::new(cell_width * count, self.height).to_ggez_point2();

            graphics::line(context, &[start, end], 1.0)?;
        }

        for count in 0..self.game_grid.height {
            let start = Point::new(0, cell_height * count).to_ggez_point2();
            let end = Point::new(self.width, cell_height * count).to_ggez_point2();

            graphics::line(context, &[start, end], 1.0)?;
        }

        Ok(())
    }

    fn draw_bots(&self, context: &mut Context) -> GameResult<()> {
        // loop through all bots on the grid, and then draw something to represent said bots
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _context: &mut Context) -> GameResult<()> {
        for bot in &mut self.bots {
            bot.update(self.turn_number)?;
            if self.turn_number == 0 {
                match self.game_grid.place_bot(bot.index, bot.location.clone()) {
                    Err(GridError::bot_exists_in_location) => println!("bot already exists where you are trying to place it"),
                    Err(GridError::out_of_bounds) => println!("bot attempted to be placed on first turn out of bounds"),
                    Ok(_) => (),
                }
            }
        }

        dbg!(&self.game_grid);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        self.draw_grid(ctx)?;
        self.draw_bots(ctx)?;

        graphics::present(ctx);
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn main_state_new() {
    let bot_locations = vec![BotFileName::Local(String::from(
        "this_will_be_a_filename.js",
    ))];
    let main_state = MainState::new(55, 42, 5, bot_locations);

    assert_eq!(main_state.width, 55);
    assert_eq!(main_state.height, 42);
    assert_eq!(main_state.game_grid.height, 5);
    assert_eq!(main_state.game_grid.width, 5);
    assert_eq!(main_state.turn_number, 0);
}
