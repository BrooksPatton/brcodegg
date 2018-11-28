extern crate brcodegg;
extern crate ggez;

use brcodegg::MainState;
use ggez::{conf, Context, graphics};

fn main() {
    let configuration = conf::Conf::new();
    let context = &mut Context::load_from_conf("brcodegg", "Brookzerker", configuration).unwrap();
    let (width, height) = graphics::get_size(context);
    let main_state = &mut MainState::new(width as f32, height as f32);

    ggez::event::run(context, main_state).unwrap();
}
