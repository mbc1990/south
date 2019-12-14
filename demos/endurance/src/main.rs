extern crate rand;

use sdl2::rect::{Point, Rect};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use sdl2::keyboard::Keycode;
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, Texture, TextureCreator, WindowCanvas};
use rand::Rng;
use std::f64;


pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 500;


fn draw_ice(canvas: &mut WindowCanvas, pos_x: i32, pos_y: i32, size: i32) {
    canvas.set_draw_color(Color::RGB(228, 240, 253));

    let mut angles = Vec::new();
    for i in 0..13 {
        angles.push(i * 30);
    }
    let mut points = Vec::new();

    let point_x = pos_x;
    let point_y = pos_y + size;

    for angle in angles {
        let angle_rad = angle as f64 * f64::consts::PI / 180 as f64;
        let r_x = angle_rad.cos() * (point_x as f64 - pos_x as f64) - angle_rad.sin() * (point_y as f64- pos_y as f64) + pos_x as f64;
        let r_y = angle_rad.sin() * (point_x as f64 - pos_x as f64) - angle_rad.cos() * (point_y as f64- pos_y as f64) + pos_x as f64;
        points.push(Point::new(r_x as i32, r_y as i32));
    }

    println!("Points: {:?}", points);

    for i in 0..points.len() - 1 {
        let p1 = points.get(i).unwrap();
        let p2 = points.get(i+1).unwrap();
        canvas.draw_line(Point::new(p1.x, p1.y), Point::new(p2.x, p2.y));
    }
}

fn main() -> Result<(), String> {
    println!("Hello, world!");

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

    draw_ice(&mut canvas, 200, 200, 50);

    // However the canvas has not been updated to the window yet, everything has been processed to
    // an internal buffer, but if we want our buffer to be displayed on the window, we need to call
    // `present`. We need to call this everytime we want to render a new frame on the window.
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;

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
        draw_ice(&mut canvas, 200, 200, 50);
        // TODO: Update game state here
        canvas.present();
        frame += 1;
    }


    Ok(())
}
