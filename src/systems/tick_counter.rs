use crate::resources::Ticks;
use specs::{RunningTime, System, WriteExpect};

pub struct TickCounter;

impl<'a> System<'a> for TickCounter {
    type SystemData = WriteExpect<'a, Ticks>;

    fn run(&mut self, mut ticks: Self::SystemData) {
        ticks.inc();
    }

    fn running_time(&self) -> RunningTime {
        RunningTime::VeryShort
    }
}
