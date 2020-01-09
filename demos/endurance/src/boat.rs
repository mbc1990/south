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

    pub fn new(position: Vector , size: u32) -> Boat {
        return Boat{direction: Vector{x: 0.0, y: 0.0}, position, size};
    }

}

impl PhysicsElement for Boat {
    fn draw(&self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(Color::RGB(213, 183, 143));
        canvas.fill_rect(Rect::new((self.position.x - (self.size / 2) as f32) as i32, (self.position.y - (self.size / 2) as f32) as i32, self.size, self.size));
    }

    fn draw_offset(&self, canvas: &mut WindowCanvas, offset: &Vector) {
        canvas.set_draw_color(Color::RGB(213, 183, 143));
        let offset_center = self.position.sub(offset);
        // println!("OFfset center: {:?}", offset_center);
        canvas.fill_rect(Rect::new((offset_center.x - (self.size / 2) as f32) as i32, (offset_center.y - (self.size / 2) as f32) as i32, self.size, self.size));
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
