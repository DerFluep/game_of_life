use std::time::Duration;

use rand::Rng;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::rect::Rect;
use sdl3::render::Canvas;
use sdl3::video::Window;

const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;
const WINDOW_WIDTH: usize = 500;
const WINDOW_HEIGHT: usize = 500;

struct Game {
    field: Vec<Vec<bool>>,
}

impl Game {
    fn new() -> Game {
        let mut rng = rand::rng();
        let mut field = vec![vec![false; WIDTH]; HEIGHT];
        for y in field.iter_mut() {
            for cell in y.iter_mut() {
                *cell = rng.random_bool(0.1);
            }
        }
        Game { field }
    }

    fn update(&mut self) {
        let mut tmp_field = vec![vec![false; WIDTH]; HEIGHT];
        let directions = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                // count surrounding cells
                let mut count = 0;

                for (dy, dx) in directions {
                    let ny = y as isize + dy;
                    let nx = x as isize + dx;
                    if ny >= 0
                        && ny < HEIGHT as isize
                        && nx >= 0
                        && nx < WIDTH as isize
                        && self.field[ny as usize][nx as usize]
                    {
                        count += 1;
                    }
                }

                if self.field[y][x] {
                    tmp_field[y][x] = true;
                }
                // Game rules
                // Rule 1: if fewer than 2 neighbors, the cell dies
                // Rule 3: if more than 3 neighbors, the cell dies by overpopulation
                if !(2..3).contains(&count) {
                    tmp_field[y][x] = false;
                }
                // Rule 4: if exactly 3 neighbors, the cell reanimates
                if count == 3 {
                    tmp_field[y][x] = true;
                }
            }
        }
        self.field.clear();
        self.field.append(&mut tmp_field);
    }

    fn draw(&self, x_off: usize, y_off: usize, cellsize: usize, canvas: &mut Canvas<Window>) {
        for y in 0..WINDOW_WIDTH / cellsize {
            for x in 0..WINDOW_HEIGHT / cellsize {
                if self.field[y + y_off][x + x_off] {
                    canvas.set_draw_color(Color::RGB(255, 255, 255));
                } else {
                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                }
                canvas
                    .fill_rect(Rect::new(
                        (x * cellsize) as i32,
                        (y * cellsize) as i32,
                        cellsize as u32,
                        cellsize as u32,
                    ))
                    .unwrap();
            }
        }
        canvas.present();
    }
}

fn main() {
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Game of life", WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas();
    let mut event_pump = sdl_context.event_pump().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut cellsize = 11;
    let mut x = WIDTH / 2;
    let mut y = HEIGHT / 2;
    let mut game = Game::new();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Equals),
                    ..
                } => {
                    if cellsize < 20 {
                        cellsize += 2
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Minus),
                    ..
                } => {
                    if cellsize > 2 {
                        cellsize -= 2
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::H),
                    ..
                } => {
                    if x > 2 {
                        x -= 2
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::L),
                    ..
                } => {
                    if x < WIDTH - WINDOW_WIDTH / cellsize {
                        x += 2
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::J),
                    ..
                } => {
                    if y < HEIGHT - WINDOW_HEIGHT / cellsize {
                        y += 2
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::K),
                    ..
                } => {
                    if y > 2 {
                        y -= 2
                    }
                }
                _ => {}
            }
        }

        game.update();
        game.draw(x, y, cellsize, &mut canvas);

        ::std::thread::sleep(Duration::from_millis(100));
    }
}
