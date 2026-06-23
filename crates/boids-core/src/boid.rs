use crate::math::Vec2;
use rand_core::Rng;

pub struct Boid {
    position: Vec2,
    velocity: Vec2
}

impl Boid {
    fn new(position: Vec2, velocity: Vec2) -> Self {
        Boid {position: position, velocity: velocity}
    }

    fn random_in<R: Rng>(rng: &mut R, area_size: Vec2) -> Self {
        let x = (rng.next_u32() as f32 / u32::MAX as f32) * area_size.x;
        let y = (rng.next_u32() as f32 / u32::MAX as f32) * area_size.y;
        Self::new(
            Vec2::new(x, y),
            Vec2::ZERO
        )
    }
}

pub fn init_boids<R: Rng>(rng: &mut R, area_size: Vec2, boid_count: usize) -> Vec<Boid> {
    // Generate a given amount of boids in random positions
    // NOTE: Consider moving out to world especially if we need more params
    // NOTE: Should we consider loading RNG in some other way
    let mut boids = Vec::<Boid>::new();
    for _ in 0..boid_count {
        boids.push(Boid::random_in(rng, area_size)); 
    }
    boids
}