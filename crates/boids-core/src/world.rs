use rand_core::Rng;

use crate::boid::Boid;
use crate::math::{self, Vec2};
use crate::params::{SimulationParams, WorldParams};

/* 
Hard coded directions to avoid the use of trig functions. 
Particularly for embedded application efficiency.
*/
const DIAGONAL_DIRECTION: f32 = 0.70710677;
const STARTING_DIRECTIONS: [Vec2; 8] = [
    Vec2 {x: 1.0, y: 0.0},
    Vec2 {x: 0.0, y: 1.0},
    Vec2 {x: -1.0, y: 0.0},
    Vec2 {x: 0.0, y: -1.0},
    Vec2 {x: DIAGONAL_DIRECTION, y: DIAGONAL_DIRECTION},
    Vec2 {x: -DIAGONAL_DIRECTION, y: DIAGONAL_DIRECTION},
    Vec2 {x: -DIAGONAL_DIRECTION, y: -DIAGONAL_DIRECTION},
    Vec2 {x: DIAGONAL_DIRECTION, y: -DIAGONAL_DIRECTION}
];

pub struct World {
    boids: Vec<Boid>,
    sim_params: SimulationParams,
    world_params: WorldParams,
}

impl World {
    fn new<R: Rng>(rng:&mut R, sim_params: SimulationParams, world_params: WorldParams) -> Self {
        let mut world = Self {
            boids: Vec::new(), sim_params: sim_params, world_params: world_params
        };
        world.init_boids(rng);

        world
    }
    
    /// Create new World with param defaults defined in params.rs
    fn new_default_params<R: Rng>(rng:&mut R) -> Self {
        Self::new(rng, SimulationParams::default(), WorldParams::default())
    }

    /// Clear and populate the specified number of boids in WorldParams in
    /// random positions, with random starting velocities.
    fn init_boids<R: Rng>(&mut self, rng: &mut R) {
        self.boids.clear();

        // Generate and push boids to instance's Boids
        for _ in 0..self.world_params.boid_count {
            let boid = self.random_boid(rng);
            self.boids.push(boid); 
        }
    }

    /// Generates a random boid with random positions and velocity
    fn random_boid<R: Rng>(&mut self, rng: &mut R) -> Boid {
        // Define some minimum speed for random speed range. This is partially arbitrary right now
        let min_speed = self.sim_params.max_speed * 0.25;
        let speed = math::random_range_f32(rng, min_speed, self.sim_params.max_speed); 
        
        // Randomly select pre-defined vectors to avoid trig function
        let index = math::random_index(rng, STARTING_DIRECTIONS.len());
        let direction_vector = STARTING_DIRECTIONS[index];

        let start_velocity = direction_vector * speed;

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