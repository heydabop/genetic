use rand::Rng;

#[derive(Debug)]
pub struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}

#[derive(Debug)]
pub struct Layer {
    neurons: Vec<Neuron>,
}

#[derive(Debug)]
pub struct Network {
    layers: Vec<Layer>,
}

impl Network {
    pub fn random<R: Rng + ?Sized>(rng: &mut R, neurons_per_layer: &[usize]) -> Self {
        let layers: Vec<Layer> = neurons_per_layer
            .windows(2)
            .map(|n| Layer {
                neurons: (0..n[0])
                    .map(|_| Neuron {
                        bias: rng.gen_range(-1.0..=1.0),
                        weights: (0..n[1]).map(|_| rng.gen_range(-1.0..=1.0)).collect(),
                    })
                    .collect(),
            })
            .collect();

        Self { layers }
    }
}
