use sdl2::rect::{Point, Rect};
use sdl2::render::{WindowCanvas};
use sdl2::pixels::Color;
use crate::vector::{Vector};
use sdl2::gfx::primitives::DrawRenderer;
use crate::render_gl::Program;
use sdl2::ttf::get_linked_version;
use crate::{HEIGHT, WIDTH};

// Represents a discrete piece of ice
#[derive(Debug, Clone)]
pub struct Boat {
    pub direction: Vector,
    pub position: Vector,
    pub size: u32,
    pub perimeter: Vec<Vector>,
}

impl Boat {

    pub fn new(position: Vector , size: u32) -> Boat {
        let mut boat = Boat{direction: Vector{x: 0.0, y: -0.0}, position, size, perimeter: vec![] };
        boat.init_perimeter();
        return boat;
    }

    // TODO: No longer matches graphic boat
    fn init_perimeter(&mut self) {
        let l1_p1_x  = 0.0 - (self.size * 1) as f32;
        let l1_p1_y  = 0.0 - (self.size * 1) as f32;
        self.perimeter.push(Vector{x: l1_p1_x, y: l1_p1_y});

        let l1_p2_x = 0.0;
        let l1_p2_y = 0.0 - (self.size * 3) as f32;
        self.perimeter.push(Vector{x: l1_p2_x, y: l1_p2_y});

        let l2_p1_x  = (self.size * 1) as f32;
        let l2_p1_y  = 0.0 - (self.size * 1) as f32;
        self.perimeter.push(Vector{x: l2_p1_x, y: l2_p1_y});

        let l3_p2_x = (self.size * 1) as f32;
        let l3_p2_y = (self.size * 1) as f32;
        self.perimeter.push(Vector{x: l3_p2_x, y: l3_p2_y});

        let l5_p2_x = (self.size / 2) as f32;
        let l5_p2_y = (self.size * 2) as f32;
        self.perimeter.push(Vector{x: l5_p2_x, y: l5_p2_y});

        let l7_p1_x = 0.0 - (self.size / 2) as f32;
        let l7_p1_y = (self.size * 2) as f32;
        self.perimeter.push(Vector{x: l7_p1_x, y: l7_p1_y});

        let l6_p1_x  = 0.0 - (self.size * 1) as f32;
        let l6_p1_y  = 0.0 + (self.size * 1) as f32;
        self.perimeter.push(Vector{x: l6_p1_x, y: l6_p1_y});
    }

    pub fn get_vertices(&self, offset: &Vector) -> Vec<f32> {
        let mut ret = Vec::new();

        // The boat, like the icebergs, is composed of triangles

        // Bow
        let mut trigons = Vec::new();
        let mut bow = Vec::new();
        let l1_p1_x  = 0.0 - (self.size * 1) as f32;
        let l1_p1_y  = 0.0 - (self.size * 1) as f32;
        bow.push(Vector{x: l1_p1_x, y: l1_p1_y});
        let l1_p2_x = 0.0;
        let l1_p2_y = 0.0 - (self.size * 3) as f32;
        bow.push(Vector{x: l1_p2_x, y: l1_p2_y});
        let l2_p1_x  = (self.size * 1) as f32;
        let l2_p1_y  = 0.0 - (self.size * 1) as f32;
        bow.push(Vector{x: l2_p1_x, y: l2_p1_y});
        trigons.push(bow);

        // Main body/middle section - right side
        let mut mid_right = Vec::new();
        let p1x = 0.0 - (self.size * 1) as f32;
        let p1y = 0.0 - (self.size * 1) as f32;
        mid_right.push(Vector{x: p1x, y: p1y});

        let p2x = 0.0 + (self.size * 1) as f32;
        let p2y = 0.0 + (self.size * 1) as f32;
        mid_right.push(Vector{x: p2x, y: p2y});

        let p3x= (self.size * 1) as f32;
        let p3y= 0.0 - (self.size * 1) as f32;
        mid_right.push(Vector{x: p3x, y: p3y});
        trigons.push(mid_right);

        // Main body/middle section - left side
        let mut mid_left = Vec::new();
        let p1x = 0.0 - (self.size * 1) as f32;
        let p1y = 0.0 - (self.size * 1) as f32;
        mid_left.push(Vector{x: p1x, y: p1y});

        let p2x = 0.0 - (self.size * 1) as f32;
        let p2y = 0.0 + (self.size * 1) as f32;
        mid_left.push(Vector{x: p2x, y: p2y});

        let p3x= (self.size * 1) as f32;
        let p3y= 0.0 + (self.size * 1) as f32;
        mid_left.push(Vector{x: p3x, y: p3y});
        trigons.push(mid_left);

        // (Temporary) rear triangle
        let mut rear = Vec::new();
        let p1x = 0.0 - (self.size * 1) as f32;
        let p1y = 0.0 + (self.size * 1) as f32;
        rear.push(Vector{x: p1x, y: p1y});

        let p2x = 0.0 + (self.size * 1) as f32;
        let p2y = 0.0 + (self.size * 1) as f32;
        rear.push(Vector{x: p2x, y: p2y});

        let p3x = 0.0;
        let p3y = 0.0 + self.size as f32 * 1.5;
        rear.push(Vector{x: p3x, y: p3y});
        trigons.push(rear);


        // TODO: Can be refactored - logic mostly duplicated from iceberg vertex conversion
        for trigon in trigons {
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
                // 0.239, 0.172, 0.062
                ret.push(0.239);
                ret.push(0.172);
                ret.push(0.062);
            }
        }
        return ret;
    }
}

