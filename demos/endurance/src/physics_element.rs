use crate::vector::Vector;
use crate::GRID_SIZE;

#[derive(Debug, Clone)]
pub struct PhysicsElement {
    pub direction: Vector,
    pub position: Vector,
    // Used in the second pass of the collision detection algorithm (do bounding circles intersect)
    // TODO: Should be able to compute this from the perimeter
    pub bounding_circle_radius: i32,
    pub perimeter: Vec<Vector>,
}

impl PhysicsElement {

    pub fn calc_grid_coords(&self) -> (i32, i32) {
        let mut grid_x = (self.position.x / GRID_SIZE as f32) as i32;
        let mut grid_y = (self.position.y / GRID_SIZE as f32) as i32;

        // Offset grid regions of negative positions due to integer division giving region 0
        // when a number x i -1.0 < x < 1.0
        // There's probably a more elegant way of doing this
        if self.position.x < 0.0 {
            grid_x -= 1;
        }
        if self.position.y < 0.0 {
            grid_y -= 1;
        }
        return (grid_x, grid_y);
    }
}