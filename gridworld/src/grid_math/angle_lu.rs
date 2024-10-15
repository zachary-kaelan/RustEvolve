use crate::consts::senses::EYE_RANGE;
use crate::consts::world::GRID_WIDTH;
use crate::*;
use std::f32::consts::PI;
use std::ops::Index;

static LU: std::sync::LazyLock<AngleLU> = std::sync::LazyLock::new(AngleLU::new);

struct AngleLU {
    cache: [f32; (EYE_RANGE * EYE_RANGE * 4) as usize],
}

impl AngleLU {
    fn new() -> Self {
        let mut cache = [0f32; (EYE_RANGE * EYE_RANGE * 4) as usize];
        let mid = pt!(EYE_RANGE, EYE_RANGE);
        for x in 0..(EYE_RANGE * 2) {
            for y in 0..(EYE_RANGE * 2) {
                let pt = pt!(x, y);
                let angle = mid.angle_to(pt);
                cache[(pt.x * EYE_RANGE * 2) as usize + pt.y as usize] =
                    if angle < 0.0 { 2.0 * PI - angle } else { angle };
            }
        }
        Self { cache }
    }

    fn at(&self, pos: GridPoint, other: GridPoint) -> f32 {
        let eff_pt = pt!(EYE_RANGE + other.x - pos.x, EYE_RANGE + other.y - pos.y);
        self.cache[(eff_pt.x * EYE_RANGE) as usize + eff_pt.y as usize]
    }
}

pub fn get_angle(pos: GridPoint, other: GridPoint) -> f32 {
    LU.at(pos, other)
}
