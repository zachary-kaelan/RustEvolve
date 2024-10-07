use crate::*;
use ndarray::{Array, Array1, Array2, ShapeBuilder};
use std::cmp::max;
use std::ops::Add;

#[derive(Copy, Clone, Debug)]
pub enum LayerType {
    Input,
    Calc,
    Output,
}

#[derive(Clone, Debug)]
pub struct Layer {
    pub layer_type: LayerType,
    pub weights: Array2<f32>,
    pub biases: Array1<f32>,
}

impl Layer {
    pub fn new(layer_type: LayerType, weights: Array2<f32>, biases: Array1<f32>) -> Self {
        Self {
            layer_type,
            weights,
            biases,
        }
    }

    pub fn random(
        layer_type: LayerType,
        input_size: usize,
        output_size: usize,
        rng: &mut dyn RngCore,
    ) -> Self {
        let weights =
            Array2::from_shape_simple_fn((input_size, output_size), || rng.gen_range(-1.0..=1.0));
        let biases = Array1::from_shape_simple_fn(output_size, || rng.gen_range(-1.0..=1.0));
        Self {
            layer_type,
            weights,
            biases,
        }
    }

    pub fn forward(&self, inputs: Array1<f32>) -> Array1<f32> {
        inputs
            .dot(&self.weights)
            .add(&self.biases)
            .map(|x| x.max(0.0f32))
    }
}
