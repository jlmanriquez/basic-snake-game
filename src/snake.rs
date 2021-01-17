use opengl_graphics::GlGraphics;
use piston::RenderArgs;

use crate::{get_new_coord, Coord, RED};

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

    pub fn get_head(&self) -> Coord {
        *self.body.get(0).unwrap()
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

        new_head_position
    }

    pub fn growing_up(&mut self) {
        let last = self.body.last().unwrap();
        let new_coord = get_new_coord(*last, self.direction, self.size);
        self.body.push(new_coord);
    }

    pub fn set_direction(&mut self, new_direction: Direction) {
        if (self.direction == Direction::UP && new_direction == Direction::DOWN)
            || (self.direction == Direction::DOWN && new_direction == Direction::UP)
            || (self.direction == Direction::RIGHT && new_direction == Direction::LEFT)
            || (self.direction == Direction::LEFT && new_direction == Direction::RIGHT)
        {
            return;
        }

        self.direction = new_direction;
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn is_my_body(&self, coord: Coord) -> bool {
        self.body
            .iter()
            .find(|b| b.0 == coord.0 && b.1 == coord.1)
            .is_some()
    }
}
