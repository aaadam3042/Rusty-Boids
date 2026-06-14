use crate::boid::Boid;
use crate::simulation_params::SimulationParams;

pub struct World {
    boids: Vec<Boid>,

    height: f32,
    width: f32,

    params: SimulationParams,
}

impl World {
    fn new(boids: Vec<Boid>, height: f32, width: f32, params: SimulationParams) -> Self {
        Self {boids: boids, height: height, width: width, params: params}
    }

    fn tick(&mut self, dt: i32) {

    }

    fn get_boids(&self) -> &[Boid] {
        &self.boids
    }
}