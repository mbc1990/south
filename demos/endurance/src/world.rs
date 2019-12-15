use crate::ice::{Ice};
use crate::boat::{Boat};
use crate::physics_element::PhysicsElement;
use crate::vector::{Vector};
use sdl2::render::{Canvas, WindowCanvas};
use sdl2::rect::Point;
use rand::Rng;
use std::mem;


pub struct World {
    size_x: u32,
    size_y: u32,
    ices: Vec<Ice>,
    boat: Boat
}

fn euc_distance(p1: &Vector, p2: &Vector) -> f32 {
    (((p1.x - p2.x).powf(2.0) + (p1.y - p2.y).powf(2.0)) as f32).sqrt()
}

impl World {
    pub fn new(size_x: u32, size_y: u32) -> World {
        // Populate the world with some randomly positioned ice bergs
        let ice = Vec::new();
        let boat = Boat::new(Vector{ x: 400.0, y: 400.0 });
        World{size_x, size_y, ices: ice, boat: boat}
    }

    pub fn key_w(&mut self) {
        let dir = Vector{x:0.0, y:-1.0};
        self.boat.direction = self.boat.direction.add(&dir);
    }
    pub fn key_a(&mut self) {
        let dir = Vector{x:-1.0, y:0.0};
        self.boat.direction = self.boat.direction.add(&dir);
    }
    pub fn key_s(&mut self) {
        let dir = Vector{x:0.0, y:1.0};
        self.boat.direction = self.boat.direction.add(&dir);
    }
    pub fn key_d(&mut self) {
        let dir = Vector{x:1.0, y:0.0};
        self.boat.direction = self.boat.direction.add(&dir);
    }

    pub fn init_with_random_ice(&mut self, mut num_bergs: i32) {
        let margin = 10;
        let mut rng = rand::thread_rng();
        while num_bergs > 0 {
            let berg_size = rng.gen_range(5, 300);
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
    fn find_collisions<S: PhysicsElement>(&self, ice: &S) -> Vec<Box<dyn PhysicsElement>> {
        let mut collisions: Vec<Box<dyn PhysicsElement>> = Vec::new();
        for other_ice in self.ices.iter() {
            if euc_distance(&other_ice.position, &ice.get_position()) < (other_ice.get_size() + ice.get_size() as u32) as f32 {
               // collisions.push(Box::new(ice.clone()));
                collisions.push(Box::new(other_ice.clone()) as Box<dyn PhysicsElement>);
            }

        }
        return collisions;
    }

    // same thing as the other one
    fn find_collisions_2<'a>(ices: &'a Vec<Ice>, ice: &Ice) -> Vec<&'a Ice> {
        let collisions = ices.iter()
            .filter(|other_ice| euc_distance(&other_ice.position, &ice.position) < (other_ice.size + ice.size) as f32)
            .collect();
        return collisions;
    }

    // Called from event loop
    pub fn tick(&mut self) {

        // Find all collisions for each iceberg, updating velocities


        // Boat collisions
        /*
        let boat_collisions = self.find_collisions(&self.boat);

        for collision in boat_collisions {
            let n = self.boat.position.sub(&collision.get_position()).norm();
            let a1 = self.boat.direction.dot(&n);
            let a2 = collision.get_direction().dot(&n);
            let optimized_p = (2.0 * (a1 - a2)) / 2.0;
            let new_direction = self.boat.direction.sub(&n.mul(optimized_p));
            self.boat.direction = new_direction;
            self.boat.position = self.boat.position.add(&self.boat.direction);
        }
        */

        // Update the boat position even if it's not colliding
        self.boat.position = self.boat.position.add(&self.boat.direction);

        println!("Boat direction: {:?}", self.boat.direction);

        let current_ices = self.ices.clone();
        let mut rng = rand::thread_rng();
        for mut ice in self.ices.iter_mut() {
            let collisions = World::find_collisions_2(&current_ices, &ice);

            for collision in collisions {

                // Don't collide with yourself
                if collision.position.x == ice.position.x && collision.position.y == ice.position.y {
                    continue;
                }

                let n = ice.position.sub(&collision.position).norm();
                let a1 = ice.direction.dot(&n);
                let a2 = collision.direction.dot(&n);
                let optimized_p = (2.0 * (a1 - a2)) / 2.0;
                let new_direction = ice.direction.sub(&n.mul(optimized_p));
                ice.direction = new_direction;
                ice.position = ice.position.add(&ice.direction);
            }


            // Update position
            let dewonk_factor = rng.gen_range(0.9, 1.1);
            ice.position = ice.position.add(&ice.direction.mul(dewonk_factor));
        }

    }

    pub fn draw(&self, canvas: &mut WindowCanvas) {
        for berg in &self.ices {
           berg.draw(canvas);
        }
        self.boat.draw(canvas);
    }
}