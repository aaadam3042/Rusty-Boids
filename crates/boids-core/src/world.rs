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
    pub fn tick(&mut self, dt: i32) {
        let boids_snapshot = self.boids.clone();

        // Rebuild the spatial grid for this tick
        self.spatial_grid.clear();
        for (index, boid) in boids_snapshot.iter().enumerate() {
            self.spatial_grid.insert(index, boid.position);
        }

        /* for each boid:
        1. calculate steering force
        2. apply force
        3. update position and velocity 
        4. handle world boundaries */
        for (index, boid) in self.boids.iter_mut().enumerate() {
            let steering_force = calculate_flocking_force(
                index, &boids_snapshot, &self.spatial_grid, &self.sim_params
            );

            boid.apply_force(steering_force);
            boid.update(dt);
            boid.handle_boundaries(self.world_params.area_size, &self.sim_params.wrap_mode);
        }
    }
}
    
// 3. World Public accessors
impl World {
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
    
        Boid::new(Vec2::new(x, y), start_velocity)
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
    let mut alignment_force = Vec2::ZERO;
    let mut cohesion_force = Vec2::ZERO;
    let mut separation_force = Vec2::ZERO;

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
        if neighbour_index == index {
            continue; // Skip self
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
        alignment_force = alignment_force + neighbour_boid.velocity;

        // Cohesion: Steer to move toward the average position of local flockmates
        cohesion_force = cohesion_force + neighbour_boid.position;

        // Separation: Steer to avoid crowding local flockmates
        if distance_squared > 0.001 && distance_squared < separation_radius_sq {
            let diff = current_boid.position - neighbour_boid.position;
            separation_force = separation_force + diff / distance_squared; // Weight by distance_squared
            separation_count = separation_count + 1;
        }   
    }

    // Calculate the average alignment and cohesion forces
    if neighbour_count > 0 {
        let neighbour_count_f = neighbour_count as f32;

        // Alignment
        let average_velocity = alignment_force / neighbour_count_f;
        alignment_force = average_velocity - current_boid.velocity;

        // Cohesion
        let average_position = cohesion_force / neighbour_count_f;
        let direction_to_center = average_position - current_boid.position;
        let desired_velocity = direction_to_center.limit_length(sim_params.max_speed);
        cohesion_force = desired_velocity - current_boid.velocity;
    } else {
        alignment_force = Vec2::ZERO;
        cohesion_force = Vec2::ZERO;
    }

    if separation_count > 0 {
        let separation_count_f = separation_count as f32;

        let average_avoidance = separation_force / separation_count_f;
        let desired_velocity = average_avoidance.limit_length(sim_params.max_speed);
        separation_force = desired_velocity - current_boid.velocity;
    } else {
        separation_force = Vec2::ZERO;
    }

    // Apply weights from SimulationParams
    alignment_force = alignment_force * sim_params.alignment_weight;
    cohesion_force = cohesion_force * sim_params.cohesion_weight;
    separation_force = separation_force * sim_params.separation_weight;

    // Combine the forces
    let mut steering_force = alignment_force + cohesion_force + separation_force;
    steering_force = steering_force.limit_length(sim_params.max_force);

    steering_force
}
    