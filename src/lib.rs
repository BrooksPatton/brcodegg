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

pub struct MainState {
    pub width: u16,
    pub height: u16,
    pub bots: Vec<Bot>,
    game_grid: GameGrid,
}

impl MainState {
    pub fn new(width: u16, height: u16, bots_to_create: u8, grid_cells_count: u16) -> MainState {
        let mut bots = Vec::new();

        for _ in 0..bots_to_create {
            let bot = Bot::new(width, height);

            bots.push(bot);
        }

        MainState {
            width,
            height,
            bots,
            game_grid: GameGrid::new(grid_cells_count, grid_cells_count),
        }
    }

    fn draw_grid(&self, context: &mut Context) -> GameResult<()> {
        let spacing_width = 36;

        for x in &self.game_grid.grid[0] {
            let mut start = x.coordinates.clone();
            let mut end = Point::new(x.coordinates.x, self.height);

            start.x *= spacing_width;
            end.x *= spacing_width;

            let points = [start.to_ggez_point2(), end.to_ggez_point2()];

            graphics::line(context, &points, 1.0)?;
        }

        for y in &self.game_grid.grid {
            let mut start = y[0].coordinates.clone();
            let mut end = Point::new(self.width, y[0].coordinates.y);

            start.y *= spacing_width;
            end.y *= spacing_width;

            let points = [start.to_ggez_point2(), end.to_ggez_point2()];

            graphics::line(context, &points, 1.0)?;
        }

        Ok(())
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

        // for bot in &mut self.bots {
        //     bot.draw(ctx)?;
        // }

        self.draw_grid(ctx)?;

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
    assert_eq!(main_state.game_grid.height, 25);
    assert_eq!(main_state.game_grid.width, 25);
}
