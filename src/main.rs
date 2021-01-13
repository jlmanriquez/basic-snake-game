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

type Coord = (f64, f64);

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("spinning-square", [300, 300])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut arena = Arena::new(GlGraphics::new(opengl));

    let mut events = Events::new(EventSettings::new()).ups(5);
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
                _ => arena.snake().direction().clone(),
            };

            arena.press_rows(direction);
        }

        if let Some(args) = e.update_args() {
            arena.update(&args);
        }
    }
}
