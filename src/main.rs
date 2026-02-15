mod game;
mod window;

use std::sync::{Arc, atomic::AtomicBool};

use game::Game;

use crate::window::Viewport;

const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;
const WINDOW_WIDTH: usize = 1000;
const WINDOW_HEIGHT: usize = 1000;

fn main() {
    if WINDOW_WIDTH > WIDTH || WINDOW_HEIGHT > HEIGHT {
        panic!("Window size can't be bigger than game size!");
    }

    let quit = Arc::new(AtomicBool::new(false));
    let run = Arc::new(AtomicBool::new(false));

    let game = Game::new();
    let game_field = game.get_field();
    let game_thread = game.run(Arc::clone(&run), Arc::clone(&quit));

    let mut viewport = Viewport::new();
    viewport.draw(&game_field, Arc::clone(&run), Arc::clone(&quit));

    game_thread.join().unwrap();
}
