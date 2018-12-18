use crate::game_grid::piece::Piece;
use crate::point::Point;

#[derive(PartialEq, Debug)]
pub struct GameGridCell {
    pub coordinates: Point<u16>,
    contains: Piece,
}

impl GameGridCell {
    pub fn new(x: u16, y: u16) -> GameGridCell {
        GameGridCell {
            coordinates: Point::new(x, y),
            contains: Piece::empty,
        }
    }
}

#[cfg(test)]
#[test]
fn game_grid_cell_new() {
    let cell = GameGridCell::new(1, 2);
    let coordinates = Point::new(1, 2);

    assert_eq!(cell.coordinates, coordinates);
    assert_eq!(cell.contains, Piece::empty);
}
// [
//     [{coordinates: {0, 0}, Piece::Bot{&Bot}}, {1, 0}, {2, 0}],
//     [{0, 1}, {1, 1}, {1, 2}]
//  ]
