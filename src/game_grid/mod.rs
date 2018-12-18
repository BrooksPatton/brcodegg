mod game_grid_cell;
mod piece;

use crate::game_grid::game_grid_cell::GameGridCell;

pub struct GameGrid {
    pub grid: Vec<Vec<GameGridCell>>,
    pub width: u16,
    pub height: u16,
}

impl GameGrid {
    pub fn new(width: u16, height: u16) -> GameGrid {
        let mut grid = Vec::new();

        for y in 0..height {
            let mut row = Vec::new();

            for x in 0..width {
                let cell = GameGridCell::new(x, y);

                row.push(cell);
            }

            grid.push(row);
        }

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
    let cell_0_0 = GameGridCell::new(0, 0);
    let cell_1_0 = GameGridCell::new(1, 0);
    let cell_0_1 = GameGridCell::new(0, 1);
    let cell_1_1 = GameGridCell::new(1, 1);
    let grid = vec![vec![cell_0_0, cell_1_0], vec![cell_0_1, cell_1_1]];

    assert_eq!(game_grid.grid, grid);
    assert_eq!(game_grid.width, 2);
    assert_eq!(game_grid.height, 2);
}
