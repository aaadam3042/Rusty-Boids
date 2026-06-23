use rand_core::Rng;

use crate::boid::{self, Boid};
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

    fn new_default_params<R: Rng>(rng:&mut R) -> Self {
        Self::new(rng, SimulationParams::default(), WorldParams::default())
    }

    fn tick(&mut self, dt: i32) {

    }

    fn get_boids(&self) -> &[Boid] {
        &self.boids
    }
}