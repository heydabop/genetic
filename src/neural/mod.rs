#![allow(dead_code)]

use rand::{
    distributions::{Distribution, Uniform},
    Rng,
};
use std::f32::EPSILON;
use std::iter::Iterator;

// A neural network where each neuron is stored as its bias and input weights
// Due to this, the first layer doesn't really exist, at least in that it doesn't have neurons with a bias.
// Instead we input raw values into the inputs of neurons in the second layer

#[derive(Debug, Clone)]
pub struct Neuron {
    bias: f32,
    input_weights: Vec<f32>,
}

impl Neuron {
    pub fn new(bias: f32, input_weights: Vec<f32>) -> Self {
        assert!(!input_weights.is_empty());

        Self {
            bias,
            input_weights,
        }
    }

    fn propagate(&self, inputs: &[f32]) -> f32 {
        assert!(inputs.len() == self.input_weights.len());

        // Sum up (input * weight) across the input synapses
        let input = inputs
            .iter()
            .zip(self.input_weights.iter())
            .map(|(i, w)| i * w)
            .reduce(|a, b| a + b)
            .unwrap();

        // Add the neuron's bias and clamp negative values to 0.
        (input + self.bias).max(0.0)
    }

    fn crossover_uniform<R: Rng + ?Sized>(&self, rng: &mut R, b: &Self) -> Self {
        Self {
            bias: if rng.gen::<bool>() { self.bias } else { b.bias },
            input_weights: self
                .input_weights
                .iter()
                .zip(b.input_weights.iter())
                .map(|(&a, &b)| if rng.gen::<bool>() { a } else { b })
                .collect(),
        }
    }

    fn mutate_uniform<R: Rng + ?Sized>(
        &mut self,
        mut rng: &mut R,
        range: Uniform<f32>,
        probability: f32,
    ) {
        if rng.gen::<f32>() < probability {
            self.bias = range.sample(&mut rng)
        }
        for w in &mut self.input_weights {
            if rng.gen::<f32>() < probability {
                *w = range.sample(&mut rng)
            }
        }
    }
}

impl PartialEq for Neuron {
    fn eq(&self, other: &Self) -> bool {
        (other.bias - self.bias).abs() < EPSILON
            && self
                .input_weights
                .iter()
                .zip(&other.input_weights)
                .all(|(a, b)| (b - a).abs() < EPSILON)
    }
}

#[derive(Debug, Clone)]
pub struct Layer {
    neurons: Vec<Neuron>,
}

impl Layer {
    pub fn new(neurons: Vec<Neuron>) -> Self {
        assert!(!neurons.is_empty());
        let num_weights = neurons[0].input_weights.len();

        assert!(neurons.iter().all(|n| n.input_weights.len() == num_weights));

        Self { neurons }
    }

    fn propagate(&self, inputs: &[f32]) -> Vec<f32> {
        assert!(inputs.len() == self.neurons[0].input_weights.len());

        // Feed the input values to each neuron in the layer and return their outputs

        self.neurons.iter().map(|n| n.propagate(inputs)).collect()
    }

    fn crossover_uniform<R: Rng + ?Sized>(&self, mut rng: &mut R, b: &Self) -> Self {
        assert_eq!(self.neurons.len(), b.neurons.len());

        Self {
            neurons: self
                .neurons
                .iter()
                .zip(b.neurons.iter())
                .map(|(a, b)| a.crossover_uniform(&mut rng, b))
                .collect(),
        }
    }

    fn mutate_uniform<R: Rng + ?Sized>(
        &mut self,
        rng: &mut R,
        range: Uniform<f32>,
        probability: f32,
    ) {
        for n in &mut self.neurons {
            n.mutate_uniform(rng, range, probability);
        }
    }
}

#[derive(Debug, Clone)]
pub struct Network {
    layers: Vec<Layer>,
}

impl Network {
    pub fn from_layers(layers: Vec<Layer>) -> Self {
        assert!(!layers.is_empty());

        Self { layers }
    }

    pub fn random<R: Rng + ?Sized>(mut rng: &mut R, neurons_per_layer: &[usize]) -> Self {
        assert!(neurons_per_layer.len() > 1);

        let range = Uniform::from(-1.0..1.0);

        // The first layer is intentionally not created, and instead the output values of what would be those neurons is represented with `inputs` in the propagate call
        let layers: Vec<Layer> = neurons_per_layer
            .windows(2)
            .map(|n| Layer {
                neurons: (0..n[1])
                    .map(|_| Neuron {
                        bias: range.sample(&mut rng),
                        input_weights: (0..n[0]).map(|_| range.sample(&mut rng)).collect(),
                    })
                    .collect(),
            })
            .collect();

        Self { layers }
    }

