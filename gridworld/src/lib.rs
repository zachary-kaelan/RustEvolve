#![feature(test)]

mod consts;
mod grid;
mod grid_math;
mod world;

pub use grid::*;
pub use grid_math::*;
pub use half::f16;

pub use std::f32::consts::PI;

pub const CREATURE_MEMORY_SIZE: usize = 16;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
