use crate::components::Score;
use crate::resources::Ticks;
use specs::{prelude::*, ReadExpect, ReadStorage, System};

pub struct PrintStats;

impl<'a> System<'a> for PrintStats {
    type SystemData = (ReadStorage<'a, Score>, ReadExpect<'a, Ticks>);

    fn run(&mut self, (scores, ticks): Self::SystemData) {
        if ticks.get() % 7200 != 0 {
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
            "Avg: {} - Min: {} - Max: {} - Total: {}",
            avg, min, max, total
        );
    }
}
