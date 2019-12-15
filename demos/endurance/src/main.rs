extern crate rand;

use sdl2::rect::{Point, Rect};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use sdl2::keyboard::Keycode;
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, Texture, TextureCreator, WindowCanvas};
use rand::Rng;
use world::World;
use std::f64;

mod world;
mod ice;
mod vector;

pub const WIDTH: u32 = 800*2;
pub const HEIGHT: u32 = 800*2;


fn main() -> Result<(), String> {
    println!("Welcome to the Endurance demo");

    let sdl_context = sdl2::init()?;
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
    let mut canvas = window.into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    println!("Using SDL_Renderer \"{}\"", canvas.info().name);
    canvas.set_draw_color(Color::RGB(6, 100, 193));
    // clears the canvas with the color we set in `set_draw_color`.
    canvas.clear();

    // draw_ice(&mut canvas, 200, 200, 50);

    // However the canvas has not been updated to the window yet, everything has been processed to
    // an internal buffer, but if we want our buffer to be displayed on the window, we need to call
    // `present`. We need to call this everytime we want to render a new frame on the window.
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;

    let mut world = World::new(WIDTH, HEIGHT);
    world.init_with_random_ice(100);
    // world.init_test();
    world.draw(&mut canvas);
    canvas.present();


    let mut frame : u32 = 0;
    'running: loop {
        // get the inputs here
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Space), repeat: false, .. } => {
                },
                Event::MouseButtonDown { x, y, mouse_btn: MouseButton::Left, .. } => {
                },
                _ => {}
            }
        }

        // update the game loop here
        if frame >= 30 {
            frame = 0;
        }
        canvas.set_draw_color(Color::RGB(6, 100, 193));
        canvas.clear();
        world.tick();
        world.draw(&mut canvas);
        canvas.present();
        frame += 1;
    }


    Ok(())
}
