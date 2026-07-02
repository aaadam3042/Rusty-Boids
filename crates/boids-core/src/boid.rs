use crate::math::{self, Vec2};

pub struct Boid {
    position: Vec2,
    velocity: Vec2
}

impl Boid {
    pub fn new(position: Vec2, velocity: Vec2) -> Self {
        Boid {position: position, velocity: velocity}
    }
}
