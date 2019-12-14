use sdl2::rect::Point;
use sdl2::render::{Canvas, WindowCanvas};

// Represents a discrete piece of ice
pub struct Ice {

    // TODO: Velocity, rotation, mass (maybe an InteractableElement trait or something)

    // Center of the berg
    position: Point,

    // Ordered list of distances from center
    zig_zags: Vec<i32>
}

impl Ice {

    // Create an empty ice
    pub fn new(position: Point) -> Ice {
        let zig_zags = Vec::new();


        Ice{zig_zags, position}
    }

    // Draw the ice to the canvas
    pub fn draw(self, canvas: &mut WindowCanvas) {

    }
}