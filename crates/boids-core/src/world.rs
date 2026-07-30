use alloc::vec::Vec;
use rand_core::Rng;

use crate::boid::Boid;
use crate::math::{self, Vec2};
use crate::params::{SimulationParams, WorldParams};
use crate::spatial_grid::SpatialGrid;

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
    world_params: WorldParams,
    sim_params: SimulationParams,
    spatial_grid: SpatialGrid
}

// 1. World Constructors
impl World {
    /// Create a new world with given simulation params and world params structures
    pub fn new<R: Rng>(rng:&mut R, sim_params: SimulationParams, world_params: WorldParams) -> Self {
        let spatial_grid = SpatialGrid::new(world_params.area_size, sim_params.neighbour_radius);
        let mut world = Self {
            boids: Vec::new(), 
            sim_params: sim_params, 
            world_params: world_params,
            spatial_grid: spatial_grid
        };
        world.init_boids(rng);

        world
    }
    
    /// Create new World with param defaults defined in params.rs
    pub fn new_default_params<R: Rng>(rng:&mut R) -> Self {
        Self::new(rng, SimulationParams::default(), WorldParams::default())
    }
}
    
// 2. World Lifecycle 
impl World {
    pub fn tick(&mut self, dt: f32) {
        assert!(dt > 0.0, "dt must be greater than 0.0");
        assert!(dt.is_finite(), "dt must be finite");

        // 1. Rebuild the spatial grid for this tick
        self.spatial_grid.clear();
        for (index, boid) in self.boids.iter().enumerate() {
            self.spatial_grid.insert(index, boid.position);
        }

        // 2. Calculate flocking forces before updating boids to avoid race
        let mut steering_forces: Vec<Vec2> = Vec::with_capacity(self.boids.len());
        for index in 0..self.boids.len() {
            let steering_force = calculate_flocking_force(
                index, &self.boids, &self.spatial_grid, &self.sim_params
            );
            steering_forces.push(steering_force);
        }

        // 3. Apply forces and update boids
        for (index, boid) in self.boids.iter_mut().enumerate() {
            boid.apply_force(steering_forces[index]);
            boid.update(dt, self.sim_params.max_speed);
            boid.handle_boundaries(self.world_params.area_size, &self.sim_params.wrap_mode);
        }

    }
}
    
// 3. World Public accessors
impl World {
    pub fn area_size(&self) -> Vec2 {
        self.world_params.area_size
    }

    pub fn boids(&self) -> &[Boid] {
        &self.boids
    }
}

// 4. World Private Helpers
impl World {
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
    fn random_boid<R: Rng>(&self, rng: &mut R) -> Boid {
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
    
        Boid::new(Vec2::new(x, y), start_velocity, Vec2::ZERO)
    }
}

/// Calcuate the flocking behaviour steering force of a boid.
fn calculate_flocking_force(index: usize, boids: &[Boid], spatial_grid: &SpatialGrid, sim_params: &SimulationParams) -> Vec2 {
    // 1. Calculate alignment force
    // 2. Calculate cohesion force
    // 3. Calculate separation force
    // 4. Combine forces with weights from SimulationParams + neighbour radius and separation radius

    let current_boid = &boids[index];

    // Accumulators
    let mut alignment_sum = Vec2::ZERO;
    let mut cohesion_sum = Vec2::ZERO;
    let mut separation_sum = Vec2::ZERO;

    let neighbour_radius_sq = sim_params.neighbour_radius * sim_params.neighbour_radius;
    let separation_radius_sq = sim_params.separation_radius * sim_params.separation_radius;

    // Loop over the candidate indices of the neighbouring boids from the spatial grid
    let neighbour_indices = spatial_grid.nearby_boid_indices(
        current_boid.position, 
        sim_params.neighbour_radius 
    );
    let mut neighbour_count = 0;
    let mut separation_count = 0;
    for neighbour_index in neighbour_indices {
        // Skip self
        if neighbour_index == index {
            continue; 
        }

        let neighbour_boid = &boids[neighbour_index];
        
        // If the neighbour is outside the neighbour radius, skip it
        let offset = neighbour_boid.position - current_boid.position;
        let distance_squared = offset.length_squared();
        if distance_squared > neighbour_radius_sq {
            continue;
        }

        neighbour_count = neighbour_count + 1;

        // Alignment: Steer towards the average heading of local flockmates
        alignment_sum = alignment_sum + neighbour_boid.velocity;

        // Cohesion: Steer to move toward the average position of local flockmates
        cohesion_sum = cohesion_sum + neighbour_boid.position;

        // Separation: Steer to avoid crowding local flockmates
        if distance_squared > 0.001 && distance_squared < separation_radius_sq {
            let diff = current_boid.position - neighbour_boid.position;
            let strength = separation_radius_sq / distance_squared - 1.0;
            separation_sum = separation_sum + diff * strength;
            separation_count = separation_count + 1;
        }   
    }

    // Alignment
    let alignment_force =
        if neighbour_count > 0 {
            let neighbour_count_f =
                neighbour_count as f32;

            let average_velocity =
                alignment_sum
                    / neighbour_count_f;

            average_velocity
                - current_boid.velocity
        } else {
            Vec2::ZERO
        };

    // Cohesion
    let cohesion_force =
        if neighbour_count > 0 {
            let neighbour_count_f =
                neighbour_count as f32;

            let average_position =
                cohesion_sum
                    / neighbour_count_f;

            average_position
                - current_boid.position
        } else {
            Vec2::ZERO
        };

    // Separation
    let separation_force =
        if separation_count > 0 {
            separation_sum / separation_count as f32
        } else {
            Vec2::ZERO
        };

    let speed_squared = current_boid.velocity.length_squared();

    let preferred_speed_squared = sim_params.preferred_speed * sim_params.preferred_speed;
    let speed_error = 1.0 - speed_squared / preferred_speed_squared;
    let propulsion_force = current_boid.velocity * speed_error * sim_params.propulsion_weight;

    let steering_force =
        alignment_force * sim_params.alignment_weight
        + cohesion_force * sim_params.cohesion_weight
        + separation_force * sim_params.separation_weight
        + propulsion_force;

    steering_force.limit_length(
        sim_params.max_force,
    )
}
    