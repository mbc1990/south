use sdl2::rect::Point;
use sdl2::render::{Canvas, WindowCanvas};
use rand::Rng;
use sdl2::pixels::Color;
use crate::vector::{Vector};

// Represents a discrete piece of ice
#[derive(Debug, Clone)]
pub struct Boat {

    pub direction: Vector,

    pub position: Vector,
}

impl Boat {

    pub fn new(position: Vector ) -> Boat {
        return Boat{direction: Vector{x: 0.0, y: 0.0}, position};
    }

    pub fn draw(&self, canvas: &mut WindowCanvas) {

    }

}

