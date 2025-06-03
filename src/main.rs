use crate::Operation::{Forward, Home, Noop, TurnLeft, TurnRight};
use crate::Orientation::{East, North, South, West};

const WIDTH: isize = 400;
const HEIGHT: isize = WIDTH;

const HOME_Y: isize = HEIGHT / 2;
const HOME_X: isize = WIDTH / 2;

const STROKE_WIDTH: usize = 5;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Forward(isize),
    TurnLeft,
    TurnRight,
    Home,
    Noop(u8),
}

#[derive(Debug, Clone, Copy)]
enum Orientation {
    East,
    North,
    South,
    West,
}

#[derive(Debug)]
struct Artist {
    x: isize,
    y: isize,
    heading: Orientation,
}
impl Artist {
    fn new() -> Artist {
        Artist {
            x: HOME_X,
            y: HOME_Y,
            heading: North,
        }
    }
    fn home(&mut self) {
        self.x = HOME_X;
        self.y = HOME_Y;
    }
    fn forward(&mut self, distance: isize) {
        match self.heading {
            North => self.y += distance,
            South => self.y -= distance,
            West => self.x += distance,
            East => self.x -= distance,
        }
    }
}
fn main() {
    println!("Hello, world!");
}
