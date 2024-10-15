use crate::stats::Statistics;
pub use crate::*;
use ::rand::rngs::OsRng;
use ::rand::RngCore;
use ndarray::{arr1, Array1};
use std::collections::HashSet;
use std::ops::Deref;
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

        let population = Box::new(RouletteWheelPopulation::random(
            config.world_creatures,
            config,
            rng,
        ));
        let world = Simulation::init_world(config, &population.population, rng);
        Self {
            config: config.clone(),
            world,
            population,
            age: 0,
            generation: 0,
            rng: *rng,
        }
    }

    fn init_world(
        config: &Config,
        population: &[Rc<Brain>],
        rng: &mut dyn RngCore,
    ) -> World<Weak<Brain>> {
        let mut world: World<Weak<Brain>> = World::default();

        world.set_border(Tiles::Wall);
        world.place_random_food(config.world_foods, rng);

        for individual in population {
            let pos = world.get_random_free_pt(rng);
            let creature = Creature::new(pos, Rc::downgrade(individual));
            world.creatures.push(creature);
        }

        world
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn world(&self) -> &World<Weak<Brain>> {
        &self.world
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) -> Option<Statistics> {
        let actions = self.process_brains();
        let valid_actions = self.process_collisions(actions);
        self.process_movements(valid_actions);
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
    fn process_movements(&self, actions: Vec<(&Creature<Weak<Brain>>, GridPoint)>) {
        for (creature, action) in actions {
            creature.pos.set(action);
        }
    }

    fn process_collisions<'a>(
        &self,
        actions: Vec<(&'a Creature<Weak<Brain>>, GridPoint)>,
    ) -> Vec<(&'a Creature<Weak<Brain>>, GridPoint)> {
        let mut valid_actions = vec![];
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
                let creature = creatures[0];
                let brain_ref = creature.brain.upgrade().unwrap();
                match tile {
                    Tiles::EnergyGain => {
                        brain_ref.add_fitness(FOOD_ENERGY_GAIN);
                        valid_actions.push((creature, target_point));
                    }
                    Tiles::EnergyLoss => {
                        brain_ref.add_fitness(-HAZARD_ENERGY_LOSS);
                        valid_actions.push((creature, target_point));
                    }
                    Tiles::Wall => {
                        brain_ref.add_fitness(-WALL_ENERGY_LOSS);
                    }
                    Tiles::Floor => {
                        valid_actions.push((creature, target_point));
                    }
                }
            }
        }

        valid_actions
    }

    fn process_brains(&self) -> Vec<(&Creature<Weak<Brain>>, GridPoint)> {
        let mut actions = vec![];
        for creature in &self.world.creatures {
            let inputs = creature.form_brain_inputs(&self.world);
            if let Some(brain) = creature.brain.upgrade() {
                let (direction, new_memory) = brain.process(arr1(&inputs));

                if let Some(direction) = direction {
                    actions.push((creature, creature.pos.get().move_dir(direction)));
                } else {
                    actions.push((creature, creature.pos.get()));
                }

                creature.memory.set(new_memory);
            } else {
                panic!("NO BRAIN")
            }
        }

        actions
    }
}

impl Simulation {
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

        let (individuals, statistics) = self.population.evolve(self.config(), rng);
        self.population = individuals;
        self.world = Simulation::init_world(&self.config, &self.population.population, rng);

        Statistics {
            generation: self.generation - 1,
            ga: statistics,
        }
    }
}
