use crate::*;
use ndarray::Array2;

#[derive(Clone, Debug)]
pub struct Network {
    layers: Vec<Layer>,
}

impl Network {
    pub(crate) fn new(layers: Vec<Layer>) -> Self {
        Self { layers }
    }

    pub fn random(topology: &[usize], rng: &mut dyn RngCore) -> Self {
        let mut layers: Vec<Layer> = topology
            .windows(2)
            .map(|layers| Layer::random(LayerType::Calc, layers[0], layers[1], rng))
            .collect();
        layers[0].layer_type = LayerType::Input;
        layers[layers.len() - 1].layer_type = LayerType::Output;
        Self { layers }
    }

    pub fn forward(&self, inputs: Array2<f32>) -> Array2<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.forward(inputs))
    }
}
