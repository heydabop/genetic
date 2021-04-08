use crate::components::Position;
use std::collections::HashSet;

pub struct DeltaTime(pub f32);

pub struct MaxPos(pub Position);

pub struct HitTargets(pub HashSet<specs::world::Index>);

#[derive(Default)]
pub struct Ticks(u64);

impl Ticks {
    pub fn inc(&mut self) {
        self.0 += 1;
    }

    pub fn get(&self) -> u64 {
        self.0
    }
}

// How frequently (in ticks) to generate a new population and reset scores
pub struct ResetInterval(pub u64);
