mod game_grid_cell;
mod piece;

use std::collections::HashMap;
use crate::game_grid::game_grid_cell::GameGridCell;

pub struct GameGrid {
    pub grid: HashMap<String, GameGridCell>,
    pub width: u16,
    pub height: u16,
}

impl GameGrid {
    pub fn new(width: u16, height: u16) -> GameGrid {
        let mut grid = HashMap::new();

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
