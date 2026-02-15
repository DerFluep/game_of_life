use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use sdl3::EventPump;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::mouse::MouseButton;
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
        let cellsize = 5;
        Viewport {
            canvas,
            event_pump,
            cellsize,
            x_off: WIDTH / 2 - WINDOW_WIDTH / cellsize / 2,
            y_off: HEIGHT / 2 - WINDOW_HEIGHT / cellsize / 2,
        }
    }

    fn get_input(&mut self, run: &Arc<AtomicBool>, field: &Arc<Mutex<Vec<Vec<bool>>>>) -> bool {
        let mut change_cell = false;
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
                        let x_middle = self.x_off + WINDOW_WIDTH / 2 / self.cellsize;
                        let y_middle = self.y_off + WINDOW_HEIGHT / 2 / self.cellsize;
                        self.cellsize += 1;
                        self.x_off = x_middle - WINDOW_WIDTH / 2 / self.cellsize;
                        self.y_off = y_middle - WINDOW_HEIGHT / 2 / self.cellsize;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Minus),
                    ..
                } => {
                    if self.cellsize > 1 {
                        // FIX: only zoom out if far enough to the left and top.
                        // Else there could be an out of bounds crash
                        let x_middle = self.x_off + WINDOW_WIDTH / 2 / self.cellsize;
                        let y_middle = self.y_off + WINDOW_HEIGHT / 2 / self.cellsize;
                        self.cellsize -= 1;
                        self.x_off = x_middle - WINDOW_WIDTH / 2 / self.cellsize;
                        self.y_off = y_middle - WINDOW_HEIGHT / 2 / self.cellsize;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::H),
                    ..
                } => {
                    if self.x_off >= 2 {
                        self.x_off -= 2
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::L),
                    ..
                } => {
                    if self.x_off < WINDOW_WIDTH - 2 {
                        self.x_off += 2
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::J),
                    ..
                } => {
                    if self.y_off < WINDOW_HEIGHT - 2 {
                        self.y_off += 2
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::K),
                    ..
                } => {
                    if self.y_off >= 2 {
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
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    ..
                } => change_cell = true,
                _ => {}
            }
        }

        if change_cell {
            let x = self.event_pump.mouse_state().x().floor();
            let y = self.event_pump.mouse_state().y().floor();
            let x_cell = (x / self.cellsize as f32).floor() as usize + self.x_off;
            let y_cell = (y / self.cellsize as f32).floor() as usize + self.y_off;

            let mut field = field.lock().unwrap();
            field[y_cell][x_cell] = !field[y_cell][x_cell];
            dbg!(x_cell);
            dbg!(y_cell);
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

            if self.get_input(&run, field) {
                quit.store(true, Ordering::Relaxed);
                break 'running;
            }

            self.canvas.set_draw_color(Color::RGB(0, 0, 0));
            self.canvas.clear();

            let field = field.lock().unwrap();
            for y in 0..WINDOW_HEIGHT / self.cellsize {
                for x in 0..WINDOW_WIDTH / self.cellsize {
                    if field[y + self.y_off][x + self.x_off] {
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

                    if self.cellsize > 6 {
                        self.canvas.set_draw_color(Color::RGB(64, 64, 64));
                        self.canvas
                            .draw_rect(Rect::new(
                                (x * self.cellsize) as i32,
                                (y * self.cellsize) as i32,
                                self.cellsize as u32,
                                self.cellsize as u32,
                            ))
                            .unwrap();
                    }
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
