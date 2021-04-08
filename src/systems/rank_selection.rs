use crate::components::{Rank, Score};
use crate::resources::Ticks;
use specs::{prelude::*, ReadExpect, ReadStorage, System, WriteStorage};

pub struct RankSelection;

impl<'a> System<'a> for RankSelection {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Score>,
        WriteStorage<'a, Rank>,
        ReadExpect<'a, Ticks>,
    );

    fn run(&mut self, (entities, scores, mut ranks, ticks): Self::SystemData) {
        if ticks.get() % 7200 != 0 {
            return;
        }
        // sort scores in ascending order and remove duplicates (so equal scores can tie and have equal ranks)
        let mut sorted_scores: Vec<u32> = scores.join().map(|s| s.score()).collect();
        sorted_scores.sort_unstable();
        sorted_scores.dedup();

        // rank each score in ascendering order
        // i.e scores of [2, 4, 4, 8, 13] would rank [1, 2, 2, 3, 4]
        // these ranks get fed into rand's choose_weighted
        for (entity, score) in (&entities, &scores).join() {
            let rank = sorted_scores
                .iter()
                .position(|&s| s == score.score())
                .unwrap() as u32
                + 1;
            ranks
                .insert(entity, Rank { rank })
                .expect("Unable to overwrite rank");
        }
    }
}
