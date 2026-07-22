use crate::math::Vec2;
use crate::params::WrapMode;

pub struct Boid {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
}

impl Boid {
    pub fn new(position: Vec2, velocity: Vec2, acceleration: Vec2) -> Self {
        Boid {position: position, velocity: velocity, acceleration: acceleration}
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.acceleration = self.acceleration + force;
    }

    pub fn update(&mut self, dt: f32, max_speed: f32) {
        assert!(dt > 0.0, "dt must be greater than 0.0");
        assert!(dt.is_finite(), "dt must be finite");
        assert!(max_speed > 0.0, "max_speed must be greater than 0.0");
        assert!(max_speed.is_finite(), "max_speed must be finite");

        self.velocity = self.velocity + self.acceleration * dt;

        // Limit the speed to max_speed
        self.velocity = self.velocity.limit_length(max_speed);

        self.position = self.position + self.velocity * dt;
        self.acceleration = Vec2 {x: 0.0, y: 0.0};
    }

    pub fn handle_boundaries(&mut self, world_dimension: Vec2, wrap_mode: &WrapMode) {
        match wrap_mode {
            WrapMode::Wrap => {
                while self.position.x < 0.0 {
                    self.position.x += world_dimension.x;
                }

                while self.position.x > world_dimension.x {
                    self.position.x -= world_dimension.x;
                }

                while self.position.y < 0.0 {
                    self.position.y += world_dimension.y;
                }

                while self.position.y > world_dimension.y {
                    self.position.y -= world_dimension.y;
                }
            },
            WrapMode::Bounce => {
                if self.position.x < 0.0 {
                    self.position.x = 0.0;
                    self.velocity.x = -self.velocity.x;
                } else if self.position.x > world_dimension.x {
                    self.position.x = world_dimension.x;
                    self.velocity.x = -self.velocity.x;
                }

                if self.position.y < 0.0 {
                    self.position.y = 0.0;
                    self.velocity.y = -self.velocity.y;
                } else if self.position.y > world_dimension.y {
                    self.position.y = world_dimension.y;
                    self.velocity.y = -self.velocity.y;
                }
            },
        }
    }
}
