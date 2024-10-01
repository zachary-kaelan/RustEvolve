use crate::config::Config;
use crate::stats::Statistics;
use crate::{Animal, Brain, VisibleAnimal, World};
use genetic_algorithm::{Population, RouletteWheelPopulation};
use rand::rngs::OsRng;
use rand::{Rng, RngCore};
use std::cell::Cell;
use std::f32::consts::PI;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

pub struct Simulation {
    config: Config,
    world: World,
    population: Box<RouletteWheelPopulation<Rc<Brain>, Config>>,
    age: usize,
    pub generation: usize,
    pub rng: OsRng,
}

impl Simulation {
    pub fn random(config: &Config) -> Self {
        let rng = &mut OsRng;
        let mut world = World::random(config, rng);
        let population = Box::new(RouletteWheelPopulation::random(
            config.world_animals,
            config,
            rng,
        ));
        for individual in &population.population {
            world
                .animals
                .push(Animal::from_brain(config, individual, rng))
        }
        Self {
            config: config.clone(),
            world,
            population,
            age: 0,
            generation: 0,
            rng: *rng,
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) -> Option<Statistics> {
        self.process_collisions(rng);
        self.process_brains();
        self.process_movements();
        self.try_evolving(rng)
    }

    pub fn train(&mut self, rng: &mut dyn RngCore) -> Statistics {
        loop {
            if let Some(statistics) = self.step(rng) {
                return statistics;
            }
        }
    }
}

impl Simulation {
    fn process_collisions(&mut self, rng: &mut dyn RngCore) {
        let visible_animals: Vec<VisibleAnimal> =
            self.world.animals.iter().map(Animal::visible).collect();
        for animal in &mut self.world.animals {
            for food in &mut self.world.foods {
                let distance = nalgebra::distance(&animal.position, &food.position);

                if distance <= self.config.food_size {
                    animal.satiation += 1;
                    animal.brain.upgrade().unwrap().fitness.set(
                        animal.satiation as f32 - animal.boosts as f32 * self.config.boost_cost,
                    );
                    food.position = rng.gen();
                }
            }

            for other_animal in &visible_animals {
                let distance = nalgebra::distance(&animal.position, &other_animal.0);
                let relative_angle = animal.rotation().angle_to(&other_animal.1);
                if distance > 0.0000001
                    && distance < self.config.animal_size
                    && relative_angle > PI / 12f32
                {
                    let stunned = animal.stunned > 0;
                    let boosting = animal.boosting;
                    let other_stunned = other_animal.2 < 0.0000001;
                    let other_boosting = other_animal.2 > self.config.sim_speed_max;

                    if !other_stunned {
                        if stunned {
                            if animal.satiation >= 3 && other_boosting {
                                animal.satiation -= 3;
                            }
                        } else {
                            let mut stun_multiplier = 1u8;
                            if boosting {
                                //stun_multiplier += 1;
                            }
                            if other_boosting {
                                stun_multiplier += 1;
                            }
                            animal.stun(self.config.stun_duration * stun_multiplier);
                        }
                    } else if !stunned && other_stunned && boosting && other_animal.3 >= 3 {
                        animal.satiation += 3;
                    }
                }
            }
        }
    }

    fn process_brains(&mut self) {
        let visible_animals: Vec<VisibleAnimal> =
            self.world.animals.iter().map(Animal::visible).collect();
        for animal in &mut self.world.animals {
            animal.process_brain(&self.config, &self.world.foods, &visible_animals, self.age);
        }
    }

    fn process_movements(&mut self) {
        let config = &self.config;
        for animal in &mut self.world.animals {
            animal.process_movement(config);
        }
    }

    fn try_evolving(&mut self, rng: &mut dyn RngCore) -> Option<Statistics> {
        self.age += 1;

        if self.age > self.config.sim_generation_length {
            Some(self.evolve(rng))
        } else {
            None
        }
    }

    fn evolve(&mut self, rng: &mut dyn RngCore) -> Statistics {
        self.age = 0;
        self.generation += 1;

        if self.config.ga_reverse == 1 {
            let max_satiation = self
                .world
                .animals
                .iter()
                .map(|animal| animal.satiation)
                .max()
                .unwrap_or_default();

            for animal in &mut self.world.animals {
                let brain = animal.brain().upgrade().unwrap();
                brain
                    .fitness
                    .set((max_satiation as f32) - brain.fitness.get());
            }
        }

        self.world.animals.clear();
        let (individuals, statistics) = self.population.evolve(self.config(), rng);
        self.population = individuals;
        for individual in &self.population.population {
            self.world
                .animals
                .push(Animal::from_brain(self.config(), individual, rng))
        }

        for food in &mut self.world.foods {
            food.position = rng.gen();
        }

        Statistics {
            generation: self.generation - 1,
            ga: statistics,
        }
    }
}
