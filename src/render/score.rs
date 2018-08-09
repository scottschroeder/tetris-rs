use color::{RawColor, Colorize};
use opengl_graphics::{GlGraphics, GlyphCache};
use graphics;
use tile;

use game::Score;

const SCORE_TEXT_HEIGHT: u32 = 5;

pub struct ScoreRender {
    background: RawColor,
    text_render: super::TextRender,
    ratio: f64,
    font: u32,
}

impl ScoreRender {
    pub fn new(color: RawColor, background: RawColor, ratio: f64, font: u32) -> ScoreRender {
        ScoreRender {
            background,
            text_render: super::TextRender::new(color),
            ratio,
            font,
        }
    }

    pub fn render(&self, vp: graphics::Viewport, gl: &mut GlGraphics, glyphs: &mut GlyphCache, x: f64, y: f64, width: f64, score: &Score) {
        use graphics::*;
        let height = 1.0 / self.ratio * (SCORE_TEXT_HEIGHT * self.font) as f64;
        let background = rectangle::rectangle_by_corners(x, y, x + width, y + height);

        let text_x = width * (1.0 - self.ratio) + x;
        let text_y = y + ((self.font / 2) as f64) + height * (1.0 - self.ratio);

        gl.draw(vp, |ctx, gl| {
            rectangle(self.background, background, ctx.transform, gl);
        });
        self.text_render.render(vp, gl, glyphs, text_x, text_y, self.font, &score_text(score))
    }
}


//fn max_font_size(glyphs: &mut GlyphCache, width: Option<f64>, height: Option<f64>) -> u32 {
//    let width_guess = width.map(|pix| )
//    let font =
//
//}


#[inline]
fn score_text(score: &Score) -> String {
    let mut points = score.score();
    let mut suffix = "";
    if points > 1_000_000_000 {
        points /= 1_000_000;
        suffix = "M";
    } else if points > 1_000_000 {
        points /= 1_000;
        suffix = "K";
    }
    // Must match SCORE_TEXT_HEIGHT
    format!("Level:\n{}\n\nScore:\n{}{}", score.level(), points, suffix)
}