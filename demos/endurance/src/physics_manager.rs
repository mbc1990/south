/*
TODO: Description
*/
use std::collections::HashMap;
use crate::physics_element::PhysicsElement;

pub struct PhysicsManager {
    elements: HashMap<i32, PhysicsElement>
}

impl PhysicsManager {
    pub fn new() -> PhysicsManager {
        let elements: HashMap<i32, PhysicsElement> = HashMap::new();
        PhysicsManager{elements}
    }
}
