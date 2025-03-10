use rand::{Rng, RngCore};
use rand::prelude::IndexedRandom;
use std::ops::Index;


// GENETIC ALGORITHM
pub struct GeneticAlgorithm<S>{
    selection_method: S,
    crossover_method: Box<dyn CrossoverMethod>,
    mutation_method: Box<dyn MutationMethod>,
}

pub trait Individual{
    fn create(chromosome: Chromosome) -> Self;
    fn fitness(&self) -> f32;
    fn chromosome(&self) -> &Chromosome;
}

impl<S> GeneticAlgorithm<S>
where
    S: SelectionMethod,
    {
    pub fn new(
        selection_method: S,
        crossover_method: impl CrossoverMethod + 'static,
        mutation_method: impl MutationMethod + 'static,
    ) -> Self {
        Self {
            selection_method,
            crossover_method: Box::new(crossover_method),
            mutation_method: Box::new(mutation_method),
        }
    }
    pub fn evolve<I>(&self, rng: &mut dyn RngCore,  population: &[I]) -> Vec<I>
    where
    I: Individual,
    {
        /* ... */
        
        (0..population.len())
        .map(|_| {
            //selection
            let parent_a = self.selection_method.select(rng, population).chromosome();
            let parent_b = self.selection_method.select(rng, population).chromosome();
            //crossover
            let mut child = self.crossover_method.crossover(rng, parent_a, parent_b);
            //mutation
            self.mutation_method.mutate(rng, &mut child);
            I::create(child)
        })
        .collect()
    }
}




// SELECTION
pub trait SelectionMethod {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where
    I: Individual;
}

pub struct RouletteWheelSelection;

impl SelectionMethod for RouletteWheelSelection {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where
    I: Individual,
    {
        population
        .choose_weighted(rng, |individual| individual.fitness())
        .expect("got an empty population")
    }
}

// CROSSOVER
pub struct Chromosome {
    genes: Vec<f32>,
}

impl Chromosome {
    pub fn len(&self) -> usize {
        self.genes.len()
    }
    
    pub fn iter(&self) -> impl Iterator<Item = &f32> {
        self.genes.iter()
    }
    
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut f32> {
        self.genes.iter_mut()
    }
}

impl Index<usize> for Chromosome {
    type Output = f32;
    
    fn index(&self, index: usize) -> &Self::Output {
        &self.genes[index]
    }
}

impl IntoIterator for Chromosome {
    type Item = f32;
    type IntoIter = std::vec::IntoIter<f32>;

    fn into_iter(self) -> Self::IntoIter {
        self.genes.into_iter()
    }
}


impl FromIterator<f32> for Chromosome {
    fn from_iter<T: IntoIterator<Item = f32>>(iter: T) -> Self {
        Self {
            genes: iter.into_iter().collect(),
        }
    }
}

pub trait CrossoverMethod {
    fn crossover(
        &self,
        rng: &mut dyn RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
    ) -> Chromosome;
}

pub struct UniformCrossover;

impl CrossoverMethod for UniformCrossover {
    fn crossover(
        &self,
        rng: &mut dyn RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
    ) -> Chromosome {
        assert_eq!(parent_a.len(), parent_b.len());

       
        parent_a
            .iter()
            .zip(parent_b.iter())
            .map(|(&a, &b)| if rng.gen_bool(0.5) { a } else { b })
            .collect()
    }
}

// MUTATION
#[derive(Clone, Debug)]
pub struct GaussianMutation {
    chance: f32,
    coeff: f32,
}

impl GaussianMutation {
    pub fn new(chance: f32, coeff: f32) -> Self {
        assert!(chance >= 0.0 && chance <= 1.0);

        Self { chance, coeff }
    }
}

pub trait MutationMethod {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome);
}
impl MutationMethod for GaussianMutation {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome) {
        for gene in child.iter_mut() {
            let sign = if rng.gen_bool(0.5) { -1.0 } else { 1.0 };

            if rng.gen_bool(self.chance as f64) {
                *gene += sign * self.coeff * rng.r#gen::<f32>();
            }
        }
    }
}