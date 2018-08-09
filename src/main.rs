extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate rand;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use opengl_graphics::{GlyphCache, TextureSettings};


mod tile;
mod game;
mod color;
mod limit;
mod input;
mod render;

use color::{RawColor, Colorize};

const WINDOW_HEIGHT: u32 = 1000;
const WINDOW_WIDTH: u32 = 800;
const BOARD_WIDTH: f64 = (WINDOW_WIDTH as f64 * 0.6);
const BOARD_LEFT: f64 = WINDOW_WIDTH as f64 / 2.0 - BOARD_WIDTH / 2.0;

const BOARD_RIGHT: f64 = BOARD_LEFT + BOARD_WIDTH;
const WINDOW_BOARD_SIDE: f64 = WINDOW_WIDTH as f64 - BOARD_RIGHT;

const SIDE_BOARD_WIDTH: f64 = (WINDOW_WIDTH as f64 - BOARD_WIDTH) * 0.8 / 2.0;
const UPCOMING_BOARD_LEFT: f64 = BOARD_RIGHT + WINDOW_BOARD_SIDE / 2.0 - SIDE_BOARD_WIDTH / 2.0;

const HOLD_BOARD_LEFT: f64 = WINDOW_BOARD_SIDE / 2.0 - SIDE_BOARD_WIDTH / 2.0;


pub struct App {
    gl: GlGraphics,
    glyphs: GlyphCache<'static>,
    tile_render: render::TileRender,
    tetromino_render: render::TetrominoRender,
    score_render: render::ScoreRender,
    game: game::Tetris,
    key_mapping: input::KeyMap,
    pause: bool,
}


impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        self.gl.draw(args.viewport(), |ctx, gl| {
            // Clear the screen.
            clear(self::color::BACKGROUND_COLOR, gl);
        });
        let vp = args.viewport();
        self.tile_render.render(vp, &mut self.gl, BOARD_LEFT, 10.0, BOARD_WIDTH, &self.game.get_board());

        let preview = self.game.get_preview();
        let mut preview_height = 10.0;
        for piece in preview.iter() {
            self.tetromino_render.render(vp, &mut self.gl, UPCOMING_BOARD_LEFT, preview_height, SIDE_BOARD_WIDTH, Some(&piece));
            preview_height += self.tetromino_render.dimmensions(SIDE_BOARD_WIDTH).2;
        }

        self.tetromino_render.render(vp, &mut self.gl, HOLD_BOARD_LEFT, 10.0, SIDE_BOARD_WIDTH, self.game.get_hold().as_ref());
        self.score_render.render(vp, &mut self.gl, &mut self.glyphs, HOLD_BOARD_LEFT, 500.0, SIDE_BOARD_WIDTH, &self.game.score);



    }

    fn update(&mut self, args: &UpdateArgs) {
        self.game.on_update(args.dt);
    }

    fn on_input(&mut self, inp: &piston::input::ButtonArgs) {
        let piston::input::ButtonArgs { state, button, .. } = inp;

        let command = if let piston::input::Button::Keyboard(key) = button {
            self.key_mapping.get(key)
        } else {
            None
        };

        trace!("{:?} {:?}", command, state);
        match (command, state) {
            (Some(c), piston::input::ButtonState::Press) => self.game.get_command_state().key_press(*c),
            (Some(c), piston::input::ButtonState::Release) => self.game.get_command_state().key_release(*c),
            (_, _) => {}
        }
    }
}

fn main() {
    pretty_env_logger::init();



    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
        "Tetris",
        [WINDOW_WIDTH, WINDOW_HEIGHT],
    )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let glyphs = {
        let font = include_bytes!("../assets/FiraSans-Regular.ttf");
        //let factory = window.factory.clone();
        GlyphCache::from_bytes(font, (), TextureSettings::new()).unwrap()
    };

    //let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();
    let mut key_map = input::KeyMap::new();
    key_map.insert(Key::Up, input::Command::RotateClockwise);
    key_map.insert(Key::Down, input::Command::DownFast);
    key_map.insert(Key::Left, input::Command::SlideLeft);
    key_map.insert(Key::Right, input::Command::SlideRight);
    key_map.insert(Key::Space, input::Command::Lock);
    key_map.insert(Key::C, input::Command::Swap);

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        glyphs,
        tile_render: render::TileRender::new(
            color::BOARD_COLOR,
        ),
        tetromino_render: render::TetrominoRender::new(
            color::BOARD_COLOR,
        ),
        score_render: render::ScoreRender::new(
            color::COLOR_GREY_LIGHT,
            color::COLOR_BLACK,
            0.9,
            32,
        ),
        game: game::Tetris::new(),
        key_mapping: key_map,
        pause: false,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(b) = e.button_args() {
            app.on_input(&b);
        }
        if let Some(r) = e.render_args() {
            app.render(&r);
        }
        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
