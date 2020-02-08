use crate::ice::{Ice};
use crate::boat::{Boat};
use crate::physics_element::PhysicsElement;
use crate::vector::{Vector};
use sdl2::render::{WindowCanvas};
use rand::Rng;
use crate::{BOAT_SIZE, ICE_DECEL_FACTOR, BERG_MIN_SIZE, BERG_MAX_SIZE, GRID_SIZE, WIDTH, HEIGHT};
use crate::keyboard_state::KeyboardState;
use std::collections::HashMap;

pub struct World {
    size_x: u32,
    size_y: u32,
    ices: Vec<Ice>,
    boat: Boat
}

fn euc_distance(p1: &Vector, p2: &Vector) -> f32 {
    (((p1.x - p2.x).powf(2.0) + (p1.y - p2.y).powf(2.0)) as f32).sqrt()
}

fn reflect(subject_pos: Vector, subject_dir: Vector, object_pos: Vector, object_dir: Vector ) -> Vector {
    let n = subject_pos.sub(&object_pos).norm();
    let a1 = subject_dir.dot(&n);
    let a2 = object_dir.dot(&n);
    let optimized_p = (2.0 * (a1 - a2)) / 2.0;
    let new_direction = subject_dir.sub(&n.mul(optimized_p).mul(1.0));  // TODO: magic number
    return new_direction;
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
            let berg_size = rng.gen_range(BERG_MIN_SIZE, BERG_MAX_SIZE);
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

    // Compares line segments making up bergs to see if they actually interact
    fn is_real_collision(ice_a: &Ice, ice_b: &Ice) -> bool {
        return true;
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

    fn get_grid_region_bergs(grid: &HashMap<i32, HashMap<i32, Vec<Ice>>>, grid_x: i32, grid_y: i32) -> Result<Vec<Ice>, String> {
        let adj_grid_col= grid.get(&(grid_x));
        let mut in_grid = Vec::new();
        match adj_grid_col {
            Some(col) => {
                let adj_grid_row = col.get(&grid_y);
                match adj_grid_row {
                    Some(bergs) => {
                        let to_append = bergs.clone();
                        in_grid.append(&mut to_append.clone());
                    },
                    _ => {
                    }
                }
            },
            _ => {
            }
        }
        return Ok(in_grid);
    }

    // Called from event loop
    pub fn tick(&mut self, keyboard_state: &KeyboardState) {

        self.respond_to_input(keyboard_state);

        let boat_pos_start_tick = self.boat.position.clone();
        let boat_dir_start_tick = self.boat.direction.clone();

        // let dir = Vector{x:0.0, y:-1.0};
        // self.boat.direction = self.boat.direction.add(&dir);

        // Boat collisions
        let boat_collisions = self.find_collisions(&self.boat);
        for collision in boat_collisions {
            self.boat.direction = reflect(self.boat.position, self.boat.direction, collision.get_position(), collision.get_direction());
        }

        // Each tick, compute the current grid position of each iceberg
        let mut grid = HashMap::new();
        for ice in self.ices.iter() {
            let (grid_x, grid_y) = ice.calc_grid();
            // println!("Berg pos {:?} grid: {:?}, {:?}", ice.get_position(), grid_x, grid_y);
            let mut col = grid.entry(grid_x).or_insert(HashMap::new());
            let mut row = col.entry(grid_y).or_insert(Vec::new());
            row.push(ice.clone());
        }


        // Update the boat position even if it's not colliding
        self.boat.position = self.boat.position.add(&self.boat.direction);

        let current_ices = self.ices.clone();
        for mut ice in self.ices.iter_mut() {

            // If the ice is colliding with the boat, update it
            if euc_distance(&boat_pos_start_tick, &ice.position) < (self.boat.size + ice.size) as f32 {
                ice.direction = reflect(ice.position, ice.direction, boat_pos_start_tick, boat_dir_start_tick);
            }

            let (grid_x, grid_y) = ice.calc_grid();

            // Colocated bergs - hopefully only a few
            // TODO: This should check if the subject is close enough to the edge of the
            // TODO: grid to collide with something in an adjacent grid

            // TODO: This only returns the berg itself, not the bergs in same grid region
            // let mut others_in_grid = grid.get(&grid_x).unwrap().get(&grid_y).unwrap();
            let mut others_in_grid = World::get_grid_region_bergs(&grid, grid_x, grid_y).unwrap();
            let mut possible_collisions = Vec::new();
            possible_collisions.append(&mut others_in_grid.clone());

            // Grid regions are squares, so the berg can be colliding with objects in up to three
            // more grid regions adjacent to the one the center of the berg is in.
            // THIS IS TRUE ONLY WHEN THE GRID SIZE IS LARGER THAN THE LARGEST POSSIBLE ICEBERG
            // TODO: Move to function
            // TODO: Also check adjacent corners
            let x_1 = (ice.position.x - ice.size as f32) < (grid_x * GRID_SIZE as i32) as f32;
            let x_2 = (ice.position.x + ice.size as f32) > ((grid_x + 1) * GRID_SIZE as i32) as f32;
            let y_1 = (ice.position.y - ice.size as f32) < (grid_y * GRID_SIZE as i32) as f32;
            let y_2 = (ice.position.y + ice.size as f32) > ((grid_y + 1) * GRID_SIZE as i32) as f32;

            if x_1 {
                let to_append = World::get_grid_region_bergs(&grid, grid_x - 1, grid_y).unwrap();
                possible_collisions.append(&mut to_append.clone());
            }
            if x_2 {
                let to_append = World::get_grid_region_bergs(&grid, grid_x + 1, grid_y).unwrap();
                possible_collisions.append(&mut to_append.clone());
            }
            if y_1 {
                let to_append = World::get_grid_region_bergs(&grid, grid_x, grid_y - 1).unwrap();
                possible_collisions.append(&mut to_append.clone());
            }
            if y_2 {
                let to_append = World::get_grid_region_bergs(&grid, grid_x, grid_y + 1).unwrap();
                possible_collisions.append(&mut to_append.clone());
            }

            // Collisions from circular bounding box
            let collisions = World::find_collisions_2(&possible_collisions, &ice);

            for collision in collisions {

                // Don't collide with yourself
                if collision.position.x == ice.position.x && collision.position.y == ice.position.y {
                    continue;
                }

                ice.direction = reflect(ice.position, ice.direction, collision.position, collision.direction);
            }

            // Collisions reduce velocity overall
            ice.direction = ice.direction.mul(ICE_DECEL_FACTOR);
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