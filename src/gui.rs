// GUI related imports
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use graphics::color::{BLACK, WHITE};

// Game state
use crate::game_state::GameState;
// Track of time
use std::time;

const SQUARE_SIZE: f64 = 15.0;
const REFRESH_RATE_MILIS: u64 = 100;

struct Cell {
    x: f64,
    y: f64,
    color: [f32; 4],
}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    window: Window,
    game_state: GameState,
    update_time: time::Instant,
}

impl App {
    pub fn new(game_state: GameState) -> App {
        // Change this to OpenGL::V2_1 if not working.
        let opengl = OpenGL::V3_2;

        // Create a Glutin window.
        let window: Window = WindowSettings::new("game-of-rust", [1920, 1080])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

        // Create a new game and run it.
        App {
            gl: GlGraphics::new(opengl),
            window,
            game_state,
            update_time: time::Instant::now()
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let mut cells: Vec<Cell> = Vec::new();
        let square = rectangle::square(0.0, 0.0, SQUARE_SIZE);

        for (ind_y, row) in self.game_state.get_matrix().iter().enumerate() {
            for (ind_x, cell) in row.iter().enumerate() {
                let (x, y) = (ind_x as f64 * SQUARE_SIZE, ind_y as f64 * SQUARE_SIZE);
                let color = match cell {
                    true => WHITE,
                    false => BLACK,
                };
                cells.push(Cell { x, y, color });
            }
        }

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            for cell in cells {
                let transform = c
                    .transform
                    .trans(cell.x, cell.y);

                // Draw a box rotating around the middle of the screen.
                rectangle(cell.color, square, transform, gl);
            }
        });
    }

    // Updates the game state
    fn update(&mut self, args: &UpdateArgs) {
        self.game_state.forward_one_step();
        self.update_time = time::Instant::now();
    }

    pub fn run(&mut self) {
        let mut events = Events::new(EventSettings::new());
        while let Some(e) = events.next(&mut self.window) {
            if let Some(args) = e.render_args() {
                self.render(&args);
            }

            if let Some(args) = e.update_args() {
                if self.update_time.elapsed() >= time::Duration::from_millis(REFRESH_RATE_MILIS) {
                    self.update(&args);
                }
            }
        }
    }
}
