#[derive(Serialize, Deserialize)]
pub struct BotMove {
    pub x: f32,
    pub y: f32
}

impl BotMove {
    fn new(x: f32, y: f32) -> BotMove {
        BotMove {
            x,
            y
        }
    }
}

#[cfg(test)]
#[test]
fn bot_move_new() {
    let bot_move = BotMove::new(1.0, 2.0);

    assert_eq!(bot_move.x, 1.0);
    assert_eq!(bot_move.y, 2.0);
}