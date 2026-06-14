use crate::math::Vec2;
use rand_core::Rng

pub struct Boid {
    position: Vec2,
    velocity: Vec2
}

impl Boid {
    pub fn new(position: Vec2, velocity: Vec2) -> Self {
        Boid {position: position, velocity: velocity}
    }

    pub fn random_in<R: Rng>(rng: R, area_size: Vec2) -> Self {
        Self {
            position: Vec2::new(
                rng.random_range((-area_size.)..area_size), rng.random_range(area_size)
            )
        }
    }
}

pub fn init_boids() -> Vec<Boid> {
    // Generate a given amount of boids in random positions
}