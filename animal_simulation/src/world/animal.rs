use crate::config::Config;
use crate::*;
use ndarray::Array1;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct Animal {
    pub(crate) position: na::Point2<f32>,
    pub(crate) rotation: na::Rotation2<f32>,
    pub(crate) vision: Vec<f32>,
    pub(crate) speed: f32,
    pub(crate) eye: Eye,
    pub(crate) brain: Weak<Brain>,
    pub satiation: usize,
}

impl Animal {
    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }

    pub fn rotation(&self) -> na::Rotation2<f32> {
        self.rotation
    }

    pub fn vision(&self) -> &[f32] {
        &self.vision
    }
}

impl Animal {
    pub(crate) fn random(config: &Config, brain: &Rc<Brain>, rng: &mut dyn RngCore) -> Self {
        //let brain = Brain::random(config, rng);

        Self::new(config, brain, rng)
    }

    pub(crate) fn from_brain(config: &Config, brain: &Rc<Brain>, rng: &mut dyn RngCore) -> Self {
        Self::new(config, brain, rng)
    }

    pub(crate) fn brain(&self) -> Weak<Brain> {
        Weak::clone(&self.brain)
    }

    pub(crate) fn process_brain(&mut self, config: &Config, foods: &[Food]) {
        self.vision = self.eye.process_vision(self.position, self.rotation, foods);

        let (speed, rotation) = self
            .brain
            .upgrade()
            .unwrap()
            .process(Array1::from_vec(self.vision.clone()));

        self.speed = (self.speed + speed).clamp(config.sim_speed_min, config.sim_speed_max);
        self.rotation = na::Rotation2::new(self.rotation.angle() + rotation);
    }

    pub(crate) fn process_movement(&mut self) {
        self.position += self.rotation * na::Vector2::new(0.0, self.speed);
        self.position.x = na::wrap(self.position.x, 0.0, 1.0);
        self.position.y = na::wrap(self.position.y, 0.0, 1.0);
    }
}

impl Animal {
    fn new(config: &Config, brain: &Rc<Brain>, rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),
            rotation: rng.gen(),
            vision: vec![0.0; config.eye_cells],
            speed: config.sim_speed_max,
            eye: Eye::new(config),
            brain: Rc::downgrade(&Rc::clone(brain)),
            satiation: 0,
        }
    }
}
