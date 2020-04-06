use crate::ice::{Ice};
use crate::boat::{Boat};
use crate::vector::{Vector};
use sdl2::render::{WindowCanvas};
use rand::Rng;
use crate::{BOAT_SIZE, ICE_DECEL_FACTOR, BERG_MIN_SIZE, BERG_MAX_SIZE, GRID_SIZE, WIDTH, HEIGHT, DEBUG_MODE, BOAT_ACCELERATION};
use crate::keyboard_state::KeyboardState;
use std::collections::HashMap;
use core::cmp;
use crate::geometry::{reflect, lines_intersect, euc_distance};

pub struct World {
    size_x: u32,
    size_y: u32,
    ices: Vec<Ice>,
    boat: Boat
}

// TODO: We need to include the object (boat perimeter segment) velocity somehow
fn reflect_line(subject_pos: Vector, subject_dir: Vector, object_p1: Vector, object_p2: Vector) -> Vector {

    // Find the normals of the line segment we're reflecting off of
    let dx = object_p2.x - object_p1.x;
    let dy = object_p2.y - object_p2.y;
    let normal_1 = Vector{x: -1.0 * dy, y: dx}.norm();
    let normal_2 = Vector{x: dy, y: -1.0 * dx}.norm();

    let a = subject_dir.norm();
    let proj = normal_1.dot(&a);



    // TODO: Need a normal for the line segment
    // TODO: otherwise, how would we know what direction the berg is coming from
    return Vector{x:0.0, y:0.0};
}

impl World {
    pub fn new(size_x: u32, size_y: u32) -> World {
        // Populate the world with some randomly positioned ice bergs
        let ice = Vec::new();
        let boat = Boat::new(Vector{ x: (size_x / 2) as f32, y: (size_y / 2) as f32 }, BOAT_SIZE);
        World{size_x, size_y, ices: ice, boat: boat}
    }

    // TODO: Make these controls more rudder-like (boat rotates)
    pub fn key_w(&mut self) {
        let dir = Vector{x:0.0, y:-1.0};
        self.boat.direction = self.boat.direction.add(&dir.mul(BOAT_ACCELERATION));
    }
    pub fn key_a(&mut self) {
        let dir = Vector{x:-1.0, y:0.0};
        self.boat.direction = self.boat.direction.add(&dir.mul(BOAT_ACCELERATION));
    }
    pub fn key_s(&mut self) {
        let dir = Vector{x:0.0, y:1.0};
        // TODO: Constants for keyboard input magic numbers
        self.boat.direction = self.boat.direction.add(&dir.mul(BOAT_ACCELERATION));
    }
    pub fn key_d(&mut self) {
        let dir = Vector{x:1.0, y:0.0};
        self.boat.direction = self.boat.direction.add(&dir.mul(BOAT_ACCELERATION));
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
            let collisions = World::find_collisions(&self.ices, &berg);

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


    fn find_boat_collisions(&self, ices: &Vec<Ice>) -> Vec<Ice> {
        let mut collisions = Vec::new();
        let boat_pos = self.boat.get_position();
        for other_ice in ices.iter() {

            // TODO: What to do about this...
            if World::is_real_collision_with_boat(&self.boat, other_ice) {
                collisions.push(other_ice.clone());
            }

        }

        return collisions;
    }

    // same thing as the other one
    fn find_collisions<'a>(ices: &'a Vec<Ice>, ice: &Ice) -> Vec<&'a Ice> {
        let collisions = ices.iter()
            .filter(|other_ice| euc_distance(&other_ice.position, &ice.position) < (other_ice.size + ice.size) as f32)
            .filter(|other_ice| &other_ice.position != &ice.position)
            .collect();
        return collisions;
    }


    // TODO: WIP, make compiler happy, will be removed soon
    fn is_real_collision_with_boat(boat: &Boat, ice: &Ice) -> bool {
        return false;
    }

    fn get_boat_collision(boat: &Boat, ice: &Ice) -> Option<(Vector, Vector)> {
        for i in 0..ice.perimeter.len() - 1 {
            for k in 0..boat.perimeter.len() - 1 {
                let l1_p1 = ice.position.add(ice.perimeter.get(i).unwrap());
                let l1_p2 = ice.position.add(ice.perimeter.get(i + 1).unwrap());

                let l2_p1 = boat.position.add(boat.perimeter.get(k).unwrap());
                let l2_p2 = boat.position.add(boat.perimeter.get(k + 1).unwrap());

                if lines_intersect(l1_p1, l1_p2, l2_p1, l2_p2) {
                    return Some((l2_p1, l2_p2));
                }
            }
        }
        return None;
    }

    // Compares line segments making up bergs to see if they actually interact
    fn is_real_collision(ice_a: &Ice, ice_b: &Ice) -> bool {
        for i in 0..ice_a.perimeter.len() - 1 {
            for k in 0..ice_b.perimeter.len() - 1 {
                let l1_p1 = ice_a.position.add(ice_a.perimeter.get(i).unwrap());
                let l1_p2 = ice_a.position.add(ice_a.perimeter.get(i + 1).unwrap());

                let l2_p1 = ice_b.position.add(ice_b.perimeter.get(k).unwrap());
                let l2_p2 = ice_b.position.add(ice_b.perimeter.get(k + 1).unwrap());

                if lines_intersect(l1_p1, l1_p2, l2_p1, l2_p2) {
                    return true;
                }
            }
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

        // Boat collisions
        // TODO: Remove this old boat collision logic?
        /*
        let boat_collisions = self.find_boat_collisions(&self.ices);
        println!("Boat collisions: {:?}", boat_collisions.len());
        for collision in boat_collisions {
            // TODO: Hack because the boat-ice collision physics still needs work
            self.boat.direction = reflect(self.boat.position, self.boat.direction, collision.get_position(), collision.get_direction());
        }
        */

        // Each tick, compute the current grid position of each iceberg
        let mut grid = HashMap::new();
        for ice in self.ices.iter() {
            let (grid_x, grid_y) = ice.calc_grid();
            let mut col = grid.entry(grid_x).or_insert(HashMap::new());
            let mut row = col.entry(grid_y).or_insert(Vec::new());
            row.push(ice.clone());
        }


        // Update the boat position
        self.boat.position = self.boat.position.add(&self.boat.direction);

        let current_ices = self.ices.clone();
        let mut total_collisions = 0;
        let ices = self.ices.iter_mut();
        for ice in ices {

            let temp_dir = ice.direction.clone();
            let temp_pos = ice.position.clone();

            match World::get_boat_collision(&self.boat, &ice) {
                Some((p1, p2)) => {
                    println!("Reflecting berg off boat line");
                    // ice.direction = reflect_line(temp_pos, temp_dir, p1, p2);

                    // TODO: Temporary hack because I don't feel like working on the (correct) collision resolution logic
                    ice.direction = self.boat.direction.mul(1.5);
                },
                None => {

                }
            }
            /*
            if World::is_real_collision_with_boat(&self.boat, &ice) {

                // TODO: Boat collision resolution problem
                // TODO: This should reflect off the surface it collided with
                ice.direction = reflect(temp_pos, temp_dir, boat_pos_start_tick, boat_dir_start_tick);
            }
            */

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
            let collisions = World::find_collisions(&possible_collisions, &ice);
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
            berg.draw(canvas, &offset);
        }
        self.boat.draw(canvas, &offset);
    }
}