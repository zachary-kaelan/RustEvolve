#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up = 0,
    UpRight = 1,
    Right = 2,
    DownRight = 3,
    Down = 4,
    DownLeft = 5,
    Left = 6,
    UpLeft = 7,
}

use Direction::*;
static DIRECTIONS: [Direction; 8] = [Up, UpRight, Right, DownRight, Down, DownLeft, Left, UpLeft];

impl From<u8> for Direction {
    fn from(value: u8) -> Self {
        DIRECTIONS[value as usize]
    }
}
