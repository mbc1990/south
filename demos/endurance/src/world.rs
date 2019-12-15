use crate::ice::Ice;
use sdl2::render::{Canvas, WindowCanvas};
use sdl2::rect::Point;
use rand::Rng;


pub struct World {
    size_x: u32,
    size_y: u32,
    ices: Vec<Ice>
}

fn euc_distance(p1: Point, p2: Point) -> f32 {
    let dist = (((p1.x - p2.x).pow(2) + (p1.y - p2.y).pow(2)) as f32).sqrt();
    println!("dist: {:?}", dist);
    return dist;
}

impl World {
    pub fn new(size_x: u32, size_y: u32) -> World {
        // Populate the world with some randomly positioned ice bergs
        let ice = Vec::new();
        World{size_x, size_y, ices: ice}
    }

    pub fn init_with_random_ice(&mut self, mut num_bergs: i32) {
        let margin = 10;
        let mut rng = rand::thread_rng();
        while num_bergs > 0 {
            let berg_size = rng.gen_range(25, 200);
            let x = rng.gen_range(berg_size + margin, self.size_x - (berg_size + margin));
            let y = rng.gen_range(berg_size + margin, self.size_y - (berg_size + margin));
            println!("\n testing berg at {:?}, {:?} with size {:?}", x, y, berg_size);
            let berg = Ice::new(Point::new(x as i32, y as i32), berg_size);

            let collisions = self.find_collisions(&berg);
            println!("Collisions: {:?}", collisions);
            if collisions.len() == 0 {
                println!("Putting berg at {:?}, {:?} with size {:?}", x, y, berg_size);
                self.ices.push(berg);
                num_bergs -= 1;
            } else {
                println!("Rejecting berg at {:?}, {:?} with size {:?}", x, y, berg_size);
            }
        }
    }

    pub fn init_test(&mut self) {
        // let berg = Ice::new(Point::new(161, 358), 73);
        // self.ices.push(berg);
    }


    // Returns all icebergs that intersect with this one
    // Currently assumes all bergs are circles, which will need to be fixed
    fn find_collisions(&self, ice: &Ice) -> Vec<&Ice> {
        let collisions = self.ices.iter()
            .filter(|other_ice| euc_distance(other_ice.position, ice.position) < (other_ice.size + ice.size) as f32)
            .collect();
        return collisions;
    }

    // Called from event loop
    pub fn tick(self) {

    }

    pub fn draw(&self, canvas: &mut WindowCanvas) {
        for berg in &self.ices {
           berg.draw(canvas);
        }
    }
}