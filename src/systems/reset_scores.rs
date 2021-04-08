use crate::components::Score;
use crate::resources::Ticks;
use specs::{prelude::*, ReadExpect, System, WriteStorage};

pub struct ResetScores;

impl<'a> System<'a> for ResetScores {
    type SystemData = (WriteStorage<'a, Score>, ReadExpect<'a, Ticks>);

    fn run(&mut self, (mut scores, ticks): Self::SystemData) {
        if ticks.get() % 7200 != 0 {
            return;
        }
        for s in (&mut scores).join() {
            s.reset();
        }
    }
}
