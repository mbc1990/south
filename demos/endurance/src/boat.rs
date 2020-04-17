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

        // TODO: Can be refactored out
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
                ret.push(0.0);
                ret.push(0.0);
                ret.push(1.0);
            }
        }
        return ret;
    }

    pub fn draw_gl(&self, program: &Program) {
        // let vertices = self.get_vertices();

    }

    pub fn draw(&self, canvas: &mut WindowCanvas, offset: &Vector) {
        canvas.set_draw_color(Color::RGB(213, 183, 143));

        let offset_position = self.position.sub(offset);
        let mut xs: Vec<i16> = Vec::new();
        let mut ys: Vec<i16> = Vec::new();

        for pt in self.perimeter.iter() {
            xs.push((offset_position.x + pt.x) as i16) ;
            ys.push((offset_position.y + pt.y) as i16) ;
        }

        let _ = canvas.filled_polygon(&xs, &ys, Color::RGB(213, 183, 143));

        // Deck bow deck structure
        let mut recs = Vec::new();
        recs.push(Rect::from(((offset_position.x - (self.size/ 2) as f32) as i32, (offset_position.y - (self.size as f32 * 1.75) )as i32, self.size, self.size*2)));
        recs.push(Rect::from(((offset_position.x - (self.size/ 2) as f32) as i32, (offset_position.y + self.size as f32 / 2.0) as i32, self.size, self.size)));

        canvas.set_draw_color(Color::RGB(148, 101, 37));
        let _ = canvas.draw_rects(&recs);


        // Sails
        canvas.set_draw_color(Color::RGB(216, 223, 235));

        // Front sail
        let l8_p1_x = offset_position.x - ((self.size) + (self.size/2)) as f32;
        let l8_p1_y = offset_position.y - (self.size + (self.size / 2)) as f32;
        let l8_p1 = Point::new(l8_p1_x as i32, l8_p1_y as i32);
        let l8_p2_x = offset_position.x + ((self.size) + (self.size/2)) as f32;
        let l8_p2_y = offset_position.y - (self.size + (self.size / 2)) as f32;
        let l8_p2 = Point::new(l8_p2_x as i32, l8_p2_y as i32);
        let _ = canvas.draw_line(l8_p1, l8_p2);

        // Middle sail
        let l9_p1_x = offset_position.x - ((self.size) + (self.size)) as f32;
        let l9_p1_y = offset_position.y;
        let l9_p1 = Point::new(l9_p1_x as i32, l9_p1_y as i32);
        let l9_p2_x = offset_position.x + ((self.size) + (self.size)) as f32;
        let l9_p2_y = offset_position.y;
        let l9_p2 = Point::new(l9_p2_x as i32, l9_p2_y as i32);
        let _ = canvas.draw_line(l9_p1, l9_p2);

        // Back sail
        let l10_p1_x = offset_position.x - ((self.size) + (self.size/2)) as f32;
        let l10_p1_y = offset_position.y + (self.size + (self.size / 2)) as f32;
        let l10_p1 = Point::new(l10_p1_x as i32, l10_p1_y as i32);
        let l10_p2_x = offset_position.x + ((self.size) + (self.size/2)) as f32;
        let l10_p2_y = offset_position.y + (self.size + (self.size / 2)) as f32;
        let l10_p2 = Point::new(l10_p2_x as i32, l10_p2_y as i32);
        let _ = canvas.draw_line(l10_p1, l10_p2);
    }

}

