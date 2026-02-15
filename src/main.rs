mod game;
mod window;

use std::sync::{Arc, atomic::AtomicBool};

use game::Game;

use crate::window::Viewport;

const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;
const WINDOW_WIDTH: usize = 500;
const WINDOW_HEIGHT: usize = 500;

fn main() {
    let quit = Arc::new(AtomicBool::new(false));
    let run = Arc::new(AtomicBool::new(false));

    let game = Game::new();
    let game_field = game.get_field();
    let game_thread = game.run(Arc::clone(&run), Arc::clone(&quit));

    let mut viewport = Viewport::new();
    viewport.draw(&game_field, Arc::clone(&run), Arc::clone(&quit));

    game_thread.join().unwrap();
}
