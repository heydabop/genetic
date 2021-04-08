use crate::components::Score;
use crate::resources::{ResetInterval, Ticks};
use specs::{prelude::*, ReadExpect, System, WriteStorage};

pub struct ResetScores;

impl<'a> System<'a> for ResetScores {
    type SystemData = (
        WriteStorage<'a, Score>,
        ReadExpect<'a, Ticks>,
        ReadExpect<'a, ResetInterval>,
    );

    fn run(&mut self, (mut scores, ticks, interval): Self::SystemData) {
        let interval = interval.0;
        if ticks.get() % interval != 0 {
            return;
        }
        for s in (&mut scores).join() {
            s.reset();
        }
    }
}
