use std::time::Duration;

use rand::Rng;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::rect::Rect;
use sdl3::render::Canvas;
use sdl3::video::Window;
pub struct Game {
    field: Vec<Vec<bool>>,
    width: usize,
    height: usize,
    window_width: usize,
    window_height: usize,
}

impl Game {
    pub fn new(width: usize, height: usize, window_width: usize, window_height: usize) -> Game {
        let mut rng = rand::rng();
        let mut field = vec![vec![false; width]; height];
        for y in field.iter_mut() {
            for cell in y.iter_mut() {
                *cell = rng.random_bool(0.1);
            }
        }
        Game {
            field,
            width,
            height,
            window_width,
            window_height,
        }
    }

    fn update(&mut self) {
        let mut tmp_field = vec![vec![false; self.width]; self.height];
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
        for (y, row) in tmp_field.iter_mut().enumerate() {
            for (x, cell) in row.iter_mut().enumerate() {
                // count surrounding cells
                let mut count = 0;

                for (dy, dx) in directions {
                    let ny = y as isize + dy;
                    let nx = x as isize + dx;
                    if ny >= 0
                        && ny < self.height as isize
                        && nx >= 0
                        && nx < self.width as isize
                        && self.field[ny as usize][nx as usize]
                    {
                        count += 1;
                    }
                }

                if self.field[y][x] {
                    *cell = true;
                }
                // Game rules
                // Rule 1: if fewer than 2 neighbors, the cell dies
                // Rule 3: if more than 3 neighbors, the cell dies by overpopulation
                if !(2..3).contains(&count) {
                    *cell = false;
                }
                // Rule 4: if exactly 3 neighbors, the cell reanimates
                if count == 3 {
                    *cell = true;
                }
            }
        }
        self.field.clear();
        self.field.append(&mut tmp_field);
    }

    fn draw(&self, x_off: usize, y_off: usize, cellsize: usize, canvas: &mut Canvas<Window>) {
        for y in 0..self.window_height / cellsize {
            for x in 0..self.window_width / cellsize {
                if self.field[y + y_off - (self.window_height / cellsize) / 2]
                    [x + x_off - (self.window_width / cellsize) / 2]
                {
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

    pub fn run(&mut self) {
        let sdl_context = sdl3::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window(
                "Game of life",
                self.window_width as u32,
                self.window_height as u32,
            )
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas();
        let mut event_pump = sdl_context.event_pump().unwrap();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        let mut cellsize = 5;
        let mut x = self.width / 2;
        let mut y = self.height / 2;
        let mut run = false;

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
                            cellsize += 1
                        }
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Minus),
                        ..
                    } => {
                        if cellsize > 1 {
                            cellsize -= 1
                        }
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::H),
                        ..
                    } => {
                        if x > self.window_width / cellsize / 2 + 2 {
                            x -= 2
                        }
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::L),
                        ..
                    } => {
                        if x < self.width - self.window_width / cellsize / 2 {
                            x += 2
                        }
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::J),
                        ..
                    } => {
                        if y < self.height - self.window_height / cellsize / 2 {
                            y += 2
                        }
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::K),
                        ..
                    } => {
                        if y > self.window_height / cellsize / 2 + 2 {
                            y -= 2
                        }
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Space),
                        ..
                    } => run = !run,
                    _ => {}
                }
            }

            if run {
                self.update();
            }
            self.draw(x, y, cellsize, &mut canvas);

            ::std::thread::sleep(Duration::from_millis(100));
        }
    }
}
