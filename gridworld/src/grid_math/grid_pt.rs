#[derive(Copy, Clone, Debug)]
pub struct GridPoint {
    pub x: u16,
    pub y: u16,
}

impl GridPoint {
    /// Chessboard distance to the other `GridPoint`.
    pub fn dist(&self, other: &Self) -> u16 {
        // chessboard distance
        self.x.abs_diff(other.x).max(self.y.abs_diff(other.y))
    }

    /// The position of `other` on the circle around this point
    pub fn angle_to(&self, other: &Self) -> f32 {
        ((other.y - self.y) as f32).atan2((other.x - self.x) as f32)
    }
}
