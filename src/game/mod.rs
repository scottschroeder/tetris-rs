use tile;
use Colorize;
use limit;
use input;

use std::default::Default;
use std::mem;
use input::CommandState;

// TODO not pub
pub mod transform;
pub mod tetromino;
mod generator;

pub use self::tetromino::Tetromino;
pub use self::transform::Point;
use self::tetromino::SlideDirection;
use self::transform::RotationDirection;

const TETRIS_BOARD_WIDTH: usize = 10;
const TETRIS_BOARD_HEIGHT: usize = 22;
const TETRIS_BOARD_VISIBLE_HEIGHT: usize = 20;
const TETRIS_BOARD_SPAWN: Point = Point { x: 5, y: 20 };

const TETRIS_BASE_GRAVITY: f64 = 0.5;
const TETRIS_LEVEL_GRAVITY: f64 = 0.05;


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TileColor {
    Red,
    Blue,
    Yellow,
    Cyan,
    Orange,
    Green,
    Purple,
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GameTile {
    Empty,
    Static(TileColor),
    Ghost(TileColor),
    Active(TileColor),
}

impl Default for GameTile {
    fn default() -> GameTile {
        GameTile::Empty
    }
}


#[derive(Debug, PartialEq, Clone, Default)]
pub struct Score {
    pub points: u64,
    pub garbage: u64,
}

impl Score {
    pub fn level(&self) -> u64 {
        self.garbage / 10
    }

    pub fn score(&self) -> u64 {
        self.points
    }

    fn wipe(&mut self, garbage: u64) {
        let multiplier = match garbage {
            0 => 0,
            1 => 40,
            2 => 100,
            3 => 300,
            4 => 1200,
            _ => unreachable!("Can not clear more than four rows in a single move!"),
        };
        self.points += (self.level() + 1) * multiplier;
        self.garbage += garbage;
    }
}

#[derive(Debug)]
pub struct Tetris {
    board: tile::TileBoard<GameTile>,
    tetromino: tetromino::Tetromino,
    hold: Option<tetromino::Tetromino>,
    hold_used: bool,
    tiles_created: usize,
    slide_timer: limit::RateLimiter,
    rotate_timer: limit::RateLimiter,
    gravity_timer: limit::RateLimiter,
    fast_fall_timer: limit::RateLimiter,
    lock_trigger: limit::SingleFireTrigger,
    lock_input_trigger: limit::SingleFireTrigger,
    command_state: CommandState,
    generator: generator::TetrominoGenerator,
    pub score: Score,
}


impl Tetris {
    pub fn new() -> Tetris {
        let mut t = Tetris {
            board: tile::TileBoard::new(TETRIS_BOARD_WIDTH, TETRIS_BOARD_HEIGHT),
            tetromino: Tetromino::new(),
            hold: None,
            hold_used: false,
            tiles_created: 0,
            slide_timer: limit::RateLimiter::new(0.05f64, Some(0.17f64)),
            rotate_timer: limit::RateLimiter::new(0.4f64, Some(0.4f64)),
            gravity_timer: limit::RateLimiter::new(TETRIS_BASE_GRAVITY, None),
            fast_fall_timer: limit::RateLimiter::new(0.05f64, None),
            lock_trigger: limit::SingleFireTrigger::new(2.0),
            lock_input_trigger: limit::SingleFireTrigger::new(0.5),
            command_state: CommandState::new(),
            generator: generator::TetrominoGenerator::new(),
            score: Score::default(),
        };
        t.new_tetromino();
        t
    }

    /// If a given piece can be placed entirely on the board without colliding with
    /// existing static tiles.
    pub fn check_piece(&self, piece: &tetromino::Tetromino) -> bool {
        piece.coordinates()
            .iter()
            .map(|p| {
                self.board.checked_get(p.x, p.y)
                    .map_or(false, |t| {
                        if let GameTile::Static(_) = t {
                            false
                        } else {
                            true
                        }
                    })
            })
            .fold(true, |a, b| a && b)
    }

    fn new_tetromino(&mut self) {
        let mut t = self.generator.pop();
        t.spawn(TETRIS_BOARD_SPAWN);
        self.tetromino = t;
    }

    fn swap(&mut self) {
        let mut swp = self.hold.take().unwrap_or_else(|| self.generator.pop());
        let previous_tetromino =
            mem::swap(&mut swp, &mut self.tetromino);
        self.hold = Some(swp);
        self.tetromino.spawn(TETRIS_BOARD_SPAWN);
        self.hold_used = true;
    }

    fn gravity_adjust(&mut self) {
        let level_multiplier = self.score.level() as f64;
        self.gravity_timer.repeat_rate = TETRIS_BASE_GRAVITY - TETRIS_LEVEL_GRAVITY * level_multiplier;
    }

    fn wipe_full_rows(&mut self) -> u64 {
        let mut row_reader = 0;
        let mut row_writer = 0;

        let mut rows_wiped = 0;

        while row_reader < TETRIS_BOARD_HEIGHT {
            let mut row_filled = 0;
            for idx in 0..TETRIS_BOARD_WIDTH {
                let tile = self.board.get(idx, row_reader).clone();
                if let GameTile::Static(_) = tile {
                    row_filled += 1;
                }
                self.board.set(idx, row_writer, tile);
            }
            row_reader += 1;
            if row_filled == TETRIS_BOARD_WIDTH {
                rows_wiped += 1;
            } else {
                row_writer += 1;
            }
        }
        while row_writer < TETRIS_BOARD_HEIGHT {
            for idx in 0..TETRIS_BOARD_WIDTH {
                self.board.set(idx, row_writer, GameTile::Empty);
            }
            row_writer += 1;
        }
        rows_wiped
    }

    fn altitude(&self, piece: &Tetromino) -> i32 {
        let ghost = self.ghost(piece);
        piece.origin.y - ghost.origin.y
    }


    fn ghost(&self, piece: &Tetromino) -> Tetromino {
        assert!(self.check_piece(piece), "Can not ghost an already unplaceable piece");
        let mut test = piece.clone();
        let mut ghost = test.clone();

        while self.check_piece(&test) {
            ghost = test.clone();
            test.move_down();
        }

        ghost
    }

    fn gravity(&mut self) {
        let event = match self.command_state.get_drop_speed() {
            input::DropSpeed::Fast => self.fast_fall_timer.get_event(),
            input::DropSpeed::Slow => self.gravity_timer.get_event(),
        };

        if event.is_some() {
            let mut test_piece = self.tetromino.clone();
            test_piece.move_down();
            if self.check_piece(&test_piece) {
                self.tetromino = test_piece;
            } else {
                self.lock_trigger.arm();
                self.lock_input_trigger.arm();
            }
        }
        if self.lock_trigger.is_ready() || self.lock_input_trigger.is_ready() {
            if self.tetromino == self.ghost(&self.tetromino) {
                self.lock();
            }
        }
    }

    fn slide(&mut self) {
        let event = self.slide_timer.get_event();
        let maybe_direction = self.command_state.do_slide();
        let mut test_piece = self.tetromino.clone();
        match (event, maybe_direction) {
            (Some(_), Some(input::SlideDirection::Left)) => test_piece.slide(SlideDirection::Left),
            (Some(_), Some(input::SlideDirection::Right)) => {
                test_piece.slide(SlideDirection::Right)
            }
            (_, None) => self.slide_timer.reset(),
            (None, _) => {}
        }
        if self.check_piece(&test_piece) {
            if self.tetromino != test_piece {
                if self.lock_trigger.is_armed() {
                    if self.altitude(&self.tetromino) < self.altitude(&test_piece) {
                        self.lock_trigger.reset();
                        self.lock_input_trigger.reset();
                    } else {
                        self.lock_input_trigger.soft_reset();
                    }
                }
                self.tetromino = test_piece;
            }
        }
    }

    fn check_and_update(&mut self, direction: RotationDirection) {
        let mut new_piece = self.tetromino.clone();
        new_piece.rotate(&direction);
        let translations = new_piece.wall_kick_options(&direction);
        for test_translate in &translations {
            let mut test_piece = new_piece.clone();
            test_piece.translate(test_translate);
            if self.check_piece(&test_piece) {
                self.tetromino = test_piece;
                self.lock_input_trigger.soft_reset();
                return;
            }
        }
    }

    fn rotate(&mut self) {
        let event = self.rotate_timer.get_event();
        let maybe_direction = self.command_state.do_rotate();
        match (event, maybe_direction) {
            (Some(_), Some(input::RotateDirection::Clockwise)) => {
                self.check_and_update(RotationDirection::Clockwise)
            }
            (Some(_), Some(input::RotateDirection::CounterClockwise)) => {
                self.check_and_update(RotationDirection::CounterClockwise)
            }
            (_, None) => self.rotate_timer.reset(),
            (None, _) => {}
        };
    }

    fn clear_timers(&mut self) {
        self.gravity_timer.reset();
        self.fast_fall_timer.reset();
        self.slide_timer.reset();
        self.rotate_timer.reset();
        self.lock_trigger.reset();
        self.lock_input_trigger.reset();
    }

    fn update_timers(&mut self, dt: f64) {
        self.gravity_timer.elapsed(dt);
        self.fast_fall_timer.elapsed(dt);
        self.slide_timer.elapsed(dt);
        self.rotate_timer.elapsed(dt);
        self.lock_trigger.elapsed(dt);
        self.lock_input_trigger.elapsed(dt);
    }


    fn lock(&mut self) {
        let ghost = self.ghost(&self.tetromino);
        for Point { x, y } in ghost.coordinates().iter() {
            self.board.set(*x as usize, *y as usize, GameTile::Static(ghost.color()));
        }
        self.new_tetromino();
        self.hold_used = false;
        let garbage = self.wipe_full_rows();
        self.score.wipe(garbage);
        self.command_state.clear_state();
        self.gravity_adjust();
        self.clear_timers();
        debug!("Score: {:?}: Level: {} Gravity: {}", self.score, self.score.level(), self.gravity_timer.repeat_rate);
    }


    pub fn on_update(&mut self, dt: f64) {
        self.update_timers(dt);

        if self.command_state.lock() {
            self.lock();
        } else if self.command_state.swap() && !self.hold_used {
            self.swap();
        } else {
            self.gravity();
            self.slide();
            self.rotate();
        }
    }

    pub fn get_command_state(&mut self) -> &mut CommandState {
        &mut self.command_state
    }

    pub fn get_preview(&self) -> [Tetromino; 3] {
        [
            self.generator.peek(0),
            self.generator.peek(1),
            self.generator.peek(2),
        ]
    }

    pub fn get_hold(&self) -> Option<Tetromino> {
        self.hold.clone()
    }

    pub fn get_board(&self) -> tile::TileBoard<GameTile> {
        let mut disp = self.board.clone();
        let ghost = self.ghost(&self.tetromino);
        for Point { x, y } in ghost.coordinates().iter() {
            disp.set(*x as usize, *y as usize, GameTile::Ghost(ghost.color()));
        }

        for Point { x, y } in self.tetromino.coordinates().iter() {
            disp.set(*x as usize, *y as usize, GameTile::Active(self.tetromino.color()));
        }

        disp.height = TETRIS_BOARD_VISIBLE_HEIGHT;
        return disp;
    }
}