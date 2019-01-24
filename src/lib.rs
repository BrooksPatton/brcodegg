extern crate ggez;
extern crate rand;

#[macro_use]
extern crate serde_derive;

mod bot;
mod bot_move;
mod game_grid;
mod point;

use crate::bot::Bot;
use crate::game_grid::GameGrid;
use crate::point::Point;
use ggez::event::EventHandler;
use ggez::{graphics, Context, GameResult};

#[derive(PartialEq, Debug)]
pub enum BotLocation {
    Local(String),
    Url(String),
}

pub struct MainState {
    pub width: u16,
    pub height: u16,
    pub bots: Vec<Bot>,
    game_grid: GameGrid,
    bot_locations: Vec<BotLocation>,
}

impl MainState {
    pub fn new(
        width: u16,
        height: u16,
        bots_to_create: u8,
        grid_cells_count: u16,
        bot_locations: Vec<BotLocation>,
    ) -> MainState {
        let game_grid = GameGrid::new(grid_cells_count, grid_cells_count);
        let mut bots = Vec::new();

        for _ in 0..bots_to_create {
            let bot = Bot::new(game_grid.width, game_grid.height);

            bots.push(bot);
        }

        MainState {
            width,
            height,
            bots,
            game_grid,
            bot_locations,
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
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        for bot in &mut self.bots {
            bot.update(ctx)?;
            //update the grid with what the bot did
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        self.draw_grid(ctx)?;

        graphics::present(ctx);
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn main_state_new() {
    let bot_locations = vec![BotLocation::Local(String::from(
        "this_will_be_a_filename.js",
    ))];
    let main_state = MainState::new(55, 42, 5, 25, bot_locations);

    assert_eq!(main_state.width, 55);
    assert_eq!(main_state.height, 42);
    assert_eq!(main_state.bots.len(), 5);
    assert_eq!(main_state.game_grid.height, 25);
    assert_eq!(main_state.game_grid.width, 25);
    assert_eq!(
        main_state.bot_locations[0],
        BotLocation::Local("this_will_be_a_filename.js".into())
    );
}
