use std::fmt;
use std::fmt::Formatter;
use rand::{thread_rng, Rng};
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

pub struct GAOptimizer {
    pub genes: i32,
    pub gene_domains: Vec<Domain>,
    pub generations: i32,
    pub population_size: i32,
    pub mutation_rate: f64,
    pub fitness_function: fn(&Vec<f64>) -> f64,
    pub best_solution_fitness: f64,
    pub best_solution: Vec<f64>,
    population_matrix: Vec<Vec<f64>>,
    rng: ThreadRng,
}
pub enum DomainCategory {
    Reals,
    Integers,
    DiscreteSet,
}
pub struct Domain {
    pub category: DomainCategory,
    pub values: Vec<f64>,
}

const REAL_MIN: f64 = -1e100;
const REAL_MAX: f64 = 1e100;

impl Domain {
    pub fn new(category: DomainCategory, values: Option<Vec<f64>>) -> Domain {
        let domain_values: Vec<f64> = match (&category, values) {
            (DomainCategory::Reals | DomainCategory::Integers | DomainCategory::DiscreteSet, Some(v)) => { v }
            (DomainCategory::Reals, None) => { vec![REAL_MIN, REAL_MAX] }
            (DomainCategory::Integers, None) => { vec![i64::MIN as f64, i64::MAX as f64] }
            (DomainCategory::DiscreteSet, None) => { panic!("Domain category DiscreteSet was specified, but no set of values was provided.") }
        };

        Domain {
            category,
            values: domain_values,
        }
    }
}

impl fmt::Display for Domain {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let domain_category = match self.category {
            DomainCategory::Reals => { "Reals" }
            DomainCategory::Integers => { "Integers" }
            DomainCategory::DiscreteSet => { "Discrete Set" }
        };

        write!(f, "({}, {:?})", domain_category, self.values)
    }
}

impl GAOptimizer {
    pub fn new(genes: i32, gene_domains: Vec<Domain>, generations: i32, population_size: i32,
               mutation_rate: f64, fitness_function: fn(&Vec<f64>) -> f64) -> GAOptimizer {
        GAOptimizer {
            genes,
            gene_domains,
            generations,
            population_size,
            mutation_rate,
            population_matrix: vec![],
            rng: thread_rng(),
            fitness_function,
            best_solution_fitness: f64::MIN,
            best_solution: vec![],
        }
    }

    fn extract_gene_value(&mut self, gene_index: usize) -> f64 {
        match self.gene_domains[gene_index].category {
            DomainCategory::Reals => {
                let left = self.gene_domains[gene_index].values[0];
                let right = self.gene_domains[gene_index].values[1];
                self.rng.gen_range(left..right)
            }
            DomainCategory::Integers => {
                let left = self.gene_domains[gene_index].values[0] as i64;
                let right = self.gene_domains[gene_index].values[1] as i64;
                self.rng.gen_range(left..right) as f64
            }
            DomainCategory::DiscreteSet => {
                let left = 0;
                let right = self.gene_domains[gene_index].values.len();
                let value_index = self.rng.gen_range(left..right);
                self.gene_domains[gene_index].values[value_index]
            }
        }
    }

    fn initialize_population(&mut self) {
        self.population_matrix = (0..self.population_size as usize).map(|_| {
            (0..self.genes as usize)
                .map(|gene_index| self.extract_gene_value(gene_index))
                .collect::<Vec<f64>>()
        })
            .collect();
    }

    fn compute_population_fitness(&mut self, fitness_values: &mut Vec<f64>) {
        for (solution_index, solution) in self.population_matrix.iter().enumerate() {
            fitness_values[solution_index] = (self.fitness_function)(solution);

            //update best solution
            if fitness_values[solution_index] > self.best_solution_fitness {
                self.best_solution_fitness = fitness_values[solution_index];
                self.best_solution = solution.to_vec(); //TODO check if there is a different way to assign
                println!("New best: {}", self.best_solution_fitness);
            }
        }


        // rescale to have only non-negative fitness values
        let min_fitness = *fitness_values.iter().min_by(|a, b| a.total_cmp(b)).unwrap();
        for fitness_index in 0..self.population_size as usize {
            fitness_values[fitness_index] = fitness_values[fitness_index] - min_fitness;
        }
    }

    fn single_point_crossover(&mut self, parent1: Vec<f64>, parent2: Vec<f64>) -> (Vec<f64>, Vec<f64>) {
        let split_point = self.rng.gen_range(1..parent1.len() - 1);
        let offspring1 = [parent1[0..split_point].iter().as_slice(), parent2[split_point..].iter().as_slice()].concat();
        let offspring2 = [parent2[0..split_point].iter().as_slice(), parent1[split_point..].iter().as_slice()].concat();

        (offspring1, offspring2)
    }

    fn mutate(&mut self, chromosome: &mut Vec<f64>) {
        let mut genes_indices: Vec<usize> = (0..self.genes as usize).into_iter().collect();
        genes_indices.shuffle(&mut self.rng);

        let genes_to_mutate = ((chromosome.len() as f64) * self.mutation_rate) as usize;
        let selected_genes = &genes_indices[0..genes_to_mutate];

        for gene in selected_genes {
            chromosome[*gene] = self.extract_gene_value(*gene);
        }
    }

    fn roulette_wheel_selection(&mut self, population_fitness: &mut Vec<f64>) {
        // build the roulette wheel
        // TODO fix the issue arising when all individuals are equal (wheel_probabilities_cdf becomes a vector of NaNs)
        let cumulative_population_fitness: f64 = population_fitness.iter().sum();
        let mut wheel_probabilities_cdf = vec![];
        wheel_probabilities_cdf.push(population_fitness[0] / cumulative_population_fitness);
        for i in 1..population_fitness.len() {
            wheel_probabilities_cdf.push(wheel_probabilities_cdf[i - 1] + population_fitness[i] / cumulative_population_fitness);
        }

        // now use it to select mating individuals
        let mut new_population = vec![];

        while new_population.len() < self.population_size as usize {
            let parent1_probability = self.rng.gen_range(0. ..1.);
            let parent2_probability = self.rng.gen_range(0. ..1.);

            // find the parent individuals according to their probability
            let parent1_index = wheel_probabilities_cdf.iter().position(|p| p >= &parent1_probability);
            let parent2_index = wheel_probabilities_cdf.iter().position(|p| p >= &parent2_probability);

            let parent1 = match parent1_index {
                Some(i) => self.population_matrix[i].clone(),
                None => { panic!("Could not compute parent 1 index.\nParent 1 probability: {}\nRoulette wheel cdf: {:?}\nPopulation: {:?}", parent1_probability, wheel_probabilities_cdf, self.population_matrix) }
            };
            let parent2 = match parent2_index {
                Some(i) => self.population_matrix[i].clone(),
                None => { panic!("Could not compute parent 2 index.\nParent 2 probability: {}\nRoulette wheel cdf: {:?}\nPopulation: {:?}", parent2_probability, wheel_probabilities_cdf, self.population_matrix) }
            };

            let (mut offspring1, mut offspring2) = self.single_point_crossover(parent1, parent2);

            self.mutate(&mut offspring1);
            self.mutate(&mut offspring2);

            new_population.push(offspring1);
            new_population.push(offspring2);
        }

        self.population_matrix = new_population;
    }

    pub fn run(&mut self) {
        self.initialize_population();
        let mut fitness_values = vec![0.; self.population_size as usize];
        for generation in 0..self.generations {
            self.compute_population_fitness(&mut fitness_values);
            self.roulette_wheel_selection(&mut fitness_values);
        }
    }
}