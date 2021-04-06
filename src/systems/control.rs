use crate::components::{Agent, Force};
use specs::{prelude::*, ReadStorage, System, WriteStorage};

pub struct Control;

impl<'a> System<'a> for Control {
    type SystemData = (ReadStorage<'a, Agent>, WriteStorage<'a, Force>);

    fn run(&mut self, (agents, mut forces): Self::SystemData) {
        for (agent, force) in (&agents, &mut forces).join() {
            if let Some(inputs) = agent.inputs.as_ref() {
                let outputs = agent.network.propagate(inputs);
                if outputs.len() == 2 {
                    // if neuron had no output, do nothing
                    // otherwise, clamp output to (0, 1] and transform to (-10, 10]
                    force.rotation = if outputs[0] > f32::EPSILON {
                        (outputs[0].min(1.0) - 0.5) * 20.0
                    } else {
                        0.0
                    };
                    force.translation = if outputs[1] > f32::EPSILON {
                        (outputs[1].min(1.0) - 0.5) * 20.0
                    } else {
                        0.0
                    };
                }
            }
        }
    }
}
