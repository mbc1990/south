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
        return Boat{direction: Vector{x: 0.0, y: -0.0}, position, size};
    }

    pub fn draw_offset_detail(&self, canvas: &mut WindowCanvas, offset: &Vector) {
        canvas.set_draw_color(Color::RGB(213, 183, 143));

        let offset_position = self.position.sub(offset);
        // For debugging TODO: Delete
        let p1 = Point::new(offset_position.x as i32, offset_position.y as i32);

        // Port bow
        let l1_p1_x  = offset_position.x - (self.size * 1) as f32;
        let l1_p1_y  = offset_position.y -  (self.size * 1) as f32;
        let l1_p1 = Point::new(l1_p1_x as i32, l1_p1_y as i32);
        let l1_p2_x = offset_position.x;
        let l1_p2_y = offset_position.y - (self.size * 3) as f32;
        let l1_p2 = Point::new(l1_p2_x as i32, l1_p2_y as i32);
        canvas.draw_line(l1_p1, l1_p2);

        // Starboard bow
        let l2_p1_x  = offset_position.x + (self.size * 1) as f32;
        let l2_p1_y  = offset_position.y -  (self.size * 1) as f32;
        let l2_p1 = Point::new(l2_p1_x as i32, l2_p1_y as i32);
        let l2_p2_x = offset_position.x;
        let l2_p2_y = offset_position.y - (self.size * 3) as f32;
        let l2_p2 = Point::new(l2_p2_x as i32, l2_p2_y as i32);
        canvas.draw_line(l2_p1, l2_p2);

        // Starboard side
        let l3_p1_x  = offset_position.x + (self.size * 1) as f32;
        let l3_p1_y  = offset_position.y - (self.size * 1) as f32;
        let l3_p1 = Point::new(l3_p1_x as i32, l3_p1_y as i32);
        let l3_p2_x = offset_position.x + (self.size * 1) as f32;
        let l3_p2_y = offset_position.y + (self.size * 1) as f32;
        let l3_p2 = Point::new(l3_p2_x as i32, l3_p2_y as i32);
        canvas.draw_line(l3_p1, l3_p2);

        // Port side
        let l4_p1_x  = offset_position.x - (self.size * 1) as f32;
        let l4_p1_y  = offset_position.y - (self.size * 1) as f32;
        let l4_p1 = Point::new(l4_p1_x as i32, l4_p1_y as i32);
        let l4_p2_x = offset_position.x - (self.size * 1) as f32;
        let l4_p2_y = offset_position.y + (self.size * 1) as f32;
        let l4_p2 = Point::new(l4_p2_x as i32, l4_p2_y as i32);
        canvas.draw_line(l4_p1, l4_p2);

        // Starboard backside
        let l5_p1_x  = offset_position.x + (self.size * 1) as f32;
        let l5_p1_y  = offset_position.y + (self.size * 1) as f32;
        let l5_p1 = Point::new(l5_p1_x as i32, l5_p1_y as i32);
        let l5_p2_x = offset_position.x + (self.size / 2) as f32;
        let l5_p2_y = offset_position.y + (self.size * 2) as f32;
        let l5_p2 = Point::new(l5_p2_x as i32, l5_p2_y as i32);
        canvas.draw_line(l5_p1, l5_p2);

        // port backside
        let l6_p1_x  = offset_position.x - (self.size * 1) as f32;
        let l6_p1_y  = offset_position.y + (self.size * 1) as f32;
        let l6_p1 = Point::new(l6_p1_x as i32, l6_p1_y as i32);
        let l6_p2_x = offset_position.x - (self.size / 2) as f32;
        let l6_p2_y = offset_position.y + (self.size * 2) as f32;
        let l6_p2 = Point::new(l6_p2_x as i32, l6_p2_y as i32);
        canvas.draw_line(l6_p1, l6_p2);

        // backside
        let l7_p1_x = offset_position.x - (self.size / 2) as f32;
        let l7_p1_y = offset_position.y + (self.size * 2) as f32;
        let l7_p1 = Point::new(l7_p1_x as i32, l7_p1_y as i32);
        let l7_p2_x = offset_position.x + (self.size / 2) as f32;
        let l7_p2_y = offset_position.y + (self.size * 2) as f32;
        let l7_p2 = Point::new(l7_p2_x as i32, l7_p2_y as i32);
        canvas.draw_line(l7_p1, l7_p2);


        // Sails
        canvas.set_draw_color(Color::RGB(216, 223, 235));

        // Front sail
        let l8_p1_x = offset_position.x - ((self.size) + (self.size/2)) as f32;
        let l8_p1_y = offset_position.y - (self.size + (self.size / 2)) as f32;
        let l8_p1 = Point::new(l8_p1_x as i32, l8_p1_y as i32);
        let l8_p2_x = offset_position.x + ((self.size) + (self.size/2)) as f32;
        let l8_p2_y = offset_position.y - (self.size + (self.size / 2)) as f32;
        let l8_p2 = Point::new(l8_p2_x as i32, l8_p2_y as i32);
        canvas.draw_line(l8_p1, l8_p2);

        // Middle sail
        let l9_p1_x = offset_position.x - ((self.size) + (self.size)) as f32;
        let l9_p1_y = offset_position.y;
        let l9_p1 = Point::new(l9_p1_x as i32, l9_p1_y as i32);
        let l9_p2_x = offset_position.x + ((self.size) + (self.size)) as f32;
        let l9_p2_y = offset_position.y;
        let l9_p2 = Point::new(l9_p2_x as i32, l9_p2_y as i32);
        canvas.draw_line(l9_p1, l9_p2);

        // Back sail
        let l10_p1_x = offset_position.x - ((self.size) + (self.size/2)) as f32;
        let l10_p1_y = offset_position.y + (self.size + (self.size / 2)) as f32;
        let l10_p1 = Point::new(l10_p1_x as i32, l10_p1_y as i32);
        let l10_p2_x = offset_position.x + ((self.size) + (self.size/2)) as f32;
        let l10_p2_y = offset_position.y + (self.size + (self.size / 2)) as f32;
        let l10_p2 = Point::new(l10_p2_x as i32, l10_p2_y as i32);
        canvas.draw_line(l10_p1, l10_p2);
    }

}

impl PhysicsElement for Boat {
    fn draw(&self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(Color::RGB(213, 183, 143));
        canvas.fill_rect(Rect::new((self.position.x - (self.size / 2) as f32) as i32, (self.position.y - (self.size / 2) as f32) as i32, self.size, self.size)).unwrap();
    }

    // TODO: Sort of weird for this to be on something called PhysicsElement
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
            // canvas.draw_line(Point::new(offset_position.x as i32, offset_position.y as i32), Point::new(p1.x, p1.y)).unwrap();
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
