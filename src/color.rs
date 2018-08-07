// Tunable game colors


const COLOR_RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const COLOR_GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const COLOR_BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

const COLOR_YELLOW: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
const COLOR_CYAN: [f32; 4] = [0.0, 1.0, 1.0, 1.0];
const COLOR_PURPLE: [f32; 4] = [0.5, 0.0, 0.5, 1.0];
const COLOR_ORANGE: [f32; 4] = [1.0, 0.4, 0.0, 1.0];


const COLOR_BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const COLOR_GREY_DARK: [f32; 4] = [0.1, 0.1, 0.1, 1.0];
const COLOR_GREY_LIGHT: [f32; 4] = [0.6, 0.6, 0.6, 1.0];
const GHOST_TILE_OPACITY: f32 = 0.15;

pub const BACKGROUND_COLOR: [f32; 4] = [0.6; 4];
pub const BOARD_COLOR: [f32; 4] = COLOR_GREY_DARK;

use game;

pub type RawColor = [f32; 4];

pub trait Colorize {
    fn as_color(&self) -> [f32; 4];
}

impl Colorize for RawColor {
    fn as_color(&self) -> [f32; 4] {
        *self
    }
}


impl Colorize for game::TileColor {
    fn as_color(&self) -> [f32; 4] {
        match self {
            game::TileColor::Yellow => COLOR_YELLOW,
            game::TileColor::Cyan => COLOR_CYAN,
            game::TileColor::Purple => COLOR_PURPLE,
            game::TileColor::Orange => COLOR_ORANGE,
            game::TileColor::Blue => COLOR_BLUE,
            game::TileColor::Green => COLOR_GREEN,
            game::TileColor::Red => COLOR_RED,
        }
    }
}

impl Colorize for game::GameTile {
    fn as_color(&self) -> [f32; 4] {
        match self {
            game::GameTile::Empty => COLOR_BLACK,
            game::GameTile::Static(tc) => tc.as_color(),
            game::GameTile::Active(tc) => tc.as_color(),
            game::GameTile::Ghost(tc) => {
                let mut piece_color = tc.as_color();
                piece_color[3] = GHOST_TILE_OPACITY;
                piece_color
            }
        }
    }
}

