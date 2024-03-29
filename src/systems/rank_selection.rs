use crate::components::{Fitness, Score};
use crate::resources::{ResetInterval, Ticks};
use specs::{prelude::*, ReadExpect, ReadStorage, RunningTime, System, WriteStorage};

pub struct RankSelection;

impl<'a> System<'a> for RankSelection {
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
        // sort scores in ascending order and remove duplicates (so equal scores can tie and have equal fitnesses)
        let mut sorted_scores: Vec<u32> = scores.join().map(|s| s.score()).collect();
        sorted_scores.sort_unstable();
        sorted_scores.dedup();

        // rank each score in ascendering order
        // i.e scores of [2, 4, 4, 8, 13] would rank [1, 2, 2, 3, 4]
        // these ranks get fed into rand's choose_weighted
        for (entity, score) in (&entities, &scores).join() {
            let fitness = sorted_scores
                .iter()
                .position(|&s| s == score.score())
                .unwrap() as u32
                + 1;
            fitnesses
                .insert(entity, Fitness { fitness })
                .expect("Unable to overwrite fitness");
        }
    }

    fn running_time(&self) -> RunningTime {
        RunningTime::Short
    }
}
