use rand::RngCore;

pub trait Individual<C> {
    fn random(params: &C, rng: &mut dyn RngCore) -> Self;
    fn fitness(&self) -> f32;
    fn mutate(self, params: &C, rng: &mut dyn RngCore) -> Self;
    fn crossover(&self, other: &Self, params: &C, rng: &mut dyn RngCore) -> Self;
}
