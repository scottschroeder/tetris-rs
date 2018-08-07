use color::{RawColor, Colorize};
use opengl_graphics::{GlGraphics};
use graphics;
use tile;

pub struct TileRender {
    background: RawColor,
}

impl TileRender {
    pub fn new(background: RawColor) -> TileRender {
        TileRender {
            background,
        }
    }

    pub fn render<T: Colorize + Clone + Default>(&self, vp: graphics::Viewport, gl: &mut GlGraphics, x: f64, y: f64, width: f64, table: &tile::TileBoard<T>) {
        use graphics::*;

        let tw = table.width;
        let th = table.height;

        let tile_inner_size = (width * super::SQUARE_TILE_RATIO) / (tw as f64);
        let border_size = (width * (1.0 - super::SQUARE_TILE_RATIO)) / (tw as f64 + 1.0);
        let tile_size = tile_inner_size + border_size;

        let height = tile_size * th as f64 + border_size;
        let background = rectangle::rectangle_by_corners(x, y, x + width, y + height);
        let square = rectangle::rectangle_by_corners(0.0, 0.0, tile_inner_size, tile_inner_size);

        gl.draw(vp, |ctx, gl| {
            rectangle(self.background, background, ctx.transform, gl);
            for i in 0..tw {
                for j in 0..th {
                    let tform = ctx.transform.trans(x + border_size + tile_size * i as f64, y + border_size + tile_size * (th-1-j) as f64);
                    rectangle(table.get(i, j).as_color(), square, tform, gl);
                }
            }
        })
    }
}
