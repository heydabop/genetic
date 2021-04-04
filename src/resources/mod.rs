use crate::components::Position;
use std::collections::HashSet;

pub struct DeltaTime(pub f32);

pub struct MaxPos(pub Position);

pub struct HitTargets(pub HashSet<specs::world::Index>);
