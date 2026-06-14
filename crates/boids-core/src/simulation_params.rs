pub enum WrapMode {
    Wrap,
    Bounce
}

pub struct SimulationParams {
    max_speed: f32,
    max_force: f32,
    neighbour_radius: f32,
    separation_radius: f32,
    alignment_weight: f32,
    cohesion_weight: f32,
    separation_weight: f32,
    wrap_mode: WrapMode
}

impl Default for SimulationParams {
    fn default() -> Self {
        Self {
            max_speed: 4.0,
            max_force: 0.1,
            neighbour_radius: 50.0,
            separation_radius: 20.0,
            alignment_weight: 1.0,
            cohesion_weight: 1.0,
            separation_weight: 1.5,
            wrap_mode: WrapMode::Wrap,
        }
    }
}
