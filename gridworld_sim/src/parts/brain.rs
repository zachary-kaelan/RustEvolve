use crate::*;
use ndarray::{Array1, Array2};
use rand::{Rng, RngCore};
use std::cell::Cell;
use std::rc::Rc;

pub struct Brain {
    pub fitness: Cell<f32>,
    pub time_since_move: Cell<u16>,
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
    pub(crate) fn add_fitness(&self, amount: f32) {
        self.fitness.set(self.fitness.get() + amount);
    }

    pub(crate) fn from_network(config: &Config, network: Network) -> Self {
        Self::new(config, network)
    }

    pub(crate) fn process(
        &self,
        inputs: Array1<f32>,
    ) -> (Option<Direction>, [f32; CREATURE_MEMORY_SIZE]) {
        let response = self.nn.forward(inputs);
        let mut max_r = 0.0;
        let mut max_r_index = 0usize;
        for i in 0..9usize {
            if response[i] > max_r {
                max_r = response[i];
                max_r_index = i;
            }
        }

        let new_memory = if CREATURE_MEMORY_SIZE > 0 {
            let mut new_memory = [0.0f32; CREATURE_MEMORY_SIZE];
            for i in 9..OUTPUTS_SIZE {
                new_memory[i - 9] = response[i];
            }
            new_memory
        } else {
            [0.0f32; CREATURE_MEMORY_SIZE]
        };

        if max_r_index == 0 {
            (None, new_memory)
        } else {
            (Some(Direction::from(max_r_index as u8 - 1)), new_memory)
        }
    }

    pub(crate) fn not_moved(&self) {
        self.time_since_move.set(self.time_since_move.get() + 1);
        if self.time_since_move.get() > 25 {
            self.add_fitness(-IDLE_PENALTY);
        }
    }
}

impl Brain {
    fn new(config: &Config, nn: Network) -> Self {
        Self {
            nn,
            fitness: Cell::new(config.start_fitness),
            time_since_move: Cell::new(0),
        }
    }

    fn topology(config: &Config) -> [usize; 4] {
        [
            INPUTS_SIZE,
            config.brain_neurons,
            config.brain_neurons,
            OUTPUTS_SIZE,
        ]
    }
}
