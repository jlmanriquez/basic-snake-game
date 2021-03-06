mod arena;
mod food;
mod snake;

use crate::arena::Arena;
use crate::snake::Direction;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{
    Button, EventLoop, EventSettings, Events, Key, PressEvent, RenderEvent, UpdateEvent,
    WindowSettings,
};

const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
const YELLOW: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

type Coord = (f64, f64);

fn main() {
    let opengl = OpenGL::V3_2;
    let width = 300;
    let height = 300;

    let mut window: Window = WindowSettings::new("spinning-square", [width, height])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut arena = Arena::new(GlGraphics::new(opengl), width, height);

    let mut events = Events::new(EventSettings::new()).ups(8);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            arena.render(&args);
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            let direction = match key {
                Key::Up => Direction::UP,
                Key::Right => Direction::RIGHT,
                Key::Down => Direction::DOWN,
                Key::Left => Direction::LEFT,
                _ => arena.snake().direction(),
            };

            arena.press_rows(direction);
        }

        if let Some(args) = e.update_args() {
            arena.update(&args);
        }
    }
}

fn get_new_coord(origin: Coord, direction: Direction, size: f64) -> Coord {
    match direction {
        Direction::UP => (origin.0, origin.1 - size),
        Direction::LEFT => (origin.0 - size, origin.1),
        Direction::RIGHT => (origin.0 + size, origin.1),
        Direction::DOWN => (origin.0, origin.1 + size),
    }
}
