#![no_std]
extern crate alloc;

mod boid;
mod math;
mod world;
mod params;
mod spatial_grid;

pub use boid::Boid;
pub use math::Vec2;
pub use params::{SimulationParams, WorldParams, WrapMode};
pub use world::World;