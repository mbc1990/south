use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, WindowCanvas};
use rand::Rng;
use sdl2::pixels::Color;
use crate::vector::{Vector};
use crate::physics_element::{PhysicsElement};

// Represents a discrete piece of ice
#[derive(Debug, Clone)]
pub struct Boat {

    pub direction: Vector,

    pub position: Vector,

    pub size: u32
}

impl Boat {

    pub fn new(position: Vector ) -> Boat {
        return Boat{direction: Vector{x: 0.0, y: 0.0}, position, size: 50};
    }

}

impl PhysicsElement for Boat {
    fn draw(&self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(Color::RGB(213, 183, 143));
        canvas.fill_rect(Rect::new((self.position.x - 25.0) as i32, (self.position.y - 25.0) as i32, 25, 25));
    }

    fn get_size(&self) -> u32 {
        return self.size;
    }

    fn get_position(&self) -> Vector {
        return self.position;
    }

    fn get_direction(&self) -> Vector {
        return self.direction;
    }

}
