use sdl2::rect::Point;
use sdl2::render::{Canvas, WindowCanvas};
use rand::Rng;
use sdl2::pixels::Color;

// Represents a discrete piece of ice
#[derive(Debug)]
pub struct Ice {

    // TODO: Velocity, rotation, mass (maybe an InteractableElement trait or something)

    // Center of the berg
    pub position: Point,

    // Maximum radius of circle underlying iceberg
    pub size: u32,

    // Ordered list of distances from center
    zig_zags: Vec<u32>
}

impl Ice {

    // Create an empty ice
    pub fn new(position: Point, size: u32) -> Ice {
        let mut zig_zags = Vec::new();
        let mut rng = rand::thread_rng();
        for i in 0..12 {
            let zig_zag_factor = rng.gen_range(size - size/2, size);
            zig_zags.push(zig_zag_factor);
        }

        // Last one should be the same as the first so the shape is closed
        zig_zags.push(*zig_zags.get(0).unwrap());
        Ice{position, size, zig_zags}
    }

    // Draw the ice to the canvas
    pub fn draw(&self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(Color::RGB(228, 240, 253));

        // Rotate a point around the circle representing the iceberg, changing the radius of the point to create jagged edges
        let point_x = self.position.x;
        let point_y = self.position.y + self.size as i32;
        let mut rng = rand::thread_rng();
        let mut points = Vec::new();
        for i in 0..13 {
            let angle = i * 30;
            let zig_zag_factor = self.zig_zags.get(i).unwrap();
            let zig_zagged_point_y = self.position.y + *zig_zag_factor as i32;
            let angle_rad = angle as f64 * std::f64::consts::PI / 180 as f64;
            let r_x = angle_rad.cos() * (point_x as f64 - self.position.x as f64) - angle_rad.sin() * (zig_zagged_point_y as f64- self.position.y as f64) + self.position.x as f64;
            let r_y = angle_rad.sin() * (point_x as f64 - self.position.x as f64) - angle_rad.cos() * (zig_zagged_point_y as f64- self.position.y as f64) + self.position.y as f64;
            points.push(Point::new(r_x as i32, r_y as i32));
        }

        // Connect the points of the iceberg polygon with lines
        for i in 0..points.len() - 1 {
            let p1 = points.get(i).unwrap();
            let p2 = points.get(i+1).unwrap();
            canvas.draw_line(Point::new(p1.x, p1.y), Point::new(p2.x, p2.y));
        }
    }
}