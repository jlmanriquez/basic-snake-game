use crate::{Coord, YELLOW};
use opengl_graphics::GlGraphics;
use piston::RenderArgs;

#[derive(Debug, Copy, Clone)]
pub struct Food {
    coord: Coord,
    color: [f32; 4],
    size: f64,
}

impl Food {
    pub fn new(coord: Coord) -> Self {
        Food {
            coord,
            color: YELLOW,
            size: 10.0,
        }
    }

    pub fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        let square = graphics::rectangle::square(self.coord.0, self.coord.1, self.size);
        let color = self.color;

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            graphics::rectangle(color, square, transform, gl);
        });
    }

    pub fn coord(&self) -> Coord {
        self.coord
    }
}
