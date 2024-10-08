use crate::stats::Statistics;
pub use crate::*;
use ndarray::Array1;
use rand::rngs::OsRng;
use rand::{random, RngCore};
use std::async_iter::from_iter;
use std::collections::HashSet;
use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};

pub struct Simulation {
    config: Config,
    world: World<Weak<Brain>>,
    population: Box<RouletteWheelPopulation<Rc<Brain>, Config>>,
    age: usize,
    pub generation: usize,
    pub rng: OsRng,
}

impl Simulation {
    pub fn random(config: &Config) -> Self {
        let rng = &mut OsRng;
        let mut world: World<Weak<Brain>> = World::default();
        world.set_border(Tiles::Wall);
        world.place_random_food(config.world_foods, rng);

        let population = Box::new(RouletteWheelPopulation::random(
            config.world_creatures,
            config,
            rng,
        ));
        for individual in &population.population {
            let pos = world.get_random_free_pt(rng);
            let creature = Creature::new(pos, Rc::downgrade(individual));
            world.creatures.push(creature);
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

    pub fn world(&self) -> &World<Weak<Brain>> {
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
    fn process_collisions(&self, actions: Vec<(&Creature<Weak<Brain>>, GridPoint)>) {
        let mut target_points: HashSet<GridPoint> = HashSet::new();
        for (_, new_pos) in &actions {
            target_points.insert(*new_pos);
        }
        for target_point in target_points {
            let mut creatures = vec![];
            for (creature, new_pos) in &actions {
                if *new_pos == target_point {
                    creatures.push(*creature);
                }
            }

            let tile = self.world[target_point];

            if creatures.len() > 1 {
                for creature in &creatures {
                    let brain_ref = creature.brain.upgrade().unwrap();
                    brain_ref.add_fitness(-BUMP_ENERGY_LOSS);
                }
                match tile {
                    Tiles::EnergyGain => {
                        let energy_per_creature =
                            gridworld::FOOD_ENERGY_GAIN / (creatures.len() + 1) as f32;
                        for creature in &creatures {
                            let brain_ref = creature.brain.upgrade().unwrap();
                            brain_ref.add_fitness(energy_per_creature);
                        }
                    }
                    Tiles::EnergyLoss => {
                        let energy_per_creature =
                            gridworld::HAZARD_ENERGY_LOSS / (creatures.len() + 1) as f32;
                        for creature in &creatures {
                            let brain_ref = creature.brain.upgrade().unwrap();
                            brain_ref.add_fitness(-energy_per_creature);
                        }
                    }
                    Tiles::Wall => {
                        for creature in &creatures {
                            let brain_ref = creature.brain.upgrade().unwrap();
                            brain_ref.add_fitness(-WALL_ENERGY_LOSS);
                        }
                    }
                    Tiles::Floor => {}
                }
            } else {
                let mut creature = creatures[0];
                let brain_ref = creature.brain.upgrade().unwrap();
                match tile {
                    Tiles::EnergyGain => {
                        brain_ref.add_fitness(FOOD_ENERGY_GAIN);
                        creature.set_pos(target_point);
                    }
                    Tiles::EnergyLoss => {
                        brain_ref.add_fitness(-HAZARD_ENERGY_LOSS);
                        creature.set_pos(target_point);
                    }
                    Tiles::Wall => {
                        brain_ref.add_fitness(-WALL_ENERGY_LOSS);
                    }
                    Tiles::Floor => {
                        creature.set_pos(target_point);
                    }
                }
            }
        }
    }

    fn process_brains(&self) -> Vec<(&Creature<Weak<Brain>>, GridPoint)> {
        let mut actions = vec![];
        for creature in &self.world.creatures {
            let inputs = creature.form_brain_inputs(&self.world);
            let brain = creature.brain.upgrade().unwrap().deref();
            let (direction, new_memory) = brain.process(Array1::from(inputs));

            if let Some(direction) = direction {
                actions.push((creature, creature.pos.move_dir(direction)));
            } else {
                actions.push((creature, creature.pos));
            }

            creature.memory.set(new_memory);
        }

        actions
    }
}
