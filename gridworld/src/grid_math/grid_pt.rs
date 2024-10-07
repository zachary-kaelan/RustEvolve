use crate::consts::world::GRID_WIDTH;
use crate::*;
use rand::{Rng, RngCore};

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct GridPoint {
    pub x: u16,
    pub y: u16,
}

impl GridPoint {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    pub fn rand(rng: &mut dyn RngCore) -> Self {
        Self {
            x: rng.gen_range(0..GRID_WIDTH),
            y: rng.gen_range(0..GRID_WIDTH),
        }
    }

    /// Chessboard distance to the other `GridPoint`.
    pub fn dist(&self, other: Self) -> u16 {
        // chessboard distance
        self.x.abs_diff(other.x).max(self.y.abs_diff(other.y))
    }

    /// The position of `other` on the circle around this point
    pub fn angle_to(&self, other: Self) -> f32 {
        ((other.y - self.y) as f32).atan2((other.x - self.x) as f32)
    }
}

impl From<usize> for GridPoint {
    fn from(value: usize) -> Self {
        let y = value % GRID_WIDTH as usize;
        let x = (value - y) / GRID_WIDTH as usize;
        GridPoint::new(x as u16, y as u16)
    }
}

#[macro_export]
macro_rules! pt {
    ( $x:expr,$y:expr ) => {
        GridPoint { x: $x, y: $y }
    };
}

impl GridPoint {
    pub const fn move_dir(&self, dir: Direction) -> Self {
        // Clamp?
        match dir {
            Direction::Up => {
                pt![self.x, self.y - 1]
            }
            Direction::UpRight => {
                pt![self.x + 1, self.y - 1]
            }
            Direction::Right => {
                pt![self.x + 1, self.y]
            }
            Direction::DownRight => {
                pt![self.x + 1, self.y + 1]
            }
            Direction::Down => {
                pt![self.x, self.y + 1]
            }
            Direction::DownLeft => {
                pt![self.x - 1, self.y + 1]
            }
            Direction::Left => {
                pt![self.x - 1, self.y]
            }
            Direction::UpLeft => {
                pt![self.x - 1, self.y - 1]
            }
        }
    }
}

pub fn get_points_in_range(pos: GridPoint, max_range: u16) -> Vec<GridPoint> {
    let mut points = vec![];

    for x in (pos.x.max(max_range) - max_range)..=(pos.x + max_range).min(GRID_WIDTH - 1) {
        for y in (pos.y.max(max_range) - max_range)..=(pos.y + max_range).min(GRID_WIDTH - 1) {
            if x == y {
                continue;
            }
            let pt = pt!(x, y);
            points.push(pt);
        }
    }
    points
}

extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::consts::world::{GRID_AREA, GRID_WIDTH};
    use test::Bencher;

    #[bench]
    fn bench_angle_to(b: &mut Bencher) {
        let middle = pt!(GRID_WIDTH / 2, GRID_WIDTH / 2);
        b.iter(|| {
            for x in 0..GRID_WIDTH {
                for y in 0..GRID_WIDTH {
                    let angle = middle.angle_to(pt!(x, y));
                }
            }
        })
    }

    #[bench]
    fn bench_angle_to_lu(b: &mut Bencher) {
        let middle = pt!(GRID_WIDTH / 2, GRID_WIDTH / 2);
        let mut lu = [0.0f32; GRID_AREA];
        for x in 0..GRID_WIDTH {
            for y in 0..GRID_WIDTH {
                let angle = middle.angle_to(pt!(x, y));
                lu[(x * GRID_WIDTH + y) as usize] = angle;
            }
        }

        let lu = lu;

        b.iter(|| {
            for x in 0..GRID_WIDTH {
                for y in 0..GRID_WIDTH {
                    let angle = lu[(x * GRID_WIDTH + y) as usize];
                }
            }
        })
    }
}
