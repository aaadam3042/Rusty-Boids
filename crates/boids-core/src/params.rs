use crate::math::Vec2;

pub enum WrapMode {
    Wrap,
    Bounce
}

/// Parameters related to the simulation.
/// Some parameters are designed to be tuneable by users:
///    - alignment
///    - cohesion
///    - neighbour
///    - separation 
pub struct SimulationParams {
    pub max_speed: f32,
    pub max_force: f32,
    pub alignment_weight: f32,
    pub cohesion_weight: f32,
    pub neighbour_radius: f32,
    pub separation_radius: f32,
    pub separation_weight: f32,
    pub wrap_mode: WrapMode
}

impl Default for SimulationParams {
    fn default() -> Self {
        let default = Self {
            max_speed: 80.0,
            max_force: 20.0,
            neighbour_radius: 50.0,
            separation_radius: 20.0,
            alignment_weight: 1.0,
            cohesion_weight: 1.0,
            separation_weight: 1.5,
            wrap_mode: WrapMode::Wrap,
        };
        
        assert!(
            default.neighbour_radius >= default.separation_radius, 
            "Default neighbour radius must be greater than default separation radius"
        );
        default
    }
}

/// Parameters related to the world setup
pub struct WorldParams {
    
    pub area_size: Vec2,
    pub boid_count: usize,
}

impl Default for WorldParams {
    fn default() -> Self {
        Self {
            area_size: Vec2::new(500f32, 500f32),
            boid_count: 100,
        }
    }
}