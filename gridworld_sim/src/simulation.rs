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
    world: RefCell<World<Weak<Brain>>>,
    population: Box<RouletteWheelPopulation<Rc<Brain>, Config>>,
    age: Cell<usize>,
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
            world: RefCell::new(world),
            population,
            age: Cell::new(0),
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
        world.spread_random_tile(Tiles::EnergyGain, config.world_foods, rng);
        world.spread_random_tile(Tiles::EnergyLoss, config.world_lava, rng);

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

    pub fn world(&self) -> Ref<World<Weak<Brain>>> {
        self.world.borrow()
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) -> Option<Statistics> {
        Simulation::process_brains(self.world());
        let consumed_foods = Simulation::process_collisions(self.world());
        self.process_movements();
        let stats = self.try_evolving(rng);

        let mut world = self.world.borrow_mut();
        for food in &consumed_foods {
            world.set(*food, Tiles::EnergyLoss);
        }
        world.spread_random_tile(Tiles::EnergyGain, consumed_foods.len(), rng);

        stats
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
    fn process_movements(&self) -> usize {
        let world = self.world.borrow();
        let mut num_movements = 0usize;
        for creature in &world.creatures {
            let brain_ref = creature.brain.upgrade().unwrap();
            if let Some(target_pos) = creature.target_pos.get() {
                creature.pos.set(target_pos);
                creature.target_pos.set(None);
                brain_ref.add_fitness(-MOVE_COST);
                brain_ref.time_since_move.set(0);
                num_movements += 1;
            } else {
                brain_ref.not_moved();
            }
        }
        //if num_movements == 0 {
        //    self.age.set(self.config.sim_generation_length);
        //}
        num_movements
    }

    fn process_collisions(world: Ref<World<Weak<Brain>>>) -> Vec<GridPoint> {
        let mut valid_actions = vec![];
        let mut food_consumed = vec![];
        let mut target_points: HashSet<GridPoint> = HashSet::new();
        let mut occupied_points: HashSet<GridPoint> = HashSet::new();
        for creature in &world.creatures {
            occupied_points.insert(creature.pos.get());
            target_points.insert(creature.target_pos.get().unwrap_or(creature.pos.get()));
        }

        for target_point in target_points {
            let mut creatures = vec![];
            for creature in &world.creatures {
                if creature.target_pos.get().unwrap_or(creature.pos.get()) == target_point {
                    creatures.push(creature);
                }
            }

            let tile = world[target_point];

            if creatures.len() > 1 {
                for creature in &creatures {
                    let brain_ref = creature.brain.upgrade().unwrap();
                    brain_ref.add_fitness(-BUMP_ENERGY_LOSS);
                    creature.target_pos.set(None);
                }

                match tile {
                    Tiles::EnergyGain => {
                        let energy_per_creature =
                            gridworld::FOOD_ENERGY_GAIN / (creatures.len() + 1) as f32;
                        for creature in creatures {
                            let brain_ref = creature.brain.upgrade().unwrap();
                            brain_ref.add_fitness(energy_per_creature);
                        }
                    }
                    Tiles::EnergyLoss => {
                        let energy_per_creature =
                            gridworld::HAZARD_ENERGY_LOSS / (creatures.len()) as f32;
                        for creature in creatures {
                            let brain_ref = creature.brain.upgrade().unwrap();
                            brain_ref.add_fitness(-energy_per_creature);
                        }
                    }
                    Tiles::Wall => {
                        for creature in creatures {
                            let brain_ref = creature.brain.upgrade().unwrap();
                            brain_ref.add_fitness(-WALL_ENERGY_LOSS);
                        }
                    }
                    Tiles::Floor => {}
                }
            } else {
                let creature = creatures[0];
                let brain_ref = creature.brain.upgrade().unwrap();

                if creature.target_pos.get().is_some() && occupied_points.contains(&target_point) {
                    brain_ref.add_fitness(-BUMP_ENERGY_LOSS * 2.0);
                    creature.target_pos.set(None);
                } else {
                    match tile {
                        Tiles::EnergyGain => {
                            brain_ref.add_fitness(FOOD_ENERGY_GAIN);
                            valid_actions.push((creature, target_point));
                            food_consumed.push(target_point);
                        }
                        Tiles::EnergyLoss => {
                            brain_ref.add_fitness(-HAZARD_ENERGY_LOSS);
                            valid_actions.push((creature, target_point));
                        }
                        Tiles::Wall => {
                            brain_ref.add_fitness(-WALL_ENERGY_LOSS);
                            creature.target_pos.set(None);
                        }
                        Tiles::Floor => {
                            valid_actions.push((creature, target_point));
                        }
                    }
                }
            }
        }

        food_consumed
    }

    fn process_brains(world: Ref<World<Weak<Brain>>>) {
        for creature in &world.creatures {
            let inputs = creature.form_brain_inputs(&world);
            if let Some(brain) = creature.brain.upgrade() {
                let (direction, new_memory) = brain.process(arr1(&inputs));

                //if let Some(direction) = direction {
                //    actions.push((creature, creature.pos.get().move_dir(direction)));
                //} else {
                //    actions.push((creature, creature.pos.get()));
                //}

                if CREATURE_MEMORY_SIZE > 0 {
                    creature.memory.set(new_memory);
                }
                if let Some(direction) = direction {
                    creature
                        .target_pos
                        .set(Some(creature.pos.get().move_dir(direction)));
                } else {
                    creature.target_pos.set(None);
                }
            } else {
                panic!("NO BRAIN")
            }
        }
    }
}

impl Simulation {
    fn try_evolving(&mut self, rng: &mut dyn RngCore) -> Option<Statistics> {
        self.age.set(self.age.get() + 1);

        if self.age.get() > self.config.sim_generation_length {
            Some(self.evolve(rng))
        } else {
            None
        }
    }

    fn evolve(&mut self, rng: &mut dyn RngCore) -> Statistics {
        self.age.set(0);
        self.generation += 1;

        let (individuals, statistics) = self.population.evolve(self.config(), rng);
        self.population = individuals;
        *self.world.borrow_mut() =
            Simulation::init_world(&self.config, &self.population.population, rng);

        Statistics {
            generation: self.generation - 1,
            ga: statistics,
        }
    }
}
