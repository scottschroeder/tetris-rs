
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point {
            x,
            y
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Orientation {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy)]
pub enum RotationDirection {
    Clockwise,
    CounterClockwise,
}

/// Rotate point around origin
/// Based off of rotation matrix multiplication
pub fn rotate_transform(t:&Point, o: &Orientation) -> Point {
    match *o {
        Orientation::North => {
            Point {
                x: t.x,
                y: t.y,
            }
        },
        Orientation::East => {
            Point {
                x: t.y,
                y: -t.x,
            }
        }
        Orientation::South => {
            Point {
                x: -t.x,
                y: -t.y,
            }
        },
        Orientation::West => {
            Point {
                x: -t.y,
                y: t.x,
            }
        },
    }
}

pub fn rotate(orient: &Orientation, rotation: &RotationDirection) -> Orientation {
    match *rotation {
        RotationDirection::Clockwise => {
            match *orient {
                Orientation::North => Orientation::East,
                Orientation::East => Orientation::South,
                Orientation::South => Orientation::West,
                Orientation::West => Orientation::North,
            }
        }
        RotationDirection::CounterClockwise => {
            match *orient {
                Orientation::North => Orientation::West,
                Orientation::East => Orientation::North,
                Orientation::South => Orientation::East,
                Orientation::West => Orientation::South,
            }
        }
    }
}
