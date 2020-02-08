use sdl2::rect::{Point, Rect};
use sdl2::render::{WindowCanvas};
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
        return Boat{direction: Vector{x: 0.0, y: -1.0}, position, size};
    }

}

impl PhysicsElement for Boat {
    fn draw(&self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(Color::RGB(213, 183, 143));
        canvas.fill_rect(Rect::new((self.position.x - (self.size / 2) as f32) as i32, (self.position.y - (self.size / 2) as f32) as i32, self.size, self.size)).unwrap();
    }

    fn draw_offset(&self, canvas: &mut WindowCanvas, offset: &Vector) {
        canvas.set_draw_color(Color::RGB(213, 183, 143));
        let offset_center = self.position.sub(offset);
        // println!("OFfset center: {:?}", offset_center);
        canvas.fill_rect(Rect::new((offset_center.x - (self.size / 2) as f32) as i32, (offset_center.y - (self.size / 2) as f32) as i32, self.size, self.size)).unwrap();
    }

    fn draw_offset_circ(&self, canvas: &mut WindowCanvas, offset: &Vector) {
        canvas.set_draw_color(Color::RGB(213, 183, 143));

        let offset_position = self.position.sub(offset);
        let point_x = offset_position.x;
        let mut points = Vec::new();
        for i in 0..13 {
            let angle = i * 30;
            let zig_zag_factor = &self.size;
            let zig_zagged_point_y = offset_position.y + *zig_zag_factor as f32;
            let angle_rad = angle as f64 * std::f64::consts::PI / 180 as f64;
            let r_x = angle_rad.cos() * (point_x as f64 - offset_position.x as f64) - angle_rad.sin() * (zig_zagged_point_y as f64- offset_position.y as f64) + offset_position.x as f64;
            let r_y = angle_rad.sin() * (point_x as f64 - offset_position.x as f64) - angle_rad.cos() * (zig_zagged_point_y as f64- offset_position.y as f64) + offset_position.y as f64;
            points.push(Point::new(r_x as i32, r_y as i32));
        }

        // Connect the points of the iceberg polygon with lines
        for i in 0..points.len() - 1 {
            let p1 = points.get(i).unwrap();
            let p2 = points.get(i+1).unwrap();

            // TODO: Handle err case
            canvas.draw_line(Point::new(p1.x, p1.y), Point::new(p2.x, p2.y)).unwrap();
            canvas.draw_line(Point::new(offset_position.x as i32, offset_position.y as i32), Point::new(p1.x, p1.y)).unwrap();
        }
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
