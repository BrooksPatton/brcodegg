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

        match self.check_location_for_bot(&bot_location) {
            Some(cell) => match cell {
                Piece::bot_index(_) => Err(GridError::bot_exists_in_location)
            },
            None => {
                self.grid.insert(bot_location, piece);
                Ok(())
            }
        }
    }

    // pub fn move_bot(&mut self, bot_index: u16, new_location: Point) -> Result<(), GridError> {
    //     // is there a bot in the new location?
    //     // 
    // }

    fn check_location_for_bot(&self, location: &Point) -> Option<&Piece> {
        self.grid.get(&location)
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

#[test]
fn check_location_for_bot() {
    let mut game_grid = GameGrid::new(5, 5);
    let bot_location = Point::new(2, 2);
    let empty_location = Point::new(1, 1);

    game_grid.place_bot(0, bot_location.clone()).unwrap();

    let bot_should_be_found = game_grid.check_location_for_bot(&bot_location);
    let bot_should_not_be_found = game_grid.check_location_for_bot(&empty_location);

    assert_eq!(bot_should_be_found, Some(&Piece::bot_index(0)));
    assert_eq!(bot_should_not_be_found, None);
}

#[test]
fn move_bot_to_empty_cell() {
    let mut game_grid = GameGrid::new(50, 50);
    let bot_location = Point::new(2, 2);
    let target_location = Point::new(3, 2);
    let bot_index = 0;

    game_grid.place_bot(bot_index, bot_location).unwrap();
    game_grid.move_bot(bot_index, target_location).unwrap();

    assert_eq!(game_grid.check_location_for_bot(&bot_location), None);
    assert_eq!(game_grid.check_location_for_bot(&target_location), Some(&Piece::bot_index(bot_index)));
}