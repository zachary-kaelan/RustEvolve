use crate::*;

pub struct Creature {
    pub memory: [f16; CREATURE_MEMORY],
    pub energy: u16,
}

impl Default for Creature {
    fn default() -> Self {
        Self {
            memory: [f16::from(0u8); CREATURE_MEMORY],
            energy: consts::energy::CREATURE_START_ENERGY,
        }
    }
}
