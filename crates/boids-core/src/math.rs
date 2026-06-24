use core::ops::{Add, Sub, Mul, Div};
use rand_core::Rng;

pub fn random_range_f32<R: Rng>(rng: &mut R, min:f32, max: f32) -> f32 {
    min + (rng.next_u32() as f32 / u32::MAX as f32) * (max - min)
}

#[derive(Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {x: self.x + other.x, y: self.y + other.y}
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub (self, rhs: Self) -> Self {
        Self {x: self.x - rhs.x, y: self.y - rhs.y}
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {x: self.x * rhs, y: self.y*rhs}
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {x: self.x/rhs, y: self.y/rhs}
    }
}

impl Vec2 {
    pub const ZERO: Self = Self {x: 0.0, y: 0.0};

    pub fn new(x: f32, y:f32) -> Self {
        Self {x: x, y: y}
    }

    pub fn length_squared(&self) -> f32 {
        self.x.powi(2) + self.y.powi(2)
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn normalise(self) -> Self {
        self/self.length()
    }

    pub fn limit_length(self, limit: f32) -> Self {
        if self.length_squared() > limit * limit {
            self.normalise() * limit
        } else {
            self
        }
    }
}