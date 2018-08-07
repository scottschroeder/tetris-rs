use rand;
use rand::Rng;
use std::collections::VecDeque;

use std::fmt;

use game::tetromino::*;

const MIN_ELEMENTS: usize = 6;

fn draw_pieces(rng: &mut rand::ThreadRng) -> Vec<Shape> {
    let mut new_pieces = vec![
        Shape::O,
        Shape::I,
        Shape::T,
        Shape::L,
        Shape::J,
        Shape::S,
        Shape::Z,
    ];
    rng.shuffle(new_pieces.as_mut_slice());
    info!("Drew random tetronimos {:?}", new_pieces);
    new_pieces
}


pub struct TetrominoGenerator {
    queue: VecDeque<Tetromino>,
    rng: rand::ThreadRng,
}

impl fmt::Debug for TetrominoGenerator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.queue)
    }
}


impl TetrominoGenerator {
    pub fn new() -> Self {
        let mut g = TetrominoGenerator {
            queue: VecDeque::new(),
            rng: rand::thread_rng(),
        };
        g.extend();
        g
    }

    fn upcoming_queue_length(&self) -> usize {
        self.queue.len()
    }

    fn extend(&mut self) {
        let new_shapes = draw_pieces(&mut self.rng);
        for shape in new_shapes {
            self.queue.push_back(Tetromino::new_shape(shape))
        }
    }

    pub fn peek(&self, i: usize) -> Tetromino {
        assert!(i < MIN_ELEMENTS);
        self.queue.get(i).unwrap().clone()
    }

    pub fn pop(&mut self) -> Tetromino {
        if self.upcoming_queue_length() <= MIN_ELEMENTS {
            self.extend();
        }
        self.queue.pop_front().unwrap()
    }
}
