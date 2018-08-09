use color::{RawColor, Colorize};
use opengl_graphics::{GlGraphics, GlyphCache};
use graphics;
use tile;
use graphics::character::CharacterCache;

pub struct TextRender {
    color: RawColor,
}

impl TextRender {
    pub fn new(color: RawColor) -> TextRender {
        TextRender {
            color,
        }
    }


    pub fn render(&self, vp: graphics::Viewport, gl: &mut GlGraphics, glyphs: &mut GlyphCache, x: f64, y: f64, font: u32, stext: &str) {
        use graphics::*;


        gl.draw(vp, |ctx, gl| {
            for (line_no, line) in stext.lines().enumerate() {
                let vertical_offset = (line_no as u32 * font) as f64;
                text::Text::new_color(self.color, font).draw(
                    line,
                    glyphs,
                    &ctx.draw_state,
                    ctx.transform.trans(x, y + vertical_offset), gl,
                ).unwrap();
            }
        })
    }
}

pub fn get_dimm(glyphs: &mut GlyphCache, font: u32, data: &str) -> (f64, f64) {
    let mut x = 0.0;
    let mut y = 0.0;
    for ch in data.chars() {
        let character = glyphs.character(font, ch).unwrap();
        x += character.width();
        if character.top() > y {
            y = character.top();
        }
    }
    (x, y)
}
