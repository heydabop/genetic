use crate::components::{Agent, Rank};
use crate::neural::Network;
use crate::resources::Ticks;
use rand::{seq::SliceRandom, thread_rng};
use specs::{prelude::*, ReadExpect, System, WriteStorage};

struct NetworkRank {
    network: Network,
    rank: u32,
}

pub struct Crossover;

impl<'a> System<'a> for Crossover {
    type SystemData = (
        WriteStorage<'a, Agent>,
        WriteStorage<'a, Rank>,
        ReadExpect<'a, Ticks>,
    );

    fn run(&mut self, (mut agents, ranks, ticks): Self::SystemData) {
        if ticks.get() % 7200 != 0 {
            return;
        }

        let networks: Vec<NetworkRank> = (&agents, &ranks)
            .join()
            .map(|(agent, rank)| NetworkRank {
                network: agent.network.clone(),
                rank: rank.rank,
            })
            .collect();

        let mut rng = thread_rng();

        for agent in (&mut agents).join() {
            let network_a = &networks
                .choose_weighted(&mut rng, |n| n.rank)
                .unwrap()
                .network;
            let network_b = &networks
                .choose_weighted(&mut rng, |n| n.rank)
                .unwrap()
                .network
                .clone();
            agent.network = network_a.uniform_crossover(&mut rng, network_b);
        }
    }
}
