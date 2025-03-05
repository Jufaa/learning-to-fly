use rand::{Rng, RngCore};
use rand::prelude::IndexedRandom;
pub struct GeneticAlgorithm;

pub trait Individual{
    fn fitness(&self) -> f32;
}
impl GeneticAlgorithm {
    pub fn evolve<I>(&self, population: &[I]) -> Vec<I>
    where
        I: Individual,        
    {
        assert!(!population.is_empty());
        todo!()
    }
}


pub trait SelectionMethod {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual;
}pub struct RouletteWheelSelection;

impl SelectionMethod for RouletteWheelSelection {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual,
    {
        let total_fitness: f32 = population
        .iter()
        .map(|individual| individual.fitness())
        .sum();

        loop{
            let indiv = population
            .choose(rng)
            .expect("got an empty population");

            let indiv_share = indiv.fitness() / total_fitness;

            if rng.gen_bool(indiv_share as f64){
                return indiv;
            }
        }
    }
}

