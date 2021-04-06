use crate::components::{Agent, Velocity};
use specs::{prelude::*, ReadStorage, System, WriteStorage};
use std::f32::consts::PI;

pub struct Control;

impl<'a> System<'a> for Control {
    type SystemData = (ReadStorage<'a, Agent>, WriteStorage<'a, Velocity>);

    fn run(&mut self, (agents, mut velocites): Self::SystemData) {
        for (agent, vel) in (&agents, &mut velocites).join() {
            if let Some(inputs) = agent.inputs.as_ref() {
                let outputs = agent.network.propagate(inputs);
                if outputs.len() == 2 {
                    println!("{:?}\n{:?}", inputs, outputs);
                    vel.heading = (vel.heading * outputs[0]).clamp(0.0, 2.0 * PI - f32::EPSILON);
                    vel.magnitude = (vel.magnitude * outputs[1]).clamp(1.0, 100.0);
                }
            }
        }
    }
}
