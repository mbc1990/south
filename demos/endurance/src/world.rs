use crate::ice::{Ice};
use crate::boat::{Boat};
use crate::vector::{Vector};
use sdl2::render::{WindowCanvas};
use rand::Rng;
use crate::{BOAT_SIZE, ICE_DECEL_FACTOR, BERG_MIN_SIZE, BERG_MAX_SIZE, GRID_SIZE, BOAT_ACCELERATION};
use crate::keyboard_state::KeyboardState;
use std::collections::HashMap;
use crate::geometry::{reflect, lines_intersect, euc_distance};
use std::time::Instant;
use crate::render_gl::Program;

pub struct World {
    size_x: u32,
    size_y: u32,
    ices: Vec<Ice>,
    boat: Boat
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

    // TODO: Rotation
    pub fn key_a(&mut self) {
        /*
        theta = deg2rad(angle);
        cs = cos(theta);
        sn = sin(theta);
        px = x * cs - y * sn;
        py = x * sn + y * cs;
        */
        let dir = Vector{x:-1.0, y:0.0};
        self.boat.direction = self.boat.direction.add(&dir.mul(BOAT_ACCELERATION));
    }
    pub fn key_s(&mut self) {
        let dir = Vector{x:0.0, y:1.0};
        // TODO: Constants for keyboard input magic numbers
        self.boat.direction = self.boat.direction.add(&dir.mul(BOAT_ACCELERATION));
    }
    // TODO: Rotation
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
                // println!("{:?} bergs remaining", num_bergs);
            }
        }
    }

    pub fn init_test(&mut self) {
        self.ices.push(Ice::new(Vector{x: 1200.0, y: 1200.0}, Vector{x:10.0, y: 0.0}.mul(0.0), 300));
        // self.ices.push(Ice::new(Vector{x: 1200.0, y: 200.0}, Vector{x:-10.0, y: 0.0}.mul(1.0), 100));
        // self.ices.push(Ice::new(Vector{x: 1200.0, y: 400.0}, Vector{x:-10.0, y: -5.0}.mul(1.0), 100));
    }

    fn find_collisions<'a>(ices: &'a Vec<Ice>, ice: &Ice) -> Vec<&'a Ice> {
        let collisions = ices.iter()
            .filter(|other_ice| euc_distance(&other_ice.position, &ice.position) < (other_ice.size + ice.size) as f32)
            .filter(|other_ice| &other_ice.position != &ice.position)
            .collect();
        return collisions;
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

    fn get_grid_region_bergs(grid: &HashMap<i32, HashMap<i32, Vec<Ice>>>, grid_x: i32, grid_y: i32) -> Vec<Ice> {
        let mut in_grid = Vec::new();
        if let Some(col) = grid.get(&grid_x) {
            if let Some(bergs) = col.get(&grid_y) {
                let to_append = bergs.clone();
                in_grid.append(&mut to_append.clone());
            }
        }
        return in_grid;
    }

    // Called from event loop
    pub fn tick(&mut self, keyboard_state: &KeyboardState) {

        self.respond_to_input(keyboard_state);

        // Each tick, compute the current grid position of each iceberg
        // TODO: Should be behind some kind of grid manager api
        let mut grid = HashMap::new();
        for ice in self.ices.iter() {
            let (grid_x, grid_y) = ice.calc_grid();
            let col = grid.entry(grid_x).or_insert(HashMap::new());
            let row = col.entry(grid_y).or_insert(Vec::new());
            row.push(ice.clone());
        }

        // Update the boat position
        self.boat.position = self.boat.position.add(&self.boat.direction);

        let ices = self.ices.iter_mut();
        for ice in ices {

            // Update ice position if it's colliding with the boat
            if let Some((_p1, _p2)) = World::get_boat_collision(&self.boat, &ice) {
                ice.direction = self.boat.direction.mul(1.5);
            }

            let (grid_x, grid_y) = ice.calc_grid();

            // Colocated bergs - hopefully only a few
            let others_in_grid = World::get_grid_region_bergs(&grid, grid_x, grid_y);
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
                let to_append = World::get_grid_region_bergs(&grid, grid_x - 1, grid_y);
                possible_collisions.append(&mut to_append.clone());
            }
            if x_2 {
                let to_append = World::get_grid_region_bergs(&grid, grid_x + 1, grid_y);
                possible_collisions.append(&mut to_append.clone());
            }
            if y_1 {
                let to_append = World::get_grid_region_bergs(&grid, grid_x, grid_y - 1);
                possible_collisions.append(&mut to_append.clone());
            }
            if y_2 {
                let to_append = World::get_grid_region_bergs(&grid, grid_x, grid_y + 1);
                possible_collisions.append(&mut to_append.clone());
            }

            // Upper left corner
            if x_1 && y_1 {
                let to_append = World::get_grid_region_bergs(&grid, grid_x - 1, grid_y - 1);
                possible_collisions.append(&mut to_append.clone());
            }

            // Lower left corner
            if x_1 && y_2 {
                let to_append = World::get_grid_region_bergs(&grid, grid_x - 1, grid_y + 1);
                possible_collisions.append(&mut to_append.clone());
            }

            // Upper right corner
            if x_2 && y_1 {
                let to_append = World::get_grid_region_bergs(&grid, grid_x + 1, grid_y - 1);
                possible_collisions.append(&mut to_append.clone());
            }

            // Lower right corner
            if x_2 && y_2 {
                let to_append = World::get_grid_region_bergs(&grid, grid_x + 1, grid_y + 1);
                possible_collisions.append(&mut to_append.clone());
            }

            // Collisions from circular bounding box
            let collisions = World::find_collisions(&possible_collisions, &ice);
            for collision in collisions {

                // Don't collide with yourself
                // TODO: Use uuid instead of position
                if collision.position.x == ice.position.x && collision.position.y == ice.position.y {
                    continue;
                }

                if World::is_real_collision(&ice, &collision) {
                    ice.direction = reflect(ice.position, ice.direction, collision.position, collision.direction);
                }
            }

            // Collisions reduce velocity overall
            ice.direction = ice.direction.mul(ICE_DECEL_FACTOR);
            ice.position = ice.position.add(&ice.direction);
        }
    }

    pub fn get_offset(&self) -> Vector {
        return self.boat.position.sub(&Vector{x: (self.size_x / 2) as f32, y: (self.size_y / 2) as f32 });
    }

    pub fn draw_gl(&self, program: &Program) {
        let offset = self.boat.position.sub(&Vector{x: (self.size_x / 2) as f32, y: (self.size_y / 2) as f32 });

        // This is a vector of f32s, each group of six serially representing a vertex (xyz) and color info (rgb)
        // Thus each group of eighteen (6*3) represents a triangle.
        let mut vertices: Vec<f32> = Vec::new();
        for berg in &self.ices {
            let mut berg_verts = berg.get_vertices(&offset);
            vertices.append(&mut berg_verts);
        }

        let num_indices= vertices.len() as i32 / 6;

        let mut vbo: gl::types::GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
        }

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,                                                       // target
                (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
                vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
                gl::STATIC_DRAW,                               // usage
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }

        // set up vertex array object
        let mut vao: gl::types::GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }

        unsafe {
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
            gl::VertexAttribPointer(
                0,         // index of the generic vertex attribute ("layout (location = 0)")
                3,         // the number of components per generic vertex attribute
                gl::FLOAT, // data type
                gl::FALSE, // normalized (int-to-float conversion)
                (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
                std::ptr::null(),                                     // offset of the first component
            );
            gl::EnableVertexAttribArray(1); // this is "layout (location = 0)" in vertex shader
            gl::VertexAttribPointer(
                1,         // index of the generic vertex attribute ("layout (location = 0)")
                3,         // the number of components per generic vertex attribute
                gl::FLOAT, // data type
                gl::FALSE, // normalized (int-to-float conversion)
                (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
                (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid, // offset of the first component
            );

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // draw triangle
        program.set_used();
        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawArrays(
                gl::TRIANGLES, // mode
                0,             // starting index in the enabled arrays
                num_indices,             // number of indices to be rendered
            );
        }

        // TODO: Draw boat with opengl
        // self.boat.draw(canvas, &offset);

    }
}