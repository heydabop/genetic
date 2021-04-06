use crate::resources::Ticks;
use specs::{System, WriteExpect};

pub struct TickCounter;

impl<'a> System<'a> for TickCounter {
    type SystemData = WriteExpect<'a, Ticks>;

    fn run(&mut self, mut ticks: Self::SystemData) {
        ticks.inc();
    }
}
