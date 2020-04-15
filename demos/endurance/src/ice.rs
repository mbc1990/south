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
    triangles: Vec<Vec<Vector>>
}

impl Ice {

    pub fn new(position: Vector, direction: Vector, size: u32) -> Ice {

        let num_sides = 5;
        let mut rng = rand::thread_rng();
        let mut perimeter  =  Vec::new();
        let mut triangles = Vec::new();
        let base_angle = 360.0 / num_sides as f32;

        for i in 0..num_sides {
            let dist = rng.gen_range(size/2, size);
            let angle = i as f32 * base_angle;
            let angle_rad = angle as f64 * std::f64::consts::PI / 180 as f64;
            let r_x = angle_rad.cos() * 0.0 - angle_rad.sin() * (dist as f64);
            let r_y = angle_rad.sin() * 0.0 - angle_rad.cos() * (dist as f64);
            perimeter.push(Vector{x: r_x as f32, y: r_y as f32 });

            // triangles (in local space) to be used for rendering later
            if i > 0 {
                let p1 = Vector{x: 0.0, y: 0.0};
                let p2 = perimeter.get(i-1).unwrap().clone();
                let p3 = perimeter.get(i).unwrap().clone();
                let triangle = vec![p1, p2, p3];
                triangles.push(triangle);
            }
        }
        // Last triangle
        let p1 = Vector{x: 0.0, y: 0.0};
        let p2 = perimeter.first().unwrap().clone();
        let p3 = perimeter.last().unwrap().clone();
        triangles.push(vec![p1, p2, p3]);

        Ice{direction, position, size, perimeter, triangles}
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

                // Offset-adjusted points (position relative to an origin in the upper left corner of the visible screen)
                let pos_x = vertex.x + self.position.x - offset.x;
                let mut pos_y = vertex.y + self.position.y - offset.y;
                let pos_z = 0.0;

                // NDC System has bottom left origin, so adjust our y value (top left origin) into that system
                pos_y = HEIGHT as f32 - pos_y;

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
}
