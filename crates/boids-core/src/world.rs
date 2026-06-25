use rand_core::Rng;

use crate::boid::Boid;
use crate::math::{self, Vec2};
use crate::params::{SimulationParams, WorldParams};

pub struct World {
    boids: Vec<Boid>,
    sim_params: SimulationParams,
    world_params: WorldParams,
}

impl World {
    fn new<R: Rng>(rng:&mut R, sim_params: SimulationParams, world_params: WorldParams) -> Self {
        let mut world = Self {boids: Vec::new(), sim_params: sim_params, world_params: world_params};
        world.init_boids(rng);

        world
    }
    
    fn new_default_params<R: Rng>(rng:&mut R) -> Self {
        Self::new(rng, SimulationParams::default(), WorldParams::default())
    }

    fn init_boids<R: Rng>(&mut self, rng: &mut R) {
        // Generate a given amount of boids in random positions
        self.boids.clear();

        for _ in 0..self.world_params.boid_count {
            let boid = self.random_boid(rng);
            self.boids.push(boid); 
        }
    }

    fn random_boid<R: Rng>(&mut self, rng: &mut R) -> Boid {
        let angle = math::random_range_f32(rng, 0.0, core::f32::consts::TAU);
        let min_speed = self.sim_params.max_speed * 0.4;
        let speed = math::random_range_f32(rng, min_speed, self.sim_params.max_speed); // How to load max speed in

        let start_velocity = Vec2::new(angle.cos(), angle.sin()) * speed;
        // We dont actually want to use sin and cos if we are dealing with embedded. there may be
        // implementations on esp32 hal but maybe we should try with a small direction lookup instead
        // As for our purposes we dont really need the full set of random directions anyway
        // Eventually the boids diverge and converge

        let area_size = self.world_params.area_size;
        let x = math::random_range_f32(rng, 0.0, area_size.x);
        let y = math::random_range_f32(rng, 0.0, area_size.y);

        Boid::new(Vec2::new(x, y), start_velocity)
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