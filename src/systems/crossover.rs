use crate::components::{Agent, Fitness};
use crate::neural::Network;
use crate::resources::{ResetInterval, Ticks};
use rand::{seq::SliceRandom, thread_rng};
use specs::{prelude::*, ReadExpect, System, WriteStorage};

struct NetworkFitness {
    network: Network,
    fitness: u32,
}

pub struct Crossover;

impl<'a> System<'a> for Crossover {
    type SystemData = (
        WriteStorage<'a, Agent>,
        WriteStorage<'a, Fitness>,
        ReadExpect<'a, Ticks>,
        ReadExpect<'a, ResetInterval>,
    );

    fn run(&mut self, (mut agents, fitnesses, ticks, interval): Self::SystemData) {
        let interval = interval.0;
        if ticks.get() % interval != 0 {
            return;
        }

        let networks: Vec<NetworkFitness> = (&agents, &fitnesses)
            .join()
            .map(|(agent, fitness)| NetworkFitness {
                network: agent.network.clone(),
                fitness: fitness.fitness,
            })
            .collect();

        let mut rng = thread_rng();

        for agent in (&mut agents).join() {
            let network_a = &networks
                .choose_weighted(&mut rng, |n| n.fitness)
                .unwrap()
                .network;
            let network_b = &networks
                .choose_weighted(&mut rng, |n| n.fitness)
                .unwrap()
                .network
                .clone();
            agent.network = network_a.crossover_uniform(&mut rng, network_b);
        }
    }
}
