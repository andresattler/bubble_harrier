mod bundles;
mod components;
mod game;
mod util;

fn main() {
    let mut game = game::Game::new();
    game.run()
}
