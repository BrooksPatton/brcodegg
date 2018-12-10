extern crate ggez;
extern crate rand;

#[macro_use]
extern crate serde_derive;

mod bot;
mod bot_move;
mod point;

use bot::Bot;
use ggez::event::EventHandler;
use ggez::{graphics, Context, GameResult};
use point::Point;

pub struct MainState {
    pub width: f32,
    pub height: f32,
    pub bots: Vec<Bot>,
    grid_cell_size: Point,
}

impl MainState {
    pub fn new(width: f32, height: f32, bots_to_create: u8, grid_cells_count: f32) -> MainState {
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

        // for bot in &mut self.bots {
        //     bot.draw(ctx)?;
        // }

        let mut start_grid = 0.0;
        while start_grid < self.width {
            let lines = [
                Point::new(start_grid, 0.0).get_ggez_point2(),
                Point::new(start_grid, self.height).get_ggez_point2(),
            ];

            graphics::line(ctx, &lines, 1.0)?;
            start_grid += self.grid_cell_size.x;
        }

        let mut start_grid = 0.0;
        while start_grid < self.height {
            let lines = [
                Point::new(0.0, start_grid).get_ggez_point2(),
                Point::new(self.width, start_grid).get_ggez_point2(),
            ];

            graphics::line(ctx, &lines, 1.0)?;
            start_grid += self.grid_cell_size.y;
        }

        graphics::present(ctx);
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn main_state_new() {
    let main_state = MainState::new(55.0, 42.0, 5, 25.0);
    let grid_cell_size = Point::new(2.2, 1.68);

    assert_eq!(main_state.width, 55.0);
    assert_eq!(main_state.height, 42.0);
    assert_eq!(main_state.bots.len(), 5);
    assert_eq!(main_state.grid_cell_size, grid_cell_size);
}
