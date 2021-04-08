use crate::components::{Fitness, Score};
use crate::resources::{ResetInterval, Ticks};
use specs::{prelude::*, ReadExpect, ReadStorage, System, WriteStorage};

pub struct RouletteSelection;

impl<'a> System<'a> for RouletteSelection {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Score>,
        WriteStorage<'a, Fitness>,
        ReadExpect<'a, Ticks>,
        ReadExpect<'a, ResetInterval>,
    );

    fn run(&mut self, (entities, scores, mut fitnesses, ticks, interval): Self::SystemData) {
        let interval = interval.0;
        if ticks.get() % interval != 0 {
            return;
        }

        for (entity, score) in (&entities, &scores).join() {
            fitnesses
                .insert(
                    entity,
                    Fitness {
                        fitness: score.score(),
                    },
                )
                .expect("Unable to overwrite fitness");
        }
    }
}
