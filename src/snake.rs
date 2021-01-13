use opengl_graphics::GlGraphics;
use piston::RenderArgs;

use crate::{Coord, RED};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug, Clone)]
pub struct Snake {
    direction: Direction,
    body: Vec<Coord>,
    size: f64,
}

impl Snake {
    pub fn new() -> Self {
        Snake {
            direction: Direction::RIGHT,
            body: vec![(10.0, 20.0)],
            size: 10.0,
        }
    }

    pub fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        gl.draw(args.viewport(), |c, gl| {
            for xy in &self.body {
                let transform = c.transform;
                let square = graphics::rectangle::square(xy.0, xy.1, self.size);
                graphics::rectangle(RED, square, transform, gl);
            }
        });
    }

    pub fn move_along(&mut self) -> Coord {
        let head = self.body[0];
        let new_head_position = get_new_coord(head, self.direction, self.size);

        let mut previous = (0.0, 0.0);
        for n in 0..self.body.len() {
            if n == 0 {
                previous = self.body[n];
                self.body[n] = new_head_position;
            } else {
                let aux = self.body[n];
                self.body[n] = previous;
                previous = aux;
            }
        }

        head
    }

    pub fn growing_up(&mut self) {
        let last = self.body.last().unwrap();
        let new_coord = get_new_coord(*last, self.direction, self.size);
        self.body.push(new_coord);
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn direction(&self) -> Direction {
        self.direction
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
