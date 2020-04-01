use sdl2::rect::Point;
use sdl2::render::{WindowCanvas};
use rand::Rng;
use sdl2::pixels::Color;
use crate::vector::{Vector};
use crate::physics_element::PhysicsElement;
use crate::{GRID_SIZE, HEIGHT, WIDTH, BERG_MIN_SIZE, BERG_MAX_SIZE};
use sdl2::gfx::primitives::DrawRenderer;

// Represents a discrete piece of ice
#[derive(Debug, Clone)]
pub struct Ice {

    // TODO: Rotation, mass (maybe an InteractableElement trait or something)

    pub direction: Vector,

    // Center of the berg
    // pub position: Point,
    pub position: Vector,


    // Maximum radius of circle underlying iceberg
    pub size: u32,

    // Ordered list of distances from center
    zig_zags: Vec<u32>,

    pub perimeter: Vec<Vector>,

    pub inner_perimeter: Vec<Vector>
}

impl Ice {

    pub fn new(position: Vector, direction: Vector, size: u32) -> Ice {
        let mut zig_zags = Vec::new();
        let mut inner_zig_zags = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..12 {
            let zig_zag_factor = rng.gen_range(size/2, size);
            zig_zags.push(zig_zag_factor);
            inner_zig_zags.push(rng.gen_range(zig_zag_factor - (zig_zag_factor / 2), zig_zag_factor - (zig_zag_factor / 3)));
        }

        // Last one should be the same as the first so the shape is closed
        zig_zags.push(*zig_zags.get(0).unwrap());
        inner_zig_zags.push(*inner_zig_zags.get(0).unwrap());


        // Rotate a point around the circle representing the iceberg, changing the radius of the point to create jagged edges
        // TODO: Simplify math
        let mut perimeter  =  Vec::new();
        let mut inner_perimeter  =  Vec::new();
        let point_x = 0;
        let point_y = 0;
        let offset_position_x = 0.0;
        let offset_position_y = 0.0;
        for i in 0..13 {
            let angle = i * 30;
            let zig_zag_factor = zig_zags.get(i).unwrap();
            let zig_zagged_point_y = offset_position_y + *zig_zag_factor as f32;
            let angle_rad = angle as f64 * std::f64::consts::PI / 180 as f64;
            let r_x = angle_rad.cos() * (point_x as f64 - offset_position_x as f64) - angle_rad.sin() * (zig_zagged_point_y as f64- offset_position_y as f64) + offset_position_x as f64;
            let r_y = angle_rad.sin() * (point_x as f64 - offset_position_x as f64) - angle_rad.cos() * (zig_zagged_point_y as f64- offset_position_y as f64) + offset_position_y as f64;
            perimeter.push(Vector{x: r_x as f32, y: r_y as f32 });

            let inner_zig_zag_factor = inner_zig_zags.get(i).unwrap();
            let inner_zig_zagged_point_y = offset_position_y + *inner_zig_zag_factor as f32;
            let angle_rad = angle as f64 * std::f64::consts::PI / 180 as f64;
            let r_x = angle_rad.cos() * (point_x as f64 - offset_position_x as f64) - angle_rad.sin() * (inner_zig_zagged_point_y as f64- offset_position_y as f64) + offset_position_x as f64;
            let r_y = angle_rad.sin() * (point_x as f64 - offset_position_x as f64) - angle_rad.cos() * (inner_zig_zagged_point_y as f64- offset_position_y as f64) + offset_position_y as f64;
            inner_perimeter.push(Vector{x: r_x as f32, y: r_y as f32 });
        }

        Ice{direction, position, size, zig_zags, perimeter, inner_perimeter}
    }

    pub fn calc_grid(&self) -> (i32, i32) {
        let grid_x = (self.position.x / GRID_SIZE as f32) as i32;
        let grid_y = (self.position.y / GRID_SIZE as f32) as i32;
        return (grid_x, grid_y);
    }
}

impl PhysicsElement for Ice {

    // Draw the ice to the canvas
    fn draw(&self, canvas: &mut WindowCanvas) {
        self.draw_offset(canvas, &Vector{x:0.0, y:0.0});
    }

    fn draw_offset_circ(&self, _canvas: &mut WindowCanvas, _offset: &Vector) {}

    fn draw_offset(&self, canvas: &mut WindowCanvas, offset: &Vector) {
        // Don't draw if not visible
        let offset_position = self.position.sub(offset);
        let x_min = 0.0 - BERG_MAX_SIZE as f32;
        let x_max = WIDTH as f32 + BERG_MAX_SIZE as f32;
        let y_min = 0.0 - BERG_MIN_SIZE as f32;
        let y_max = HEIGHT as f32 + BERG_MAX_SIZE as f32;
        if !(offset_position.x > x_min && offset_position.x < x_max && offset_position.y > y_min && offset_position.y < y_max) {
            return;
        }

        canvas.set_draw_color(Color::RGB(228, 240, 253));

        let mut xs = Vec::new();
        let mut ys= Vec::new();

        let mut inner_xs = Vec::new();
        let mut inner_ys = Vec::new();

        // Connect the points of the iceberg polygon with lines
        for i in 0..self.perimeter.len() - 1 {
            let p1 = self.perimeter.get(i).unwrap();
            xs.push((p1.x + self.position.x - offset.x) as i16);
            ys.push((p1.y + self.position.y - offset.y) as i16);
            let p1 = self.inner_perimeter.get(i).unwrap();
            inner_xs.push((p1.x + self.position.x - offset.x) as i16);
            inner_ys.push((p1.y + self.position.y - offset.y) as i16);
        }
        // canvas.filled_polygon(&xs, &ys, Color::RGB(192, 234, 242));

        canvas.polygon(&xs, &ys, Color::RGB(192, 234, 242));
        // canvas.filled_polygon(&inner_xs, &inner_ys, Color::RGB(228, 240, 253));
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
