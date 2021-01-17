use crate::food::Food;
use crate::snake::{Direction, Snake};
use crate::{get_new_coord, Coord, BLACK, BLUE};
use graphics::Context;
use opengl_graphics::GlGraphics;
use piston::{RenderArgs, UpdateArgs};
use rand::Rng;

pub struct Arena {
    gl: GlGraphics,
    snake: Snake,
    food: Vec<Food>,
    color: [f32; 4],
    width: u32,
    height: u32,
    game_over: bool,
}

impl Arena {
    pub fn new(gl: GlGraphics, width: u32, height: u32) -> Self {
        Arena {
            gl,
            snake: Snake::new(),
            food: Arena::create_food(10),
            color: BLACK,
            width,
            height,
            game_over: false,
        }
    }

    pub fn snake(&self) -> &Snake {
        &self.snake
    }

    pub fn render(&mut self, args: &RenderArgs) {
        if self.game_over {
            return;
        }

        let color = self.color;
        let food = &self.food;
        let snake = &self.snake;
        let width = self.width;
        let height = self.height;

        self.gl.draw(args.viewport(), |c, gl| {
            graphics::clear(color, gl);

            // Pain borders
            Arena::render_walls(width, height, c, gl);

            food.iter().for_each(|f| f.render(gl, args));
            snake.render(gl, args);
        });
    }

    pub fn press_rows(&mut self, direction: Direction) {
        if self.game_over {
            return;
        }

        self.snake.set_direction(direction);
    }

    pub fn update(&mut self, _args: &UpdateArgs) {
        if self.game_over {
            return;
        }

        let head = self.snake.get_head();
        let new_direction = self.snake.direction();
        let possible_new_head = get_new_coord(head, new_direction, 10.0);

        if self.snake.is_my_body(possible_new_head) {
            self.game_over = true;
            return;
        }

        if self.detect_wall_collision(possible_new_head) {
            self.game_over = true;
            return;
        }

        if self.food.is_empty() {
            self.game_over = true;
            return;
        }

        self.detect_food_and_eat(possible_new_head);

        self.snake.move_along();
    }

    fn detect_wall_collision(&mut self, head: Coord) -> bool {
        head.0 == 0.0
            || head.0 == (self.width - 10) as f64
            || head.1 == 0.0
            || head.1 == (self.height - 10) as f64
    }

    fn detect_food_and_eat(&mut self, head: Coord) -> bool {
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
            return true;
        }

        false
    }

    fn render_walls(width: u32, height: u32, c: Context, gl: &mut GlGraphics) {
        for i in 0..width / 10 {
            let top = graphics::rectangle::square((i * 10) as f64, 0.0, 10.0);
            graphics::rectangle(BLUE, top, c.transform, gl);

            let y = height - 10;
            let bottom = graphics::rectangle::square((i * 10) as f64, y as f64, 10.0);
            graphics::rectangle(BLUE, bottom, c.transform, gl);
        }

        for i in 0..height / 10 {
            let left = graphics::rectangle::square(0.0, (i * 10) as f64, 10.0);
            graphics::rectangle(BLUE, left, c.transform, gl);

            let x = width - 10;
            let right = graphics::rectangle::square(x as f64, (i * 10) as f64, 10.0);
            graphics::rectangle(BLUE, right, c.transform, gl);
        }
    }

    fn create_food(n: i32) -> Vec<Food> {
        let mut food = Vec::new();

        for _ in 0..n {
            let mut rng = rand::thread_rng();
            let x = rng.gen_range(1..29) * 10;
            let y = rng.gen_range(1..29) * 10;

            food.push(Food::new((x as f64, y as f64)));
        }

        food
    }
}
