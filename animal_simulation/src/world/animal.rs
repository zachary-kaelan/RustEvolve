use crate::config::Config;
use crate::*;
use ndarray::Array1;
use std::rc::{Rc, Weak};

pub struct VisibleAnimal(
    pub na::Point2<f32>,
    pub na::Rotation2<f32>,
    pub f32,
    pub usize,
);

#[derive(Debug)]
pub struct Animal {
    pub(crate) position: na::Point2<f32>,
    pub(crate) rotation: na::Rotation2<f32>,
    pub(crate) vision: Vec<f32>,
    pub(crate) speed: f32,
    pub(crate) eye: Eye,
    pub(crate) brain: Weak<Brain>,
    pub satiation: usize,
    pub boosts: usize,
    pub boosting: bool,
    pub stunned: u8,
    pub stunned_cooldown: u8,
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

    pub fn visible(&self) -> VisibleAnimal {
        VisibleAnimal(self.position, self.rotation, self.speed, self.satiation)
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

    pub(crate) fn process_brain(
        &mut self,
        config: &Config,
        foods: &[Food],
        animals: &[VisibleAnimal],
        age: usize,
    ) {
        if self.stunned > 0 {
            return;
        }

        self.vision = self
            .eye
            .process_vision(self.position, self.rotation, foods, animals);

        let mut inputs = self.vision.clone();
        inputs.push(self.satiation as f32);
        //inputs.push(age as f32 / config.sim_generation_length as f32);

        let (speed, rotation, boost) = self
            .brain
            .upgrade()
            .unwrap()
            .process(Array1::from_vec(inputs));

        self.speed = if boost <= 0.0 {
            (self.speed + speed).clamp(config.sim_speed_min, config.sim_speed_max)
        } else {
            config.sim_speed_max * 2.0
        };
        self.rotation = na::Rotation2::new(self.rotation.angle() + rotation);

        if boost > 0.0 {
            self.boosts += 1;
            self.boosting = true;
        } else {
            self.boosting = false;
        }
    }

    pub(crate) fn process_movement(&mut self, config: &Config) {
        if self.stunned == 0 {
            self.position += self.rotation * na::Vector2::new(0.0, self.speed);
            self.position.x = na::wrap(self.position.x, 0.0, 1.0);
            self.position.y = na::wrap(self.position.y, 0.0, 1.0);
            if self.stunned_cooldown > 0 {
                self.stunned_cooldown -= 1;
            }
        } else {
            self.stunned -= 1;
            if self.stunned == 0 {
                self.stunned_cooldown = config.stun_cooldown;
            }
        }
    }

    pub(crate) fn stun(&mut self, duration: u8) {
        if self.stunned == 0 && self.stunned_cooldown == 0 {
            self.stunned = duration;
            self.boosting = false;
            self.speed = 0.0;
            if self.satiation > 0 {
                //self.satiation -= 1;
            }
        }
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
            boosts: 0,
            boosting: false,
            stunned: 0,
            stunned_cooldown: 0,
        }
    }
}
