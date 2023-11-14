use crate::components::Score;
use crate::resources::{ResetInterval, Ticks};
use specs::{prelude::*, ReadExpect, ReadStorage, RunningTime, System};

pub struct PrintStats;

impl<'a> System<'a> for PrintStats {
    type SystemData = (
        ReadStorage<'a, Score>,
        ReadExpect<'a, Ticks>,
        ReadExpect<'a, ResetInterval>,
    );

    fn run(&mut self, (scores, ticks, interval): Self::SystemData) {
        let interval = interval.0;
        if ticks.get() % interval != 0 {
            return;
        }
        let mut total = 0;
        let mut max = 0;
        let mut min = u32::MAX;
        let mut num = 0;
        for s in scores.join() {
            let s = s.score();
            total += s;
            max = max.max(s);
            min = min.min(s);
            num += 1;
        }
        let avg = total as f32 / num as f32;

        println!(
            "Gen {} = Avg: {:.2} - Min: {} - Max: {} - Total: {}",
            ticks.get() / interval,
            avg,
            min,
            max,
            total
        );
    }

    fn running_time(&self) -> RunningTime {
        RunningTime::VeryShort
    }
}
