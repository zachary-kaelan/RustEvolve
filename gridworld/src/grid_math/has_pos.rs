use crate::GridPoint;

pub trait HasPos {
    fn get_pos(&self) -> GridPoint;
}
