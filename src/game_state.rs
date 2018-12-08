use point::Point;

#[derive(Serialize, Deserialize)]
pub struct GameState {
    width: f32,
    height: f32,
    current_location_x: f32,
    current_location_y: f32
}

impl GameState {
    pub fn new(width: f32, height: f32, location: Point) -> GameState {
        GameState {
            width,
            height,
            current_location_x: location.x,
            current_location_y: location.y
        }
    }
}

#[cfg(test)]
#[test]
fn game_state_new() {
    use point::Point;

    let location = Point::new(55.0, 45.0);
    let game_state = GameState::new(100.0, 101.0, location);

    assert_eq!(game_state.width, 100.0);
    assert_eq!(game_state.height, 101.0);
    assert_eq!(game_state.current_location_x, 55.0);
    assert_eq!(game_state.current_location_y, 45.0);
}