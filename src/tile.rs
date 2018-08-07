
use std::default::Default;
use Colorize;

#[derive(Debug, Clone, PartialEq)]
pub struct TileBoard<T> {
    squares:  Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T: Clone + Default> TileBoard<T> {
    pub fn new(width: usize, height: usize) -> TileBoard<T> {
        TileBoard {
            squares: vec![T::default(); width * height],
            width,
            height,
        }
    }
    pub fn get(&self, idx: usize, idy: usize) -> &T {
        &self.squares[idy * self.width + idx]
    }

    pub fn checked_get(&self, idx: i32, idy: i32) -> Option<&T> {
        if idx >= 0 && idx < self.width as i32 {
            if idy >= 0 && idy < self.height as i32 {
                return Some(self.get(idx as usize, idy as usize))
            }
        }
        None
    }

    pub fn set(&mut self, idx: usize, idy: usize, tile: T) {
        self.squares[idy * self.width + idx] = tile
    }
}

