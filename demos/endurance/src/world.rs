use crate::ice::{Ice};
use crate::boat::{Boat};
use crate::physics_element::PhysicsElement;
use crate::vector::{Vector};
use sdl2::render::{WindowCanvas};
use rand::Rng;
use crate::BOAT_SIZE;
use crate::keyboard_state::KeyboardState;

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
        let boat = Boat::new(Vector{ x: (size_x / 2) as f32, y: (size_y / 2) as f32 }, BOAT_SIZE);
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
            let berg_size = rng.gen_range(5, 200);
            let x = rng.gen_range(berg_size + margin, self.size_x - (berg_size + margin));
            let y = rng.gen_range(-1 * self.size_y as i32 *4, self.size_y as i32 );
            let berg = Ice::new(Vector{x:x as f32, y:y as f32}, Vector{x:0.0, y:0.0}, berg_size);
            let collisions = self.find_collisions(&berg);

            if euc_distance(&self.boat.position, &berg.position) < (self.boat.size + *&berg.size) as f32 {
               continue;
            }

            if collisions.len() == 0 {
                self.ices.push(berg);
                num_bergs -= 1;
                println!("{:?} bergs remaining", num_bergs);
            }
        }
    }

    pub fn init_test(&mut self) {
        self.ices.push(Ice::new(Vector{x: 100.0, y: 500.0}, Vector{x:2.0, y: 0.0}.mul(0.0), 200));
        self.ices.push(Ice::new(Vector{x: 800.0, y: 500.0}, Vector{x:0.0, y: 0.0}.mul(0.0), 200));
        self.ices.push(Ice::new(Vector{x: 1500.0, y: 500.0}, Vector{x:-2.0, y: 0.0}.mul(0.0), 200));
    }


    // Returns copies of all icebergs that intersect with this one
    // Currently assumes all bergs are circles, which will need to be fixed
    fn find_collisions<S: PhysicsElement>(&self, ice: &S) -> Vec<Box<dyn PhysicsElement>> {
        let mut collisions: Vec<Box<dyn PhysicsElement>> = Vec::new();
        for other_ice in self.ices.iter() {
            if &other_ice.position != &ice.get_position() && euc_distance(&other_ice.position, &ice.get_position()) < (other_ice.get_size() + ice.get_size() as u32) as f32 {
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
            .filter(|other_ice| &other_ice.position != &ice.position)
            .collect();
        return collisions;
    }

    fn respond_to_input(&mut self, keyboard_state: &KeyboardState) {
        if keyboard_state.w {
            self.key_w();
        }
        if keyboard_state.a {
            self.key_a();
        }
        if keyboard_state.s {
            self.key_s();
        }
        if keyboard_state.d {
            self.key_d();
        }
    }

    // Called from event loop
    pub fn tick(&mut self, keyboard_state: &KeyboardState) {

        self.respond_to_input(keyboard_state);

        let boat_pos_start_tick = self.boat.position.clone();
        let boat_dir_start_tick = self.boat.direction.clone();

        // Boat collisions
        let boat_collisions = self.find_collisions(&self.boat);

        for collision in boat_collisions {
            let n = self.boat.position.sub(&collision.get_position()).norm();
            let a1 = self.boat.direction.dot(&n);
            let a2 = collision.get_direction().dot(&n);
            let optimized_p = (2.0 * (a1 - a2)) / 2.0;
            let new_direction = self.boat.direction.sub(&n.mul(optimized_p).mul(1.0));  // TODO: magic number
            self.boat.direction = new_direction;
        }

        // Update the boat position even if it's not colliding
        self.boat.position = self.boat.position.add(&self.boat.direction);

        let current_ices = self.ices.clone();
        for mut ice in self.ices.iter_mut() {

            // If the ice is colliding with the boat, update it
            if euc_distance(&boat_pos_start_tick, &ice.position) < (self.boat.size + ice.size) as f32 {
                let n = ice.position.sub(&boat_pos_start_tick).norm();
                let a1 = ice.direction.dot(&n);
                let a2 = boat_dir_start_tick.dot(&n);
                let optimized_p = (2.0 * (a1 - a2)) / 2.0;
                let new_direction = ice.direction.sub(&n.mul(optimized_p));
                ice.direction = new_direction;
            }

            // if ice is colliding with other ice, also update it
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
            }


            // Collisions reduce velocity overall
            ice.direction = ice.direction.mul(0.95);  // TODO: Magic number

            ice.position = ice.position.add(&ice.direction);
        }


        // Hack - push all bergs still colliding with the boat away
        for mut ice in self.ices.iter_mut() {
            while euc_distance(&boat_pos_start_tick, &ice.position) < (self.boat.size + ice.size) as f32 {
                ice.position = ice.position.add(&boat_dir_start_tick);
            }
        }

    }

    pub fn draw(&self, canvas: &mut WindowCanvas) {
        let offset = self.boat.position.sub(&Vector{x: (self.size_x / 2) as f32, y: (self.size_y / 2) as f32 });
        for berg in &self.ices {
           berg.draw_offset(canvas, &offset);
        }
        self.boat.draw_offset_circ(canvas, &offset);
    }
}