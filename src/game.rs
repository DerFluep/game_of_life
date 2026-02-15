use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self, JoinHandle},
    time::Duration,
};

use rand::Rng;

use crate::{HEIGHT, WIDTH};

pub struct Game {
    field: Arc<Mutex<Vec<Vec<bool>>>>,
}

impl Game {
    pub fn new() -> Game {
        let mut rng = rand::rng();
        let mut field = vec![vec![false; WIDTH]; HEIGHT];
        for y in field.iter_mut() {
            for cell in y.iter_mut() {
                *cell = rng.random_bool(0.1);
            }
        }
        Game {
            field: Arc::new(Mutex::new(field)),
        }
    }

    pub fn get_field(&self) -> Arc<Mutex<Vec<Vec<bool>>>> {
        Arc::clone(&self.field)
    }

    fn update(&self) {
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
        let mut field = self.field.lock().unwrap();
        for (y, row) in tmp_field.iter_mut().enumerate() {
            for (x, cell) in row.iter_mut().enumerate() {
                // count surrounding cells
                let mut count = 0;

                for (dy, dx) in directions {
                    let ny = y as isize + dy;
                    let nx = x as isize + dx;
                    if ny >= 0
                        && ny < HEIGHT as isize
                        && nx >= 0
                        && nx < WIDTH as isize
                        && field[ny as usize][nx as usize]
                    {
                        count += 1;
                    }
                }

                if field[y][x] {
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
        field.clear();
        field.append(&mut tmp_field);
        drop(field);
    }

    pub fn run(self, run: Arc<AtomicBool>, quit: Arc<AtomicBool>) -> JoinHandle<()> {
        thread::spawn(move || {
            let game = self;
            'running: loop {
                if quit.load(Ordering::Relaxed) {
                    break 'running;
                }
                if run.load(Ordering::Relaxed) {
                    game.update();
                }
                ::std::thread::sleep(Duration::from_millis(100));
            }
        })
    }
}
