use sdl2::render::{Canvas, WindowCanvas};
use crate::vector::Vector;

// Anything that's part of the physics/collision world
pub trait PhysicsElement {
    fn draw(&self, canvas: &mut WindowCanvas);
    fn draw_offset(&self, canvas: &mut WindowCanvas, offset: &Vector);
    fn draw_offset_circ(&self, canvas: &mut WindowCanvas, offset: &Vector);
    fn get_size(&self) -> u32;
    fn get_position(&self) -> Vector;
    fn get_direction(&self) -> Vector;
}



