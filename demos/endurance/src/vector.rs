#[derive(Debug, Clone, Copy)]
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
    pub fn norm(&self) -> Vector {
        let magnitude = (self.x.powf(2.0) + self.y.powf(2.0)).sqrt();
        return Vector{x: self.x  / magnitude, y: self.y / magnitude};
    }
    pub fn dot(&self, vector: &Vector) -> f32 {
        return self.x * vector.x + self.y * vector.y;
    }
    pub fn mul(&self, scalar: f32) -> Vector {
        return Vector{x: self.x * scalar, y: self.y * scalar};
    }
}

