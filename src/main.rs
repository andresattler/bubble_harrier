#[macro_use]
extern crate serde_derive;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate nalgebra as na;
extern crate ncollide3d as nc;

mod bundles;
mod components;
mod game;
mod resources;
mod util;

fn main() {
    env_logger::init();
    let mut game = game::Game::new();
    game.run()
}
