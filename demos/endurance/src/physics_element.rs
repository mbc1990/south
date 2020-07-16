use crate::vector::Vector;

pub struct PhysicsElement {
    pub direction: Vector,
    pub position: Vector,
    // Used in the second pass of the collision detection algorithm (do bounding circles intersect)
    // TODO: Should be able to compute this from the perimeter
    pub bounding_circle_radius: i32,
    pub perimeter: Vec<Vector>,
}