use crate::components::Agent;
use crate::resources::{ResetInterval, Ticks};
use rand::thread_rng;
use specs::{prelude::*, ReadExpect, System, WriteStorage};

pub struct Mutate;

impl<'a> System<'a> for Mutate {
    type SystemData = (
        WriteStorage<'a, Agent>,
        ReadExpect<'a, Ticks>,
        ReadExpect<'a, ResetInterval>,
    );

    fn run(&mut self, (mut agents, ticks, interval): Self::SystemData) {
        let interval = interval.0;
        if ticks.get() % interval != 0 {
            return;
        }

        let mut rng = thread_rng();

        for agent in (&mut agents).join() {
            agent.network.mutate_uniform(&mut rng, 0.0125);
        }
    }
}
