extern crate rand;

use sdl2::pixels::Color;
use sdl2::render::{WindowCanvas};
use world::World;
use std::{thread, time};
use std::time::{Instant};
use crate::input_manager::InputManager;
use crate::hud::Hud;
use sdl2::Sdl;

mod world;
mod physics_element;
mod ice;
mod boat;
mod vector;
mod keyboard_state;
mod input_manager;
mod hud;

pub const WIDTH: u32 = 800*2;
pub const HEIGHT: u32 = 800*2;
pub const FPS: u32 = 30;
pub const BOAT_SIZE: u32 = 50;
pub const NUM_BERGS: i32 = 3500;
pub const BERG_MIN_SIZE: u32 = 5;
pub const BERG_MAX_SIZE: u32 = 200;
pub const ICE_COLLISION_DECEL_FACTOR: f32 = 0.95;
pub const HUD_FONT_PATH: &str = "/home/malcolm/Downloads/RobotoCondensed-Bold.ttf";

// TODO: This can be optimized but affects gameplay behavior
// pub const GRID_SIZE: u32 = BERG_MAX_SIZE + 5;
pub const GRID_SIZE: u32 = 10;

fn main() -> Result<(), String> {
    println!("Welcome to the Endurance demo");
    let sdl_context = sdl2::init()?;

    let mut canvas = construct_canvas(&sdl_context)?;
    let event_pump = sdl_context.event_pump()?;
    let mut input_manager = InputManager::new(event_pump);
    let hud = Hud::new();
    let mut world = World::new(WIDTH, HEIGHT);

    canvas.set_draw_color(Color::RGB(6, 100, 193));
    // clears the canvas with the currently set color
    canvas.clear();
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

        canvas.set_draw_color(Color::RGB(6, 100, 193));
        canvas.clear();
        world.tick(&keyboard_state);
        world.draw(&mut canvas);

        let elapsed = frame_start.elapsed();
        if elapsed.as_millis() < frame_length as u128 {
            thread::sleep(time::Duration::from_millis((frame_length - elapsed.as_millis() as f32) as u64));
        }

        hud.draw_fps(&mut canvas, 1000.0 / frame_start.elapsed().as_millis() as f32);
        canvas.present();
    }

    Ok(())
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
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    // the canvas allows us to both manipulate the property of the window and to change its content
    // via hardware or software rendering. See CanvasBuilder for more info.
    let canvas = window.into_canvas()
        .target_texture()
        .present_vsync()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;

    return Ok(canvas);
}
