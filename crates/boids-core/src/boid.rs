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

pub fn init_boids() -> Vec<Boid> {
    // Generate a given amount of boids in random positions
    // We need the Rng and area size as well, as well as amount of boids. This feels like it should take 
    // a struct of params. Maybe we load rng in using a separate funciton, but that shouldnt exist on a boid
}