    pub fn propagate(&self, inputs: &[f32]) -> Vec<f32> {
        assert_eq!(inputs.len(), self.layers[0].neurons[0].input_weights.len());

        let inputs = inputs.to_vec();

        // Input the output of each layer into the next layer
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(&inputs))
    }

    pub fn input_size(&self) -> usize {
        self.layers[0].neurons[0].input_weights.len()
    }

    pub fn output_size(&self) -> usize {
        self.layers.last().unwrap().neurons.len()
    }

    pub fn crossover_uniform<R: Rng + ?Sized>(&self, mut rng: &mut R, b: &Self) -> Self {
        assert_eq!(self.layers.len(), b.layers.len());
        Self {
            layers: self
                .layers
                .iter()
                .zip(b.layers.iter())
                .map(|(a, b)| a.crossover_uniform(&mut rng, b))
                .collect(),
        }
    }

    pub fn mutate_uniform<R: Rng + ?Sized>(&mut self, rng: &mut R, probability: f32) {
        let range = Uniform::from(-1.0..1.0);

        for l in &mut self.layers {
            l.mutate_uniform(rng, range, probability);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand_pcg::Pcg64Mcg;

    #[test]
    fn neuron_propagate() {
        let neuron = Neuron::new(0.1, vec![0.3, 0.4, 0.6]);
        assert!((neuron.propagate(&[-0.1, 0.7, 0.3]) - 0.53).abs() < EPSILON);
    }

    #[test]
    fn neuron_propagate_zero() {
        let neuron = Neuron::new(-0.06, vec![0.6, 0.4, 0.5]);
        assert!(neuron.propagate(&[-0.5, 0.3, 0.45]).abs() < EPSILON);
    }

    #[test]
    fn layer_propagate() {
        let neurons = vec![
            Neuron::new(0.6, vec![0.4, 0.6]),
            Neuron::new(0.8, vec![0.2, 0.5]),
            Neuron::new(-0.4, vec![0.7, 0.3]),
        ];
        let layer = Layer::new(neurons);
        let outputs = layer.propagate(&[0.7, 0.1]);

        assert!((outputs[0] - 0.94).abs() < EPSILON);
        assert!((outputs[1] - 0.99).abs() < EPSILON);
        assert!((outputs[2] - 0.12).abs() < EPSILON);
    }

    #[test]
    fn network_propagate() {
        let network = Network::from_layers(vec![
            Layer::new(vec![
                Neuron::new(0.6, vec![0.4, 0.6]),
                Neuron::new(0.8, vec![0.2, 0.5]),
                Neuron::new(-0.4, vec![0.7, 0.3]),
            ]),
            Layer::new(vec![Neuron::new(0.5, vec![0.3, 0.4, 0.5])]),
        ]);

        assert!((network.propagate(&[0.7, 0.1])[0] - 1.238).abs() < EPSILON);
    }

    #[test]
    fn neuron_crossover_uniform() {
        let mut rng = Pcg64Mcg::new(0xcafef00dd15ea5e5);

        let a = Neuron::new(0.1, (0..10).map(|i| i as f32).collect());
        let b = Neuron::new(-0.1, (0..10).map(|i| (i * 20 + 1) as f32).collect());

        let c = a.crossover_uniform(&mut rng, &b);
        assert_eq!(
            c,
            Neuron {
                bias: 0.1,
                input_weights: vec![0.0, 1.0, 41.0, 3.0, 81.0, 5.0, 6.0, 7.0, 161.0, 181.0]
            }
        );
    }

    #[test]
    fn neuron_mutate() {
        let mut rng = Pcg64Mcg::new(0xcafef00dd15ea5e5);
        let range = Uniform::from(-1.0..1.0);

        let mut a = Neuron {
            bias: 0.1,
            input_weights: vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9],
        };

        a.mutate_uniform(&mut rng, range, 0.2);
        assert_eq!(
            a,
            Neuron {
                bias: 0.1,
                input_weights: vec![0.0, 0.1, 0.2, 0.3, 0.18795109, 0.5, 0.6, 0.7, 0.8, 0.882401]
            }
        );
    }
}
