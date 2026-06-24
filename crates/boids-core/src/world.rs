use rand_core::Rng;

use crate::boid::{self, Boid};
use crate::math::{self, Vec2};
use crate::params::{SimulationParams, WorldParams};

pub struct World {
    boids: Vec<Boid>,
    sim_params: SimulationParams,
    world_params: WorldParams,
}

impl World {
    fn new<R: Rng>(rng:&mut R, sim_params: SimulationParams, world_params: WorldParams) -> Self {
        let boids = boid::init_boids(rng, world_params.area_size, world_params.boid_count);

        Self {boids: boids, sim_params: sim_params, world_params: world_params}
    }

    fn init_boids<R: Rng>(rng: &mut R, area_size: Vec2, boid_count: usize) -> Vec<Boid> {
    // Generate a given amount of boids in random positions
    let mut boids = Vec::<Boid>::new();

    let angle = math::random_range_f32(rng, 0.0, core::f32::consts::TAU);
    let speed = math::random_range_f32(rng, min, max); // How to load max speed in

    let start_velocity = Vec2::new(angle.cos(), angle.sin()) * speed;
    // We dont actually want to use sin and cos if we are dealing with embedded. there may be
    // implementations on esp32 hal but maybe we should try with a small direction lookup instead
    // As for our purposes we dont really need the full set of random directions anyway
    // Eventually the boids diverge and converge

    for _ in 0..boid_count {
        boids.push(Boid::random_in(rng, area_size)); 
    }
    boids
}

    fn new_default_params<R: Rng>(rng:&mut R) -> Self {
        Self::new(rng, SimulationParams::default(), WorldParams::default())
    }

    fn tick(&mut self, dt: i32) {
        /* for each boid:
        1. calculate steering force
        2. apply force
        3. update position and velocity 
        4. handle world boundaries*/

    }

    fn get_boids(&self) -> &[Boid] {
        &self.boids
    }
}