use crate::math::{self, Vec2};

#[derive(Clone)]
pub struct Boid {
    pub position: Vec2,
    pub velocity: Vec2
}

impl Boid {
    pub fn new(position: Vec2, velocity: Vec2) -> Self {
        Boid {position: position, velocity: velocity}
    }
}
