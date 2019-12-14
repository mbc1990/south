use crate::ice::Ice;
use sdl2::render::{Canvas, WindowCanvas};


pub struct World {
    size_x: u32,
    size_y: u32,
    ice: Vec<Ice>
}

impl World {
    pub fn new(size_x: u32, size_y: u32) -> World {

        let num_bergs = 10;

        let ice = Vec::new();
        World{size_x, size_y, ice}
    }

    // Called from event loop
    pub fn tick(self) {

    }

    pub fn draw(&self, canvas: &mut WindowCanvas) {

    }
}