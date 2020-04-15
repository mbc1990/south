extern crate rand;

use sdl2::pixels::Color;
use sdl2::render::{WindowCanvas};
use world::World;
use std::{thread, time};
use std::time::{Instant};
use crate::input_manager::InputManager;
use crate::hud::Hud;
use sdl2::Sdl;
use std::ffi::CString;

mod world;
mod ice;
mod boat;
mod geometry;
mod vector;
mod keyboard_state;
mod input_manager;
mod hud;
pub mod render_gl;


pub const WIDTH: u32 = 800*2;
pub const HEIGHT: u32 = 800*2;
pub const FPS: u32 = 30;
pub const BOAT_SIZE: u32 = 25;
pub const NUM_BERGS: i32 = 1;
pub const BERG_MIN_SIZE: u32 = 10;
pub const BERG_MAX_SIZE: u32 = 75;
pub const ICE_DECEL_FACTOR: f32 = 0.99;
pub const BOAT_ACCELERATION: f32 = 0.2;
pub const HUD_FONT_PATH: &str = "/home/malcolm/Downloads/RobotoCondensed-Bold.ttf";
pub const DEBUG_MODE: bool = false;
pub const GRID_SIZE: u32 = 2 * BERG_MAX_SIZE + 10;

fn main() -> Result<(), String> {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);

    let window = video_subsystem
        .window("Game", WIDTH, HEIGHT)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let _gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    let vert_shader =
        render_gl::Shader::from_vert_source(&CString::new(include_str!("triangle.vert")).unwrap())
            .unwrap();

    let frag_shader =
        render_gl::Shader::from_frag_source(&CString::new(include_str!("triangle.frag")).unwrap())
            .unwrap();

    let shader_program = render_gl::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

    let event_pump = sdl.event_pump()?;
    let mut input_manager = InputManager::new(event_pump);
    // TODO: Migrate hud to opengl
    // let hud = Hud::new();
    let mut world = World::new(WIDTH, HEIGHT);

    // world.init_test();

    // TODO: Can render 32x4, but 64x4 (18,432 bytes) segfaults
    world.init_with_random_ice(2048);
    println!("Done init?");
    unsafe {
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
    world.draw_gl(&shader_program);
    println!("Buffers all send over");
    window.gl_swap_window();

    // world.draw(&mut canvas);

    // canvas.present();

    let frame_length = 1000.0 / FPS as f32;
    'running: loop {
        let frame_start = Instant::now();
        unsafe {
            gl::ClearColor(0.6, 0.0, 0.8, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        let keyboard_state = input_manager.get_keyboard_state();
        if keyboard_state.esc {
            break 'running;
        }

        world.tick(&keyboard_state);
        // world.draw_gl(&mut canvas);
        world.draw_gl(&shader_program);
        window.gl_swap_window();

        let elapsed = frame_start.elapsed();
        if elapsed.as_millis() < frame_length as u128 {
            thread::sleep(time::Duration::from_millis((frame_length - elapsed.as_millis() as f32) as u64));
        }
        // canvas.present();
    }

    Ok(())
}


fn oldmain() -> Result<(), String> {

    println!("Welcome to the Endurance demo");
    let sdl_context = sdl2::init()?;
    let mut canvas = construct_canvas(&sdl_context)?;
    let event_pump = sdl_context.event_pump()?;
    let mut input_manager = InputManager::new(event_pump);
    let hud = Hud::new();
    let mut world = World::new(WIDTH, HEIGHT);

    // canvas.set_draw_color(Color::RGB(6, 100, 193));
    // canvas.clear();
    world.init_with_random_ice(NUM_BERGS);
    // world.init_test();
    world.draw(&mut canvas);
    canvas.present();

    let frame_length = 1000.0 / FPS as f32;
    'running: loop {
        let frame_start = Instant::now();

        let keyboard_state = input_manager.get_keyboard_state();
        if keyboard_state.esc {
            break 'running;
        }

        // canvas.set_draw_color(Color::RGB(6, 100, 193));
        // canvas.clear();
        world.tick(&keyboard_state);
        world.draw(&mut canvas);

        let elapsed = frame_start.elapsed();
        if elapsed.as_millis() < frame_length as u128 {
            thread::sleep(time::Duration::from_millis((frame_length - elapsed.as_millis() as f32) as u64));
        }

        if DEBUG_MODE {
            hud.draw_fps(&mut canvas, 1000.0 / frame_start.elapsed().as_millis() as f32);
            hud.draw_collision_grid(&mut canvas, world.get_offset());
        }
        canvas.present();
    }
    Ok(())
}

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

fn construct_canvas(sdl_context: &Sdl) -> Result<WindowCanvas, String> {
    let video_subsystem = sdl_context.video()?;

    // the window is the representation of a window in your operating system,
    // however you can only manipulate properties of that window, like its size, whether it's
    // fullscreen, ... but you cannot change its content without using a Canvas or using the
    // `surface()` method.
    let window = video_subsystem
        .window("Endurance",
                WIDTH,
                HEIGHT)
        .opengl()
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    // the canvas allows us to both manipulate the property of the window and to change its content
    // via hardware or software rendering. See CanvasBuilder for more info.
    let canvas = window.into_canvas()
        .index(find_sdl_gl_driver().unwrap())
        .build()
        .map_err(|e| e.to_string())?;

    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
    canvas.window().gl_set_context_to_current();

    return Ok(canvas);
}
