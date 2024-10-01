use crate::config::Config;
use crate::*;
use ndarray::{Array1, Array2};
use std::cell::Cell;
use std::iter::zip;
use std::rc::Rc;

#[derive(Debug)]
pub struct Brain {
    speed_accel: f32,
    rotation_accel: f32,
    pub fitness: Cell<f32>,
    nn: Network,
}

impl Individual<Config> for Rc<Brain> {
    fn random(params: &Config, rng: &mut dyn RngCore) -> Self {
        let nn = Network::random(&Brain::topology(params), rng);
        Rc::new(Brain::new(params, nn))
    }

    fn fitness(&self) -> f32 {
        self.fitness.get()
    }

    fn mutate(self, params: &Config, rng: &mut dyn RngCore) -> Self {
        if rng.gen_bool(params.ga_mut_chance as f64) {
            return self;
        }

        let mut layers = vec![];
        for l1 in &self.nn.layers {
            let weights: Vec<f32> = l1
                .weights
                .iter()
                .map(|x1| {
                    if rng.gen_bool(params.ga_mut_chance as f64) {
                        let sign = if rng.gen_bool(0.5) { -1.0 } else { 1.0 };
                        *x1 + sign * params.ga_mut_coeff * rng.gen::<f32>()
                    } else {
                        *x1
                    }
                })
                .collect();
            let weights =
                Array2::from_shape_vec((l1.weights.shape()[0], l1.weights.shape()[1]), weights);
            let biases: Array1<f32> = l1
                .biases
                .iter()
                .map(|x1| {
                    if rng.gen_bool(params.ga_mut_chance as f64) {
                        let sign = if rng.gen_bool(0.5) { -1.0 } else { 1.0 };
                        *x1 + sign * params.ga_mut_coeff * rng.gen::<f32>()
                    } else {
                        *x1
                    }
                })
                .collect();
            let layer = Layer::new(l1.layer_type, weights.unwrap(), biases);
            layers.push(layer);
        }
        let nn = Network::new(layers);
        Rc::new(Brain::new(params, nn))
    }

    fn crossover(&self, other: &Self, params: &Config, rng: &mut dyn RngCore) -> Self {
        let mut layers = vec![];
        for (l1, l2) in self.nn.layers.iter().zip(&other.nn.layers) {
            let weights: Vec<f32> = l1
                .weights
                .iter()
                .zip(l2.weights.iter())
                .map(|(x1, x2)| if rng.gen_bool(0.5) { *x1 } else { *x2 })
                .collect();
            let weights =
                Array2::from_shape_vec((l1.weights.shape()[0], l1.weights.shape()[1]), weights);
            let biases: Array1<f32> = l1
                .biases
                .iter()
                .zip(l2.biases.iter())
                .map(|(x1, x2)| if rng.gen_bool(0.5) { *x1 } else { *x2 })
                .collect();
            let layer = Layer::new(l1.layer_type, weights.unwrap(), biases);
            layers.push(layer);
        }
        let nn = Network::new(layers);
        Rc::new(Brain::new(params, nn))
    }
}

impl Brain {
    pub(crate) fn from_network(config: &Config, network: Network) -> Self {
        Self::new(config, network)
    }

    pub(crate) fn process(&self, inputs: Array1<f32>) -> (f32, f32, f32) {
        let response = self.nn.forward(inputs);

        let r0 = response[0].clamp(0.0, 1.0) - 0.5;
        let r1 = response[1].clamp(0.0, 1.0) - 0.5;
        let speed = (r0 + r1).clamp(-self.speed_accel, self.speed_accel);
        let rotation = (r0 - r1).clamp(-self.rotation_accel, self.rotation_accel);
        let boost = if response[2] > 0.0 { 1.0 } else { -1.0 };

        (speed, rotation, boost)
    }
}

impl Brain {
    fn new(config: &Config, nn: Network) -> Self {
        Self {
            speed_accel: config.sim_speed_accel,
            rotation_accel: config.sim_rotation_accel,
            nn,
            fitness: Cell::new(0.0),
        }
    }

    fn topology(config: &Config) -> [usize; 3] {
        [config.eye_cells * 4 + 1, config.brain_neurons, 3]
    }
}
