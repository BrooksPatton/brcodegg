mod piece;

use std::collections::HashMap;
use crate::Point;

pub struct GameGrid {
    pub grid: HashMap<String, GameGridCell>,
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
fn place_bot_on_grid() {
    let game_grid = GameGrid::new(2, 2);
    let bot_index = 0;
    let bot_location = Point::new(1, 1);

    game_grid.place_bot(bot_index, bot_location);
    // TODO: finish creating the place bot on grid function (may need a cell piece structure)
}