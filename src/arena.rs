use crate::food::Food;
use crate::snake::{Direction, Snake};
use crate::BLACK;
use opengl_graphics::GlGraphics;
use piston::{RenderArgs, UpdateArgs};
use rand::Rng;

pub struct Arena {
    gl: GlGraphics,
    snake: Snake,
    food: Vec<Food>,
    color: [f32; 4],
}

impl Arena {
    pub fn new(gl: GlGraphics) -> Self {
        Arena {
            gl,
            snake: Snake::new(),
            food: create_food(10),
            color: BLACK,
        }
    }

    pub fn snake(&self) -> &Snake {
        &self.snake
    }

    pub fn render(&mut self, args: &RenderArgs) {
        let color = self.color;
        let food = &self.food;
        let snake = &self.snake;

        self.gl.draw(args.viewport(), |_c, gl| {
            graphics::clear(color, gl);

            food.iter().for_each(|f| f.render(gl, args));
            snake.render(gl, args);
        });
    }

    pub fn press_rows(&mut self, direction: Direction) {
        self.snake.set_direction(direction);
    }

    pub fn update(&mut self, _args: &UpdateArgs) {
        let head = self.snake.move_along();
        let filter_food: Vec<Food> = self
            .food
            .clone()
            .into_iter()
            .filter(|f| f.coord() != head)
            .collect();

        // Al ser diferentes quiere decir que el filtro quito un Food existente
        // en la posicion head
        if filter_food.len() != self.food.len() {
            self.food = filter_food;
            self.snake.growing_up();
        }
    }
}

fn create_food(n: i32) -> Vec<Food> {
    let mut food = Vec::new();

    for _ in 0..n {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..29) * 10;
        let y = rng.gen_range(0..29) * 10;

        food.push(Food::new((x as f64, y as f64)));
    }

    food
}
