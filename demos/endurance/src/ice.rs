use sdl2::render::{WindowCanvas};
use rand::Rng;
use sdl2::pixels::Color;
use crate::vector::{Vector};
use crate::{GRID_SIZE, HEIGHT, WIDTH, BERG_MIN_SIZE, BERG_MAX_SIZE, DEBUG_MODE};
use sdl2::gfx::primitives::DrawRenderer;

// Represents a discrete piece of ice
#[derive(Debug, Clone)]
pub struct Ice {
    pub direction: Vector,
    pub position: Vector,
    // Maximum radius of circle underlying iceberg
    pub size: u32,
    pub perimeter: Vec<Vector>,
    pub inner_perimeter: Vec<Vector>,
    triangles: Vec<Vec<Vector>>
}

impl Ice {

    pub fn new(position: Vector, direction: Vector, size: u32) -> Ice {
        // Randomly generated perimeter & inner perimeter
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
        // TODO: Simplify math, or at least remove some of this ridiculous casting
        let mut perimeter  =  Vec::new();
        let mut inner_perimeter  =  Vec::new();
        let point_x = 0;
        // TODO: Offset can be removed, since it's set to 0 here
        let offset_position_x = 0.0;
        let offset_position_y = 0.0;
        let mut triangles = Vec::new();
        for i in 0..13 {
            let angle = i * 30;
            let zig_zag_factor = zig_zags.get(i).unwrap();
            let zig_zagged_point_y = offset_position_y + *zig_zag_factor as f32;
            let angle_rad = angle as f64 * std::f64::consts::PI / 180 as f64;
            let r_x = angle_rad.cos() * (point_x as f64 - offset_position_x as f64) - angle_rad.sin() * (zig_zagged_point_y as f64- offset_position_y as f64) + offset_position_x as f64;
            let r_y = angle_rad.sin() * (point_x as f64 - offset_position_x as f64) - angle_rad.cos() * (zig_zagged_point_y as f64- offset_position_y as f64) + offset_position_y as f64;
            perimeter.push(Vector{x: r_x as f32, y: r_y as f32 });

            //
            if i > 1 {
                let last_point = perimeter.get(i - 1).unwrap();
                let triangle = vec![Vector{x: r_x as f32, y: r_y as f32}, Vector{x: last_point.x, y: last_point.y}, Vector{x: offset_position_x, y: offset_position_y}];
                triangles.push(triangle);
            }

            let inner_zig_zag_factor = inner_zig_zags.get(i).unwrap();
            let inner_zig_zagged_point_y = offset_position_y + *inner_zig_zag_factor as f32;
            let angle_rad = angle as f64 * std::f64::consts::PI / 180 as f64;
            let r_x = angle_rad.cos() * (point_x as f64 - offset_position_x as f64) - angle_rad.sin() * (inner_zig_zagged_point_y as f64- offset_position_y as f64) + offset_position_x as f64;
            let r_y = angle_rad.sin() * (point_x as f64 - offset_position_x as f64) - angle_rad.cos() * (inner_zig_zagged_point_y as f64- offset_position_y as f64) + offset_position_y as f64;
            inner_perimeter.push(Vector{x: r_x as f32, y: r_y as f32 });
        }

        let last_point = perimeter.last().unwrap();
        let first_point = perimeter.first().unwrap();
        // TODO: Still broken, still slow
        let triangle = vec![Vector{x: first_point.x, y: first_point.y}, Vector{x: last_point.x, y: last_point.y}, Vector{x: offset_position_x, y: offset_position_y}];
        triangles.push(triangle);

        Ice{direction, position, size, perimeter, inner_perimeter, triangles}
    }

    pub fn calc_grid(&self) -> (i32, i32) {
        let mut grid_x = (self.position.x / GRID_SIZE as f32) as i32;
        let mut grid_y = (self.position.y / GRID_SIZE as f32) as i32;

        // Offset grid regions of negative positions due to integer division giving region 0
        // when a number x i -1.0 < x < 1.0
        // There's probably a more elegant way of doing this
        if self.position.x < 0.0 {
            grid_x -= 1;
        }
        if self.position.y < 0.0 {
            grid_y -= 1;
        }
        return (grid_x, grid_y);
    }

    pub fn get_vertices(&self, offset: &Vector) -> Vec<f32> {
        let mut ret = Vec::new();
        for trigon in &self.triangles {
            for vertex in trigon {

                // Offset-adjusted points
                let pos_x = vertex.x + self.position.x - offset.x;
                let pos_y = vertex.y + self.position.y - offset.y;
                let pos_z = 0.0;

                // Map these points into the normalized device coordinates space
                let input_range = WIDTH as f32;
                let output_range = 1.0 - -1.0;
                let output_x = (pos_x - 0.0)*output_range / input_range + -1.0;

                let input_range = HEIGHT as f32;
                let output_range = 1.0 - -1.0;
                let output_y = (pos_y - 0.0)*output_range / input_range + -1.0;
                ret.push(output_x);
                ret.push(output_y);
                ret.push(pos_z);

                // Colors
                ret.push(0.0);
                ret.push(1.0);
                ret.push(0.0);

            }
        }
        return ret;
    }

    pub fn draw(&self, canvas: &mut WindowCanvas, offset: &Vector) {
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

        for trigon in &self.triangles {
            unsafe {
                // gl::ClearColor(0.6, 0.0, 0.8, 1.0);
                // gl::Clear(gl::COLOR_BUFFER_BIT);
            }
            // TODO: Is there an opengl call to draw triangles that's faster than this?
            // TODO: This seems to ulimately use the polygon fill algorithm
            /*
            canvas.filled_trigon((trigon.0.x + self.position.x - offset.x) as i16,
                                 (trigon.0.y + self.position.y - offset.y) as i16,
                                 (trigon.1.x + self.position.x - offset.x) as i16,
                                 (trigon.1.y + self.position.y - offset.y) as i16,
                                 (trigon.2.x + self.position.x - offset.x) as i16,
                                 (trigon.2.y + self.position.y - offset.y) as i16,
            Color::RGB(228, 240, 253));
            */
        }

        /*

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
        if DEBUG_MODE {
            let _ = canvas.polygon(&xs, &ys, Color::RGB(192, 234, 242));
        } else {
            let _ = canvas.filled_polygon(&xs, &ys, Color::RGB(192, 234, 242));
            // let _ = canvas.aa_polygon(&xs, &ys, Color::RGB(192, 234, 242));
            // let _ = canvas.polygon(&xs, &ys, Color::RGB(192, 234, 242));
            // let _ = canvas.filled_polygon(&inner_xs, &inner_ys, Color::RGB(228, 240, 253));
        }
        */
    }
}
