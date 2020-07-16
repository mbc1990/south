/*
The idea is that this controls the core physics interactions that are universal to game state,
such as collisions that need to be computed every tick. This also acts as the source of truth
for physics aspects of game objects. A "Boat" might have its own game-related data (such as a
variable representing fuel), but it also has an associated PhysicsElement that is owned by the
singleton instance of this struct.

All positional changes are passed through the PhysicsManager API. For now, this involves doing
a hashmap lookup, which provides a nice logical boundary but may be a performance issue.

*/
use std::collections::HashMap;
use crate::physics_element::PhysicsElement;
use uuid::Uuid;


pub struct PhysicsManager {
    elements: HashMap<String, PhysicsElement>,
}

impl PhysicsManager {
    pub fn new() -> PhysicsManager {
        let elements: HashMap<String, PhysicsElement> = HashMap::new();
        PhysicsManager{elements}
    }

    pub fn register_element(&mut self, element: PhysicsElement) -> String {
        let uuid = Uuid::new_v4().to_string();
        self.elements.insert(uuid.clone(), element);
        return uuid;
    }

    pub fn get_element(&self, physics_id: String) -> Option<&PhysicsElement> {
       self.elements.get(&physics_id)
    }

    pub fn get_element_mut(&mut self, physics_id: String) -> Option<&mut PhysicsElement> {
        self.elements.get_mut(&physics_id)
    }

    // TODO: Collisions, etc.
    pub fn tick(&mut self) {

    }
}
