extern crate brcodegg;
extern crate ggez;

use brcodegg::MainState;
use ggez::{conf, graphics, Context};
use std::fs;

// Cannot be above 32767 due to us using u16 everywhere
// We are using u16 to be able to convert back into f32 when communicating with ggez
const WINDOW_WIDTH: u16 = 900;
const WINDOW_HEIGHT: u16 = 500;

fn main() {
    let mut conf_file = get_conf_file();
    let configuration = conf::Conf::from_toml_file(&mut conf_file).unwrap();

    configuration
        .window_mode
        .dimensions(WINDOW_WIDTH.into(), WINDOW_HEIGHT.into());

    let context = &mut Context::load_from_conf("brcodegg", "Brookzerker", configuration).unwrap();
    let num_bots_to_create = 1;
    let grid_cells = 25;
    let main_state =
        &mut MainState::new(WINDOW_WIDTH, WINDOW_HEIGHT, num_bots_to_create, grid_cells);

    ggez::event::run(context, main_state).unwrap();
}

fn get_conf_file() -> fs::File {
    let file = fs::File::open("conf.toml").unwrap();
    file
}
