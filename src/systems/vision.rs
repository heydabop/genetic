use crate::components::{Agent, Position, Target, Velocity};
use crate::resources::MaxPos;
use specs::{prelude::*, ReadStorage, System, WriteStorage};
use std::f32::consts::PI;

struct DistanceAngle {
    distance: f32,
    angle: f32,
}

pub struct Vision;

impl<'a> System<'a> for Vision {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteStorage<'a, Agent>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Velocity>,
        ReadStorage<'a, Target>,
        ReadExpect<'a, MaxPos>,
    );

    fn run(&mut self, (mut agents, positions, velocities, targets, max): Self::SystemData) {
        let viewing_distance = 600.0; // distance that an agent can see a target
        let vision_cone = PI; // agent field of view in radians
        for (agent, agent_pos, agent_velocity) in (&mut agents, &positions, &velocities).join() {
            let max = max.0;
            let num_receptors = agent.network.input_size();
            // individual vision receptor field of view
            let cone_slice = vision_cone / num_receptors as f32;
            // start and end of field of view
            let start = -vision_cone / 2.0;
            let end = vision_cone / 2.0;

            // duplicate targets across boundaries so that agents can "see" to the other side of the word
            let cloned_targets: Vec<Position> = (&positions, &targets)
                .join()
                .flat_map(|(&p, _)| {
                    vec![
                        p,
                        Position {
                            x: 0.0 - p.x,
                            y: p.y,
                        },
                        Position {
                            x: p.x,
                            y: 0.0 - p.y,
                        },
                        Position {
                            x: p.x + max.x,
                            y: p.y,
                        },
                        Position {
                            x: p.x,
                            y: p.y + max.y,
                        },
                        Position {
                            x: 0.0 - p.x,
                            y: 0.0 - p.y,
                        },
                        Position {
                            x: 0.0 - p.x,
                            y: p.y + max.y,
                        },
                        Position {
                            x: p.x + max.x,
                            y: 0.0 - p.y,
                        },
                        Position {
                            x: p.x + max.x,
                            y: p.y + max.y,
                        },
                    ]
                })
                .collect();
            // get distances and angles to all targets that are within agent's vision cone
            let mut visible_targets: Vec<DistanceAngle> = cloned_targets
                .iter()
                .filter_map(|target_pos| {
                    let y_diff = target_pos.y - agent_pos.y;
                    let x_diff = target_pos.x - agent_pos.x;
                    let mut angle = y_diff.atan2(x_diff);
                    if angle < 0.0 {
                        // keep angle within (0, 2PI]
                        angle += 2.0 * PI;
                    }
                    angle = agent_velocity.heading - angle;
                    if angle >= start && angle < end {
                        Some(DistanceAngle {
                            distance: target_pos.distance(&agent_pos),
                            angle,
                        })
                    } else {
                        None
                    }
                })
                .collect();
            // sort targets by distance from agent
            visible_targets.sort_by(|a, b| a.distance.partial_cmp(&(b.distance)).unwrap());

            let neuron_inputs: Vec<f32> = (0..num_receptors)
                .map(|i| {
                    // start and end of field of view for this receptor
                    let slice_start = start + (cone_slice * i as f32);
                    let slice_end = start + (cone_slice * (i + 1) as f32);
                    // find closest target within cone of vision
                    let closest_visible = visible_targets
                        .iter()
                        .find(|t| t.angle >= slice_start && t.angle < slice_end);
                    // return value [0, 1) based on distance to target (closer => 1)
                    match closest_visible {
                        None => 0.0,
                        Some(c) => ((viewing_distance - c.distance) / 800.0).max(0.0),
                    }
                })
                .collect();

            agent.inputs = Some(neuron_inputs);
        }
    }
}
