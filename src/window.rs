use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use sdl3::EventPump;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::rect::Rect;
use sdl3::render::Canvas;
use sdl3::video::Window;

use crate::{HEIGHT, WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH};

pub struct Viewport {
    canvas: Canvas<Window>,
    event_pump: EventPump,
    cellsize: usize,
    x_off: usize,
    y_off: usize,
}

impl Viewport {
    pub fn new() -> Viewport {
        let sdl_context = sdl3::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("Game of life", WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas();
        let event_pump = sdl_context.event_pump().unwrap();
        Viewport {
            canvas,
            event_pump,
            cellsize: 5,
            x_off: WIDTH / 2,
            y_off: HEIGHT / 2,
        }
    }

    fn get_input(&mut self, run: &Arc<AtomicBool>) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return true,
                Event::KeyDown {
                    keycode: Some(Keycode::Equals),
                    ..
                } => {
                    if self.cellsize < 20 {
                        self.cellsize += 1
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Minus),
                    ..
                } => {
                    if self.cellsize > 1 {
                        self.cellsize -= 1
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::H),
                    ..
                } => {
                    if self.x_off > WINDOW_WIDTH / self.cellsize / 2 + 2 {
                        self.x_off -= 2
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::L),
                    ..
                } => {
                    if self.x_off < WIDTH - WINDOW_WIDTH / self.cellsize / 2 {
                        self.x_off += 2
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::J),
                    ..
                } => {
                    if self.y_off < HEIGHT - WINDOW_HEIGHT / self.cellsize / 2 {
                        self.y_off += 2
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::K),
                    ..
                } => {
                    if self.y_off > WINDOW_HEIGHT / self.cellsize / 2 + 2 {
                        self.y_off -= 2
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    let run_tmp = run.load(Ordering::Relaxed);
                    run.store(!run_tmp, Ordering::Relaxed);
                }
                _ => {}
            }
        }
        false
    }

    pub fn draw(
        &mut self,
        field: &Arc<Mutex<Vec<Vec<bool>>>>,
        run: Arc<AtomicBool>,
        quit: Arc<AtomicBool>,
    ) {
        let interval = Duration::from_micros(1000000 / 60);
        'running: loop {
            let before = Instant::now();

            if self.get_input(&run) {
                quit.store(true, Ordering::Relaxed);
                break 'running;
            }
            let field = field.lock().unwrap();
            for y in 0..WINDOW_HEIGHT / self.cellsize {
                for x in 0..WINDOW_WIDTH / self.cellsize {
                    if field[y + self.y_off - (WINDOW_HEIGHT / self.cellsize) / 2]
                        [x + self.x_off - (WINDOW_WIDTH / self.cellsize) / 2]
                    {
                        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
                    } else {
                        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
                    }
                    self.canvas
                        .fill_rect(Rect::new(
                            (x * self.cellsize) as i32,
                            (y * self.cellsize) as i32,
                            self.cellsize as u32,
                            self.cellsize as u32,
                        ))
                        .unwrap();
                }
            }
            self.canvas.present();

            let elapsed = Instant::now() - before;
            if elapsed < interval {
                ::std::thread::sleep(interval - elapsed);
            }
        }
    }
}
