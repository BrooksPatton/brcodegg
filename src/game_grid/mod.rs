mod piece;
mod error;

use std::collections::HashMap;
use crate::Point;
use piece::Piece;
pub use error::GridError;

#[derive(Debug)]
pub struct GameGrid {
    pub grid: HashMap<Point, Piece>,
    pub width: u16,
    pub height: u16,
}

impl GameGrid {
    pub fn new(width: u16, height: u16) -> GameGrid {
        let grid = HashMap::new();

        GameGrid {
            grid,
            width,
            height,
        }
    }

    pub fn place_bot(&mut self, bot_index: u16, bot_location: Point) -> Result<(), GridError> {
        let piece = Piece::bot_index(bot_index);

        if bot_location.x > self.width || bot_location.y > self.height {
            return Err(GridError::out_of_bounds);
        }

        match self.grid.get(&bot_location) {
            Some(cell) => match cell {
                Piece::bot_index(_) => Err(GridError::bot_exists_in_location)
            },
            None => {
                self.grid.insert(bot_location, piece);
                Ok(())
            }
        }
    }
}

#[cfg(test)]
#[test]
fn grid_new() {
    let columns = 2;
    let rows = 2;
    let game_grid = GameGrid::new(columns, rows);
    let empty_grid = HashMap::new();

    assert_eq!(game_grid.grid, empty_grid);
    assert_eq!(game_grid.width, 2);
    assert_eq!(game_grid.height, 2);
}

#[test]
fn place_first_bot_on_grid() {
    let mut game_grid = GameGrid::new(2, 2);
    let bot_index = 0;
    let bot_location = Point::new(1, 1);
    let get_bot_location = bot_location.clone();
    let bot_piece = Piece::bot_index(0);

    game_grid.place_bot(bot_index, bot_location).unwrap();

    assert_eq!(game_grid.grid.get(&get_bot_location), Some(&bot_piece));
}

#[test]
fn place_bot_out_of_bounds_on_grid() {
    let mut game_grid = GameGrid::new(2, 2);
    let bot_location = Point::new(25, 25);

    match game_grid.place_bot(0, bot_location) {
        Err(error) => assert_eq!(error, GridError::out_of_bounds),
        _ => panic!("should have errored with out of bounds")
    };
}

#[test]
fn place_bot_on_top_of_other_bot() {
    let mut game_grid = GameGrid::new(2, 2);

    game_grid.place_bot(0, Point::new(1, 1)).unwrap();
    match game_grid.place_bot(1, Point::new(1, 1)) {
        Err(error) => assert_eq!(error, GridError::bot_exists_in_location),
        _ => panic!("should have errored with bot exists in another location")
    };
}