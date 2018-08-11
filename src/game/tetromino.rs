use game::transform::{Point, Orientation, RotationDirection, rotate, rotate_transform};
use game::TileColor;


#[derive(Debug, Clone)]
pub enum SlideDirection {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    O,
    T,
    I,
    L,
    J,
    S,
    Z,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tetromino {
    shape: Shape,
    pub origin: Point,
    orientation: Orientation,
}

pub type TetronimoPoints = [Point; 4];

impl Tetromino {
    /// Creates a dummy tetromino
    pub fn new() -> Self {
        Tetromino {
            shape: Shape::O,
            origin: Point::default(),
            orientation: Orientation::North,
        }
    }

    /// Creates a tetromino with a desired shape
    pub fn new_shape(shape: Shape) -> Self {
        Tetromino {
            shape,
            origin: Point::default(),
            orientation: Orientation::North,
        }
    }

    pub fn spawn(&mut self, p: Point) {
        self.origin = p;
        self.orientation = Orientation::North;
    }

    pub fn slide(&mut self, direction: SlideDirection) {
        match direction {
            SlideDirection::Left => self.origin.x -= 1,
            SlideDirection::Right => self.origin.x += 1,
        }
    }

    pub fn move_down(&mut self) {
        self.origin.y -= 1;
    }

    pub fn translate(&mut self, offset: &Point) {
        self.origin.x += offset.x;
        self.origin.y += offset.y;
    }

    pub fn rotate(&mut self, rotation: &RotationDirection) {
        self.orientation = rotate(&self.orientation, rotation);
    }

    pub fn wall_kick_options(&self, direction: &RotationDirection) -> Vec<Point> {
        wall_kicks(&self.shape, &self.orientation, direction)
    }

    pub fn coordinates(&self) -> TetronimoPoints {
        let mut base_shape = self.raw_points();
        //trace!("Base Shape {:?}", base_shape);
        for p in base_shape.iter_mut() {
            let rot = rotate_transform(p, &self.orientation);
            //trace!("Rotate {:?}: {:?} -> {:?}", self.orientation, p, rot);
            *p = Point {
                x: (rot.x >> 1) + self.origin.x,
                y: (rot.y >> 1) + self.origin.y,
            };
            //trace!("Translate {:?}: {:?}", self.origin, p);

        }
        //trace!("Shape {:?}({:?}) at {:?} -> {:?}", self.shape, self.orientation, self.origin, base_shape);
        base_shape
    }

    pub fn raw_points(&self) -> TetronimoPoints {
        tetromino_points(self.shape)
    }

