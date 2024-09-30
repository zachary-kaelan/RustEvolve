use crate::config::Config;
use crate::stats::Statistics;
use crate::{Animal, Brain, World};
use genetic_algorithm::{Population, RouletteWheelPopulation};
use rand::rngs::OsRng;
use rand::{Rng, RngCore};
use std::cell::Cell;
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
        for animal in &mut self.world.animals {
            for food in &mut self.world.foods {
                let distance = nalgebra::distance(&animal.position, &food.position);

                if distance <= self.config.food_size {
                    animal.satiation += 1;
                    animal
                        .brain
                        .upgrade()
                        .unwrap()
                        .fitness
                        .set(animal.satiation as f32);
                    food.position = rng.gen();
                }
            }
        }
    }

    fn process_brains(&mut self) {
        for animal in &mut self.world.animals {
            animal.process_brain(&self.config, &self.world.foods);
        }
    }

    fn process_movements(&mut self) {
        for animal in &mut self.world.animals {
            animal.process_movement();
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
