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
use crate::physics_manager::PhysicsManager;
use crate::physics_element::PhysicsElement;

pub struct World {
    size_x: u32,
    size_y: u32,
    ices: Vec<Ice>,
    boat: Boat,
    physics_manager: PhysicsManager
    /*
        GraphicsManager
            - init -> Set up shaders
            - set_camera_position
            - set_camera_rotation
            - draw_objects(Vec<&dyn TriangleBasedDrawable>)
                   object.get_vertices()

    */
}

impl World {
    pub fn new(size_x: u32, size_y: u32) -> World {
        // Populate the world with some randomly positioned ice bergs
        let ice = Vec::new();
        let boat = Boat::new(Vector{ x: (size_x / 2) as f32, y: (size_y / 2) as f32 }, BOAT_SIZE);
        let pm = PhysicsManager::new();
        World{size_x, size_y, ices: ice, boat: boat, physics_manager: pm}
    }

    // TODO: Make these controls more rudder-like (boat rotates)
    pub fn key_w(&mut self) {
        let dir = Vector{x:0.0, y:-1.0};
        self.boat.direction = self.boat.direction.add(&dir.mul(BOAT_ACCELERATION));
    }

    // TODO: Rotation
    pub fn key_a(&mut self) {
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
            let mut berg = Ice::new(Vector{x:x as f32, y:y as f32}, Vector{x:dir_x, y:dir_y}.mul(vel), berg_size);

            // let berg = Ice::new(Vector{x:x as f32, y:y as f32}, Vector{x:0.0, y:0.0}, berg_size);
            let collisions = World::find_collisions_init(&self.ices, &berg);

            if euc_distance(&self.boat.position, &berg.position) < (self.boat.size * 3 + *&berg.size) as f32 {
               continue;
            }

            if collisions.len() == 0 {
                let pe = berg.build_physics_element();
                let physics_id = self.physics_manager.register_element(pe);
                berg.physics_id = Some(physics_id);
                self.ices.push(berg);
                num_bergs -= 1;
                println!("{:?} bergs remaining", num_bergs);
            }
        }
    }

    pub fn init_test(&mut self) {
        // TODO: This is a little awkward...
        let mut ice = Ice::new(Vector{x: 1200.0, y: 1200.0}, Vector{x:-2.0, y: 0.0}.mul(1.0), 300);
        let pe = ice.build_physics_element();
        let physics_id = self.physics_manager.register_element(pe);
        ice.physics_id = Some(physics_id);
        self.ices.push(ice);

        let mut ice2 = Ice::new(Vector{x: 400.0, y: 1200.0}, Vector{x:2.0, y: 0.0}.mul(1.0), 300);
        let pe2 = ice2.build_physics_element();
        let physics_id2 = self.physics_manager.register_element(pe2);
        ice2.physics_id = Some(physics_id2);
        self.ices.push(ice2);
    }

    // TODO: Refactor this - quick fix for random berg init
    fn find_collisions_init<'a>(ices: &'a Vec<Ice>, ice: &Ice) -> Vec<&'a Ice> {
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
        self.physics_manager.tick();
        // TODO: Game logic
    }

    pub fn get_offset(&self) -> Vector {
        return self.boat.position.sub(&Vector{x: (self.size_x / 2) as f32, y: (self.size_y / 2) as f32 });
    }

    pub fn draw(&self) {

    }

    pub fn draw_gl(&self, program: &Program) {
        let offset = self.boat.position.sub(&Vector{x: (self.size_x / 2) as f32, y: (self.size_y / 2) as f32 });

        // This is a vector of f32s, each group of six serially representing a vertex (xyz) and color info (rgb)
        // Thus each group of eighteen (6*3) represents a triangle.
        let mut vertices: Vec<f32> = Vec::new();
        for berg in &self.ices {
            let mut berg_verts = berg.get_vertices(&offset, &self.physics_manager);
            vertices.append(&mut berg_verts);
        }

        // TODO: This DISABLES drawing the boat
        // let mut boat_verts = self.boat.get_vertices(&offset);
        // vertices.append(&mut boat_verts);

        let num_indices= vertices.len() as i32 / 6;

        let mut vbo: gl::types::GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
        }

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                vertices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
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
        /*
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        */

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
    }
}