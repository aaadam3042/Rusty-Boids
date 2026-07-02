#![no_std]
extern crate alloc;

mod boid;
mod math;
mod world;
mod params;
mod spatial_grid;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
