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
use crate::{GRID_SIZE, BERG_MAX_SIZE, ICE_DECEL_FACTOR};
use crate::geometry::{euc_distance, lines_intersect, reflect};
use crate::vector::Vector;


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

    // TODO: Take reference
    pub fn get_element(&self, physics_id: String) -> Option<&PhysicsElement> {
       self.elements.get(&physics_id)
    }

    // TODO: Take reference
    pub fn get_element_mut(&mut self, physics_id: String) -> Option<&mut PhysicsElement> {
        self.elements.get_mut(&physics_id)
    }


    fn get_grid_region_elems(grid: &HashMap<i32, HashMap<i32, Vec<PhysicsElement>>>, grid_x: i32, grid_y: i32) -> Vec<&PhysicsElement> {
        let mut in_grid = Vec::new();
        if let Some(col) = grid.get(&grid_x) {
            if let Some(els) = col.get(&grid_y) {
                for el in els {
                    in_grid.push(el);
                }
            }
        }
        return in_grid;
    }

    // Compares line segments making up two PhysicsElements to see if they actually intersect
    fn is_real_collision(el_a: &PhysicsElement, el_b: &PhysicsElement) -> bool {
        for i in 0..el_a.perimeter.len() - 1 {
            for k in 0..el_b.perimeter.len() - 1 {
                let l1_p1 = el_a.position.add(el_a.perimeter.get(i).unwrap());
                let l1_p2 = el_a.position.add(el_a.perimeter.get(i + 1).unwrap());

                let l2_p1 = el_b.position.add(el_b.perimeter.get(k).unwrap());
                let l2_p2 = el_b.position.add(el_b.perimeter.get(k + 1).unwrap());

                if lines_intersect(l1_p1, l1_p2, l2_p1, l2_p2) {
                    return true;
                }
            }
        }
        return false;
    }

    pub fn tick(&mut self) {

        // Build grid
        let mut grid = HashMap::new();
        for (_, el) in self.elements.iter() {
            let (grid_x, grid_y) = el.calc_grid_coords();
            let col = grid.entry(grid_x).or_insert(HashMap::new());
            let row = col.entry(grid_y).or_insert(Vec::new());
            row.push((*el).clone());
        }

        // Initialize new elements map
        let mut new_elements = HashMap::new();

        for (uuid, el) in self.elements.iter() {

            let mut new_el = el.clone();

            let (grid_x, grid_y) = el.calc_grid_coords();

            // Colocated elements - hopefully only a few
            let mut possible_collisions = PhysicsManager::get_grid_region_elems(&grid, grid_x, grid_y);

            // TODO: Iceberg specific assumptions (max size)
            // Grid regions are squares, so the berg can be colliding with objects in up to three
            // more grid regions adjacent to the one the center of the berg is in.
            // THIS IS TRUE ONLY WHEN THE GRID SIZE IS GREATER THAN 2 * MAX_BERG_SIZE
            // The bounds here are a little tricky. Our ice might be completely within its grid, but collide with
            // another berg at the edge of an adjacent grid
            let x_1 = (el.position.x - el.bounding_circle_radius as f32) < ((grid_x * GRID_SIZE as i32) + BERG_MAX_SIZE as i32) as f32;
            let x_2 = (el.position.x + el.bounding_circle_radius as f32) > (((grid_x + 1) * GRID_SIZE as i32) - BERG_MAX_SIZE as i32) as f32;
            let y_1 = (el.position.y - el.bounding_circle_radius as f32) < ((grid_y * GRID_SIZE as i32) + BERG_MAX_SIZE as i32) as f32;
            let y_2 = (el.position.y + el.bounding_circle_radius as f32) > (((grid_y + 1) * GRID_SIZE as i32) - BERG_MAX_SIZE as i32) as f32;

            if x_1 {
                let to_append = PhysicsManager::get_grid_region_elems(&grid, grid_x - 1, grid_y);
                possible_collisions.append(&mut to_append.clone());
            }
            if x_2 {
                let to_append = PhysicsManager::get_grid_region_elems(&grid, grid_x + 1, grid_y);
                possible_collisions.append(&mut to_append.clone());
            }
            if y_1 {
                let to_append = PhysicsManager::get_grid_region_elems(&grid, grid_x, grid_y - 1);
                possible_collisions.append(&mut to_append.clone());
            }
            if y_2 {
                let to_append = PhysicsManager::get_grid_region_elems(&grid, grid_x, grid_y + 1);
                possible_collisions.append(&mut to_append.clone());
            }

            // Upper left corner
            if x_1 && y_1 {
                let to_append = PhysicsManager::get_grid_region_elems(&grid, grid_x - 1, grid_y - 1);
                possible_collisions.append(&mut to_append.clone());
            }

            // Lower left corner
            if x_1 && y_2 {
                let to_append = PhysicsManager::get_grid_region_elems(&grid, grid_x - 1, grid_y + 1);
                possible_collisions.append(&mut to_append.clone());
            }

            // Upper right corner
            if x_2 && y_1 {
                let to_append = PhysicsManager::get_grid_region_elems(&grid, grid_x + 1, grid_y - 1);
                possible_collisions.append(&mut to_append.clone());
            }

            // Lower right corner
            if x_2 && y_2 {
                let to_append = PhysicsManager::get_grid_region_elems(&grid, grid_x + 1, grid_y + 1);
                possible_collisions.append(&mut to_append.clone());
            }

            // Collisions from circular bounding box
            let cbb_collisions: Vec<&PhysicsElement> = possible_collisions.iter()
                .filter(|other_el| euc_distance(&other_el.position, &el.position) < (other_el.bounding_circle_radius + el.bounding_circle_radius) as f32)
                .filter(|other_el| & other_el.position != & el.position)  // TODO: Should check UUIDs (those aren't on PhysicsElements currently)
                .map(|other_el| *other_el)
                .collect();

            for collision in cbb_collisions {

                // Don't collide with yourself
                // TODO: Use uuid instead of position
                if collision.position.x == el.position.x && collision.position.y == el.position.y {
                    continue;
                }

                if PhysicsManager::is_real_collision(&el, &collision) {
                    new_el.direction = reflect(el.position, el.direction, collision.position, collision.direction);
                }
            }

            // Collisions reduce velocity overall
            // TODO: Ice-specific behavior
            new_el.direction = new_el.direction.mul(ICE_DECEL_FACTOR);
            new_el.position = new_el.position.add(&new_el.direction);
            new_elements.insert(uuid.to_string(), new_el);
        }

        // Finally, replace last tick's physics state with the updated one
        self.elements = new_elements;
    }
}
