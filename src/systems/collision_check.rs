use crate::components::{Position, Score, Target};
use crate::resources::HitTargets;
use specs::{prelude::*, ReadStorage, System, WriteStorage};

pub struct CollisionCheck;

impl<'a> System<'a> for CollisionCheck {
    type SystemData = (
        ReadStorage<'a, Position>,
        WriteStorage<'a, Score>,
        ReadStorage<'a, Target>,
        WriteExpect<'a, HitTargets>,
        Entities<'a>,
    );

    fn run(&mut self, (position, mut score, target, mut hit_targets, entities): Self::SystemData) {
        let hit_targets = &mut (hit_targets.0);
        for (pos, score) in (&position, &mut score).join() {
            for (target, _, e) in (&position, &target, &entities).join() {
                // bounding box check
                if (pos.x - target.x).abs() < 4.0 && (pos.y - target.y).abs() < 4.0 {
                    // inside circle check
                    if (pos.x - target.x).powi(2) + (pos.y - target.y).powi(2) < 16.0 {
                        // It's possible for multiple agents to hit the same target in a single tick here
                        // I'm okay with this because it seems "confusing" for an agent to follow behavior that normally results in a hit and it suddenly get nothing
                        hit_targets.insert(e.id());
                        score.inc();
                    }
                }
            }
        }
    }
}
