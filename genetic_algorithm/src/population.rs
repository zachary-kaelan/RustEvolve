mod roulette_wheel;

use crate::individual::Individual;
use crate::PopulationStatistics;
use rand::RngCore;

pub use roulette_wheel::*;

pub trait Population<I, C>
where
    I: Individual<C>,
    C: Clone,
{
    fn new(individuals: Vec<I>, config: &C) -> Box<Self>;
    fn get_config(&self) -> &C;
    fn size(&self) -> usize;
    fn random(size: usize, params: &C, rng: &mut dyn RngCore) -> Self;
    fn select(&self, params: &C, rng: &mut dyn RngCore) -> &I;
    fn get_population(&self) -> &Vec<I>;

    fn evolve(&self, params: &C, rng: &mut dyn RngCore) -> (Box<Self>, PopulationStatistics) {
        let mut max_similarity = 0.0f32;
        let new_population: Vec<I> = (0..self.size())
            .map(|_| {
                let parent_a = self.select(params, rng);
                let parent_b = self.select(params, rng);

                parent_a
                    .crossover(parent_b, params, rng)
                    .mutate(params, rng)
            })
            .collect();

        let stats = PopulationStatistics::new(self.get_population(), max_similarity);

        (Self::new(new_population, self.get_config()), stats)
    }
}
