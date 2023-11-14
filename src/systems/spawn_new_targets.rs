use crate::components::Position;
use crate::resources::{HitTargets, MaxPos};
use rand::{thread_rng, Rng};
use specs::{prelude::*, RunningTime, System, WriteStorage};

pub struct SpawnNewTargets;

impl<'a> System<'a> for SpawnNewTargets {
    type SystemData = (
        WriteStorage<'a, Position>,
        WriteExpect<'a, HitTargets>,
        ReadExpect<'a, MaxPos>,
        Entities<'a>,
    );

    fn run(&mut self, (mut position, mut hit_targets, max, entities): Self::SystemData) {
        let max = max.0;
        hit_targets.0.drain().for_each(|id| {
            let t = entities.entity(id);
            let pos = position.get_mut(t).expect("Unable to find old target");
            pos.x = thread_rng().gen_range(0.0..max.x);
            pos.y = thread_rng().gen_range(0.0..max.y);
        });
    }

    fn running_time(&self) -> RunningTime {
        RunningTime::Short
    }
}
