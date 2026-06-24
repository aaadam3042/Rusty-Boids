use rand_core::Rng;

use crate::math::{self, Vec2};

pub struct Boid {
    position: Vec2,
    velocity: Vec2
}

impl Boid {
    fn new(position: Vec2, velocity: Vec2) -> Self {
        Boid {position: position, velocity: velocity}
    }

    pub fn random_in<R: Rng>(rng: &mut R, area_size: Vec2) -> Self {
        let x = math::random_range_f32(rng, 0.0, area_size.x);
        let y = math::random_range_f32(rng, 0.0, area_size.y);
        Self::new(
            Vec2::new(x, y),
            Vec2::ZERO
        )
    }
}
