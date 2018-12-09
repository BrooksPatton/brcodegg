extern crate brcodegg;
extern crate ggez;

use brcodegg::{MainState};
use ggez::{conf, Context, graphics};
use std::fs;

fn main() {
    let mut conf_file = get_conf_file();
    let configuration = conf::Conf::from_toml_file(&mut conf_file).unwrap();
    let context = &mut Context::load_from_conf("brcodegg", "Brookzerker", configuration).unwrap();
    let (width, height) = graphics::get_size(context);
    let num_bots_to_create = 1;
    let main_state = &mut MainState::new(width as f32, height as f32, num_bots_to_create);

    ggez::event::run(context, main_state).unwrap();
}

fn get_conf_file() -> fs::File {
    let file = fs::File::open("conf.toml").unwrap();
    file
}