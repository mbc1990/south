use crate::ice::{Ice};
use crate::boat::{Boat};
use crate::physics_element::PhysicsElement;
use crate::vector::{Vector};
use sdl2::render::{WindowCanvas};
use rand::Rng;
use crate::{BOAT_SIZE, ICE_DECEL_FACTOR, BERG_MIN_SIZE, BERG_MAX_SIZE, GRID_SIZE, WIDTH, HEIGHT, DEBUG_MODE};
use crate::keyboard_state::KeyboardState;
use std::collections::HashMap;
use core::cmp;

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
        self.boat.direction = self.boat.direction.add(&dir.mul(0.5));
    }
    pub fn key_s(&mut self) {
        let dir = Vector{x:0.0, y:1.0};
        // TODO: Constants for keyboard input magic numbers
        self.boat.direction = self.boat.direction.add(&dir.mul(0.5));
    }
    pub fn key_d(&mut self) {
        let dir = Vector{x:1.0, y:0.0};
        self.boat.direction = self.boat.direction.add(&dir.mul(0.5));
    }

    pub fn init_with_random_ice(&mut self, mut num_bergs: i32) {
        let margin = 10;
        let mut rng = rand::thread_rng();
        while num_bergs > 0 {
            let berg_size = rng.gen_range(BERG_MIN_SIZE, BERG_MAX_SIZE);
            let x = rng.gen_range(berg_size + margin, self.size_x - (berg_size + margin));
            let y = rng.gen_range(-1 * self.size_y as i32, self.size_y as i32 );

            // Debugging, randomly pick a direction
            let dir_x = rng.gen_range(-1.0,1.0);
            let dir_y = rng.gen_range(-1.0,1.0);
            let vel = rng.gen_range(0.0, 1.0);
            let berg = Ice::new(Vector{x:x as f32, y:y as f32}, Vector{x:dir_x, y:dir_y}.mul(vel), berg_size);

            // let berg = Ice::new(Vector{x:x as f32, y:y as f32}, Vector{x:0.0, y:0.0}, berg_size);
            let collisions = self.find_collisions(&berg);

            if euc_distance(&self.boat.position, &berg.position) < (self.boat.size * 3 + *&berg.size) as f32 {
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
        self.ices.push(Ice::new(Vector{x: 800.0, y: 100.0}, Vector{x:10.0, y: 0.0}.mul(0.0), 300));
        // self.ices.push(Ice::new(Vector{x: 1200.0, y: 200.0}, Vector{x:-10.0, y: 0.0}.mul(1.0), 100));
        // self.ices.push(Ice::new(Vector{x: 1200.0, y: 400.0}, Vector{x:-10.0, y: -5.0}.mul(1.0), 100));
    }


    // Returns copies of all icebergs that intersect with this one
    // Currently assumes all bergs are circles, which will need to be fixed
    // TODO: only used in init_with_random_ice
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

    fn find_boat_collisions(&self, ices: &Vec<Ice>) -> Vec<Ice> {
        let mut collisions = Vec::new();
        for other_ice in ices.iter() {

            let boat_pos = self.boat.get_position();

            // Check each of the circles that make up the boat
            // Bow
            let bow_circle_pos = Vector{x:boat_pos.x, y: boat_pos.y - (2.0*self.boat.size as f32 + self.boat.size as f32 / 4.0)};
            let dist = euc_distance(&other_ice.position, &bow_circle_pos);
            if dist < (other_ice.get_size() as f32 + (self.boat.size as f32 / 4.0))  {
                collisions.push(other_ice.clone());
                continue;
            }

            // Mid deck
            let front_circle_pos = Vector{x:boat_pos.x, y: boat_pos.y - (self.boat.size as f32 + self.boat.size as f32 / 2.0)};
            if euc_distance(&other_ice.position, &front_circle_pos) < (other_ice.get_size() as f32 + (self.boat.size as f32 / 2.0)) {
                collisions.push(other_ice.clone());
                continue;
            }

            // Center
            if &other_ice.position != &self.boat.get_position() && euc_distance(&other_ice.position, &self.boat.get_position()) < (other_ice.get_size() + self.boat.get_size() as u32) as f32 {
                collisions.push(other_ice.clone());
                continue;
            }

            // Rear
            let rear_circle_pos = Vector{x:boat_pos.x, y: boat_pos.y + (self.boat.size as f32 + self.boat.size as f32 / 2.0)};
            if euc_distance(&other_ice.position, &rear_circle_pos) < (other_ice.get_size() as f32 + (self.boat.size as f32 / 2.0)) {
                collisions.push(other_ice.clone());
                continue;
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

    // Implementation adapted from:
    // https://www.geeksforgeeks.org/check-if-two-given-line-segments-intersect/
    // TODO: Return enum not int
    fn orientation(p: Vector, q: Vector, r: Vector) -> i32 {

        let val = (q.y - p.y) * (r.x - q.x) - (q.x- p.x) * (r.y - q.y);
        if val == 0.0 {
            return 0;
        }
        if val > 0.0 {
            return 1;
        }
        return 2;
    }

    fn on_segment(p: Vector, q: Vector, r: Vector) -> bool {
        if (q.x <= f32::max(p.x, r.y) && q.x >= f32::min(p.x, r.x) && q.y <= f32::max(p.y, r.y) && q.y >= f32::min(p.y, r.y)) {
            return true;
        }
        return false;
    }


    // Compares line segments making up bergs to see if they actually interact
    fn is_real_collision(ice_a: &Ice, ice_b: &Ice) -> bool {
        for i in 0..ice_a.perimeter.len() - 1 {
            for k in 0..ice_b.perimeter.len() - 1 {
                let l1_p1 = ice_a.position.add(ice_a.perimeter.get(i).unwrap());
                let l1_p2 = ice_a.position.add(ice_a.perimeter.get(i + 1).unwrap());

                let l2_p1 = ice_b.position.add(ice_b.perimeter.get(k).unwrap());
                let l2_p2 = ice_b.position.add(ice_b.perimeter.get(k + 1).unwrap());

                if World::lines_intersect(l1_p1, l1_p2, l2_p1, l2_p2) {
                    return true;
                }
            }
        }
        return false;
    }

    fn lines_intersect(p1: Vector, q1: Vector, p2: Vector, q2: Vector) -> bool {

        let o1 = World::orientation(p1, q1, p2);
        let o2 = World::orientation(p1, q1, q2);
        let o3 = World::orientation(p2, q2, p1);
        let o4 = World::orientation(p2, q2, q1);

        // General case
        if (o1 != o2 && o3 != o4) {
            return true;
        }
        if (o1 == 0 && World::on_segment(p1, p2, q1)) {
            return true;
        }

        // p1, q1 and q2 are colinear and q2 lies on segment p1q1
        if (o2 == 0 && World::on_segment(p1, q2, q1)) {
            return true;
        }

        // p2, q2 and p1 are colinear and p1 lies on segment p2q2
        if (o3 == 0 && World::on_segment(p2, p1, q2)) {
            return true;
        }

        // p2, q2 and q1 are colinear and q1 lies on segment p2q2
        if (o4 == 0 && World::on_segment(p2, q1, q2)) {
            return true;
        }

        return false;
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
        let boat_collisions = self.find_boat_collisions(&self.ices);
        println!("Boat collisions: {:?}", boat_collisions.len());
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


        // Update the boat position
        self.boat.position = self.boat.position.add(&self.boat.direction);

        let current_ices = self.ices.clone();
        let mut total_collisions = 0;
        for mut ice in self.ices.iter_mut() {

            // If the ice is colliding with the boat, update it
            // TODO: Should this be behind an abstraction that shares geometry with the debug boat drawing mode?
            // Center circle
            if euc_distance(&boat_pos_start_tick, &ice.position) < (self.boat.size + ice.size) as f32 {
                ice.direction = reflect(ice.position, ice.direction, boat_pos_start_tick, boat_dir_start_tick);
            }

            // Front circle
            let front_pos = boat_pos_start_tick.add(&Vector{x:0.0, y: -1.0 * (self.boat.size as f32 + self.boat.size as f32/ 2.0)});
            if euc_distance(&front_pos, &ice.position) < ((self.boat.size as f32/ 2.0)+ ice.size as f32) as f32 {
                ice.direction = reflect(ice.position, ice.direction, front_pos, boat_dir_start_tick);
            }

            // Bow circle
            let bow_pos = boat_pos_start_tick.add(&Vector{x:0.0, y: -1.0 * (2.0 * self.boat.size as f32 + self.boat.size as f32/ 4.0)});
            if euc_distance(&bow_pos, &ice.position) < ((self.boat.size as f32/ 4.0)+ ice.size as f32) as f32 {
                ice.direction = reflect(ice.position, ice.direction, bow_pos, boat_dir_start_tick);
            }

            // Rear circle
            let front_pos = boat_pos_start_tick.sub(&Vector{x:0.0, y: -1.0 * (self.boat.size as f32 + self.boat.size as f32/ 2.0)});
            if euc_distance(&front_pos, &ice.position) < ((self.boat.size as f32/ 2.0)+ ice.size as f32) as f32 {
                ice.direction = reflect(ice.position, ice.direction, front_pos, boat_dir_start_tick);
            }

            let (grid_x, grid_y) = ice.calc_grid();

            // Colocated bergs - hopefully only a few
            let mut others_in_grid = World::get_grid_region_bergs(&grid, grid_x, grid_y).unwrap();
            let mut possible_collisions = Vec::new();
            possible_collisions.append(&mut others_in_grid.clone());

            // Grid regions are squares, so the berg can be colliding with objects in up to three
            // more grid regions adjacent to the one the center of the berg is in.
            // THIS IS TRUE ONLY WHEN THE GRID SIZE IS GREATER THAN 2 * MAX_BERG_SIZE
            // The bounds here are a little tricky. Our ice might be completely within its grid, but collide with
            // another berg at the edge of an adjacent grid
            let x_1 = (ice.position.x - ice.size as f32) < ((grid_x * GRID_SIZE as i32) + BERG_MAX_SIZE as i32) as f32;
            let x_2 = (ice.position.x + ice.size as f32) > (((grid_x + 1) * GRID_SIZE as i32) - BERG_MAX_SIZE as i32) as f32;
            let y_1 = (ice.position.y - ice.size as f32) < ((grid_y * GRID_SIZE as i32) + BERG_MAX_SIZE as i32) as f32;
            let y_2 = (ice.position.y + ice.size as f32) > (((grid_y + 1) * GRID_SIZE as i32) - BERG_MAX_SIZE as i32) as f32;

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

            // Upper left corner
            if x_1 && y_1 {
                let to_append = World::get_grid_region_bergs(&grid, grid_x - 1, grid_y - 1).unwrap();
                possible_collisions.append(&mut to_append.clone());
            }

            // Lower left corner
            if x_1 && y_2 {
                let to_append = World::get_grid_region_bergs(&grid, grid_x - 1, grid_y + 1).unwrap();
                possible_collisions.append(&mut to_append.clone());
            }

            // Upper right corner
            if x_2 && y_1 {
                let to_append = World::get_grid_region_bergs(&grid, grid_x + 1, grid_y - 1).unwrap();
                possible_collisions.append(&mut to_append.clone());
            }

            // Lower right corner
            if x_2 && y_2 {
                let to_append = World::get_grid_region_bergs(&grid, grid_x + 1, grid_y + 1).unwrap();
                possible_collisions.append(&mut to_append.clone());
            }

            // Collisions from circular bounding box
            let collisions = World::find_collisions_2(&possible_collisions, &ice);
            total_collisions += collisions.len();
            for collision in collisions {

                // Don't collide with yourself
                // TODO: Use uuid instead of position
                if collision.position.x == ice.position.x && collision.position.y == ice.position.y {
                    continue;
                }

                if (World::is_real_collision(&ice, &collision)) {
                    let pre_reflection = ice.direction.clone();
                    ice.direction = reflect(ice.position, ice.direction, collision.position, collision.direction);
                    // Hack - Give the berg a small boost
                    // TODO: We probably don't want to use this since it produces a lot of velocity in close quarters
                    // ice.direction = ice.direction.mul(1.020);
                    // println!("Reflecting {:?} to {:?}", pre_reflection, ice.direction);
                }
            }

            // Collisions reduce velocity overall
            ice.direction = ice.direction.mul(ICE_DECEL_FACTOR);
            ice.position = ice.position.add(&ice.direction);
        }
        println!("Total collisions: {:}", total_collisions);
    }

    pub fn get_offset(&self) -> Vector {
        return self.boat.position.sub(&Vector{x: (self.size_x / 2) as f32, y: (self.size_y / 2) as f32 });
    }

    pub fn draw(&self, canvas: &mut WindowCanvas) {
        let offset = self.boat.position.sub(&Vector{x: (self.size_x / 2) as f32, y: (self.size_y / 2) as f32 });
        for berg in &self.ices {
            berg.draw_offset(canvas, &offset);
        }
        if DEBUG_MODE {
            self.boat.draw_offset_circ(canvas, &offset);
        } else {
            self.boat.draw_offset_detail(canvas, &offset);
        }
    }
}