#[derive(Debug, Clone)]
pub struct Vector {
    pub x: f32,
    pub y: f32
}

impl Vector {
    pub fn add(&self, vector: &Vector) -> Vector {
        return Vector {x: self.x + vector.x, y: self.y + vector.y}
    }
    pub fn sub(&self, vector: &Vector) -> Vector {
        return Vector {x: self.x - vector.x, y: self.y - vector.y}
    }
}

