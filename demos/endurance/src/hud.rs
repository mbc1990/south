use sdl2::pixels::Color;
use sdl2::render::{WindowCanvas, TextureQuery};
use sdl2::rect::{Rect, Point};
use crate::{HUD_FONT_PATH, WIDTH, GRID_SIZE, HEIGHT};
use crate::vector::Vector;

pub struct Hud {
}

impl Hud {
    pub fn new() -> Hud {
        return Hud{};
    }

    // Draw the grids used in collision detection
    pub fn draw_collision_grid(&self, canvas: &mut WindowCanvas, offset: Vector) {
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        for x in -100i32..100i32 {
            let p1 = Point::new((GRID_SIZE as i32 * x) - offset.x as i32, 0);
            let p2 = Point::new((GRID_SIZE as i32 * x) - offset.x as i32, HEIGHT as i32);
            let _ = canvas.draw_line(p1, p2);
        }
        for y in -100i32..100i32 {
            let p1 = Point::new(0, (GRID_SIZE as i32 * y) - offset.y as i32);
            let p2 = Point::new(WIDTH as i32, (GRID_SIZE as i32 * y) - offset.y as i32);
            let _ = canvas.draw_line(p1, p2);
        }
    }

    pub fn draw_fps(&self, canvas: &mut WindowCanvas, fps: f32) {
        let to_draw = format!("FPS: {}", fps as u32);
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
        let mut font = ttf_context.load_font(HUD_FONT_PATH, 60).unwrap();
        font.set_style(sdl2::ttf::FontStyle::BOLD);

        let surface = font.render(&to_draw)
            .blended(Color::RGBA(0, 0, 0, 255)).map_err(|e| e.to_string()).unwrap();

        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.create_texture_from_surface(&surface)
            .map_err(|e| e.to_string()).unwrap();

        let TextureQuery { width, height, .. } = texture.query();
        let target = Rect::new(15, 0, width, height);
        canvas.copy(&texture, None, Some(target)).unwrap();
    }
}