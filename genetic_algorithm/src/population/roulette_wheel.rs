use crate::*;
use rand::seq::SliceRandom;
use rand::RngCore;

pub struct RouletteWheelPopulation<I, C>
where
    I: Individual<C>,
    C: Clone,
{
    pub population: Vec<I>,
    config: C,
}

impl<I, C> Population<I, C> for RouletteWheelPopulation<I, C>
where
    I: Individual<C>,
    C: Clone,
{
    fn new(individuals: Vec<I>, config: &C) -> Box<Self> {
        Box::new(Self {
            population: Vec::from(individuals),
            config: config.clone(),
        })
    }

    fn get_config(&self) -> &C {
        &self.config
    }

    fn size(&self) -> usize {
        self.population.len()
    }

    fn get_population(&self) -> &Vec<I> {
        &self.population
    }

    fn random(size: usize, config: &C, rng: &mut dyn RngCore) -> Self {
        let population: Vec<I> = (0..size).map(|_x| I::random(config, rng)).collect();
        Self {
            population,
            config: config.clone(),
        }
    }

    fn select(&self, config: &C, rng: &mut dyn RngCore) -> &I {
        self.population
            .choose_weighted(rng, |individual| individual.fitness().max(0.00001))
            .expect("empty population")
    }
}