    pub fn color(&self) -> TileColor {
        match self.shape {
            Shape::O => {
                TileColor::Yellow
            }
            Shape::I => {
                TileColor::Cyan
            }
            Shape::T => {
                TileColor::Purple
            }
            Shape::L => {
                TileColor::Orange
            }
            Shape::J => {
                TileColor::Blue
            }
            Shape::S => {
                TileColor::Green
            }
            Shape::Z => {
                TileColor::Red
            }
        }
    }
}

fn tetromino_points(shape: Shape) -> TetronimoPoints {
    match shape {
        Shape::O => {
            [
                Point::new(-1, 1),
                Point::new(-1, -1),
                Point::new(1, 1),
                Point::new(1, -1),
            ]
        }
        Shape::I => {
            [
                Point::new(-3, 1),
                Point::new(-1, 1),
                Point::new(1, 1),
                Point::new(3, 1),
            ]
        }
        Shape::T => {
            [
                Point::new(0, 0),
                Point::new(-2, 0),
                Point::new(2, 0),
                Point::new(0, 2),
            ]
        }
        Shape::L => {
            [
                Point::new(0, 0),
                Point::new(-2, 0),
                Point::new(2, 0),
                Point::new(2, 2),
            ]
        }
        Shape::J => {
            [
                Point::new(0, 0),
                Point::new(-2, 0),
                Point::new(2, 0),
                Point::new(-2, 2),
            ]
        }
        Shape::S => {
            [
                Point::new(0, 0),
                Point::new(-2, 0),
                Point::new(0, 2),
                Point::new(2, 2),
            ]
        }
        Shape::Z => {
            [
                Point::new(0, 2),
                Point::new(-2, 2),
                Point::new(0, 0),
                Point::new(2, 0),
            ]
        }
    }
}

// All the Following information is from the SRS rotation model.
// Reference https://tetris.wiki/SRS

fn wall_kicks(shape: &Shape,
              orientation: &Orientation,
              direction: &RotationDirection)
              -> Vec<Point> {
    match *shape {
        Shape::I => {
            match (orientation, direction) {
                (&Orientation::North, &RotationDirection::Clockwise) => {
                    vec![Point::new(0, 0),
                         Point::new(1, 0),
                         Point::new(-2, 0),
                         Point::new(1, -2),
                         Point::new(-2, 1)]
                }
                (&Orientation::North, &RotationDirection::CounterClockwise) => {
                    vec![Point::new(0, 0),
                         Point::new(2, 0),
                         Point::new(-1, 0),
                         Point::new(2, 1),
                         Point::new(-1, -2)]
                }
                (&Orientation::East, &RotationDirection::Clockwise) => {
                    vec![Point::new(0, 0),
                         Point::new(-2, 0),
                         Point::new(1, 0),
                         Point::new(-2, -1),
                         Point::new(1, 2)]
                }
                (&Orientation::East, &RotationDirection::CounterClockwise) => {
                    vec![Point::new(0, 0),
                         Point::new(1, 0),
                         Point::new(-2, 0),
                         Point::new(1, -2),
                         Point::new(-2, 1)]
                }
                (&Orientation::South, &RotationDirection::Clockwise) => {
                    vec![Point::new(0, 0),
                         Point::new(-1, 0),
                         Point::new(2, 0),
                         Point::new(-1, 2),
                         Point::new(2, -1)]
                }
                (&Orientation::South, &RotationDirection::CounterClockwise) => {
                    vec![Point::new(0, 0),
                         Point::new(-2, 0),
                         Point::new(1, 0),
                         Point::new(-2, -1),
                         Point::new(1, 2)]
                }
                (&Orientation::West, &RotationDirection::Clockwise) => {
                    vec![Point::new(0, 0),
                         Point::new(2, 0),
                         Point::new(-1, 0),
                         Point::new(2, 1),
                         Point::new(-1, -2)]
                }
                (&Orientation::West, &RotationDirection::CounterClockwise) => {
                    vec![Point::new(0, 0),
                         Point::new(-1, 0),
                         Point::new(2, 0),
                         Point::new(-1, 2),
                         Point::new(2, -1)]
                }
            }
        }
        Shape::O => vec![Point::new(0, 0)],
        _ => {
            match (orientation, direction) {
                (&Orientation::North, &RotationDirection::Clockwise) => {
                    vec![Point::new(0, 0),
                         Point::new(-1, 0),
                         Point::new(-1, -1),
                         Point::new(0, 2),
                         Point::new(-1, 2)]
                }
                (&Orientation::North, &RotationDirection::CounterClockwise) => {
                    vec![Point::new(0, 0),
                         Point::new(1, 0),
                         Point::new(1, -1),
                         Point::new(0, 2),
                         Point::new(1, 2)]
                }
                (&Orientation::East, &RotationDirection::Clockwise) => {
                    vec![Point::new(0, 0),
                         Point::new(-1, 0),
                         Point::new(-1, 1),
                         Point::new(0, -2),
                         Point::new(-1, -2)]
                }
                (&Orientation::East, &RotationDirection::CounterClockwise) => {
                    vec![Point::new(0, 0),
                         Point::new(-1, 0),
                         Point::new(-1, 1),
                         Point::new(0, -2),
                         Point::new(-1, -2)]
                }
                (&Orientation::South, &RotationDirection::Clockwise) => {
                    vec![Point::new(0, 0),
                         Point::new(1, 0),
                         Point::new(1, -1),
                         Point::new(0, 2),
                         Point::new(1, 2)]
                }
                (&Orientation::South, &RotationDirection::CounterClockwise) => {
                    vec![Point::new(0, 0),
                         Point::new(-1, 0),
                         Point::new(-1, -1),
                         Point::new(0, 2),
                         Point::new(-1, 2)]
                }
                (&Orientation::West, &RotationDirection::Clockwise) => {
                    vec![Point::new(0, 0),
                         Point::new(1, 0),
                         Point::new(1, 1),
                         Point::new(0, -2),
                         Point::new(1, -2)]
                }
                (&Orientation::West, &RotationDirection::CounterClockwise) => {
                    vec![Point::new(0, 0),
                         Point::new(1, 0),
                         Point::new(1, 1),
                         Point::new(0, -2),
                         Point::new(1, -2)]
                }
            }
        }
    }
}
