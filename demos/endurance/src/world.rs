use crate::ice::Ice;
use sdl2::render::{Canvas, WindowCanvas};
use sdl2::rect::Point;
use rand::Rng;


pub struct World {
    size_x: u32,
    size_y: u32,
    ice: Vec<Ice>
}

impl World {
    pub fn new(size_x: u32, size_y: u32) -> World {

        // Populate the world with some randomly positioned ice bergs
        let num_bergs = 5;
        let margin = 10;
        let mut ice = Vec::new();
        let mut rng = rand::thread_rng();
        for i in 0..num_bergs {
            let berg_size = rng.gen_range(25, 100);
            let x = rng.gen_range(berg_size + margin, size_x - (berg_size + margin));
            let y = rng.gen_range(berg_size + margin, size_y - (berg_size + margin));
            println!("Putting berg at {:?}, {:?}", x, y);
            let berg = Ice::new(Point::new(x as i32, y as i32), berg_size);
            ice.push(berg);
        }
        World{size_x, size_y, ice}
    }

    // Called from event loop
    pub fn tick(self) {

    }

    pub fn draw(&self, canvas: &mut WindowCanvas) {
        for berg in &self.ice {
           berg.draw(canvas);
        }
    }
}