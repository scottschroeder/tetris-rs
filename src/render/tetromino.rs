use color::{RawColor, Colorize};
use opengl_graphics::GlGraphics;
use graphics;
use tile;
use game::{Tetromino, Point};

use std::cmp;

const WIDTH: f64 = 4.5;
const HEIGHT: f64 = 2.8;

pub struct TetrominoRender {
    background: RawColor,
}

impl TetrominoRender {
    pub fn new(background: RawColor) -> TetrominoRender {
        TetrominoRender {
            background,
        }
    }

    pub fn dimmensions(&self, width: f64) -> (f64, f64, f64) {
        let tile_inner_size = (width * super::SQUARE_TILE_RATIO) / WIDTH;
        let border_size = (width * (1.0 - super::SQUARE_TILE_RATIO)) / (WIDTH + 1.0);
        let tile_size = tile_inner_size + border_size;
        let height = tile_size * HEIGHT + border_size;
        (tile_size, tile_inner_size, height)
    }

    pub fn render(&self, vp: graphics::Viewport, gl: &mut GlGraphics, x: f64, y: f64, width: f64, tetromino: Option<&Tetromino>) {
        use graphics::*;

        let (tile_size, tile_inner_size, height) = self.dimmensions(width);

        let background = rectangle::rectangle_by_corners(x, y, x + width, y + height);
        let square = rectangle::centered_square(0.0, 0.0, tile_inner_size / 2.0);


        gl.draw(vp, |ctx, gl| {
            rectangle(self.background, background, ctx.transform, gl);

            if let Some(piece) = tetromino {
                let (mid_x, mid_y) = tetromino_midpoint(piece);
                for point in piece.raw_points().iter() {
                    let tform = ctx.transform.trans(
                        x + width / 2.0 + tile_size * (point.x as f64 - mid_x) / 2.0,
                        y + height / 2.0 + tile_size * -(point.y as f64 - mid_y) / 2.0,
                    );
                    rectangle(piece.color().as_color(), square, tform, gl);
                }
            }
        })
    }
}


fn tetromino_midpoint(t: &Tetromino) -> (f64, f64) {
    let mut x_min = 100;
    let mut x_max = -100;
    let mut y_min = 100;
    let mut y_max = -100;
    for p in t.raw_points().iter() {
        x_min = cmp::min(p.x, x_min);
        x_max = cmp::max(p.x, x_max);
        y_min = cmp::min(p.y, y_min);
        y_max = cmp::max(p.y, y_max);
    }

    ((x_min + x_max) as f64 / 2.0, (y_min + y_max) as f64 / 2.0)
}
