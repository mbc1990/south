use crate::ice::{Ice, Vector};
use sdl2::render::{Canvas, WindowCanvas};
use sdl2::rect::Point;
use rand::Rng;
use std::mem;


pub struct World {
    size_x: u32,
    size_y: u32,
    ices: Vec<Ice>
}

fn euc_distance(p1: &Vector, p2: &Vector) -> f32 {
    (((p1.x - p2.x).powf(2.0) + (p1.y - p2.y).powf(2.0)) as f32).sqrt()
}

impl World {
    pub fn new(size_x: u32, size_y: u32) -> World {
        // Populate the world with some randomly positioned ice bergs
        let ice = Vec::new();
        World{size_x, size_y, ices: ice}
    }

    pub fn init_with_random_ice(&mut self, mut num_bergs: i32) {
        let margin = 10;
        let mut rng = rand::thread_rng();
        while num_bergs > 0 {
            let berg_size = rng.gen_range(5, 200);
            let x = rng.gen_range(berg_size + margin, self.size_x - (berg_size + margin));
            let y = rng.gen_range(berg_size + margin, self.size_y - (berg_size + margin));
            let berg = Ice::new(Vector{x:x as f32, y:y as f32}, berg_size);
            let collisions = self.find_collisions(&berg);
            if collisions.len() == 0 {
                self.ices.push(berg);
                num_bergs -= 1;
                println!("{:?} bergs remaining", num_bergs);
            }
        }
    }

    pub fn init_test(&mut self) {
        // let berg = Ice::new(Point::new(161, 358), 73);
        // self.ices.push(berg);
    }


    // Returns all icebergs that intersect with this one
    // Currently assumes all bergs are circles, which will need to be fixed
    fn find_collisions(&self, ice: &Ice) -> Vec<&Ice> {
        let collisions = self.ices.iter()
            .filter(|other_ice| euc_distance(&other_ice.position, &ice.position) < (other_ice.size + ice.size) as f32)
            .collect();
        return collisions;
    }

    // Called from event loop
    pub fn tick(&mut self) {

        // Find all collisions for each iceberg, updating velocities
        for mut ice in self.ices.iter_mut() {
            ice.position.x += ice.direction.x;
            ice.position.y += ice.direction.y;
        }

    }

    pub fn draw(&self, canvas: &mut WindowCanvas) {
        for berg in &self.ices {
           berg.draw(canvas);
        }
    }
}