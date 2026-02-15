mod game;

use game::Game;

fn main() {
    let width = 1000;
    let height = 1000;
    let window_width = 500;
    let window_height = 500;
    let mut game = Game::new(width, height, window_width, window_height);
    game.run();
}
