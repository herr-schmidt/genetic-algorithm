use std::fmt;
use std::fmt::Formatter;
use rand::{thread_rng, Rng};
use rand::rngs::ThreadRng;

pub struct GAOptimizer {
    pub genes: i32,
    pub gene_domains: Vec<Domain>,
    pub generations: i32,
    pub population_size: i32,
    pub population_matrix: Option<Vec<Vec<f64>>>,
    pub rng: ThreadRng,
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
    pub fn new(genes: i32, gene_domains: Vec<Domain>, generations: i32, population_size: i32) -> GAOptimizer {
        GAOptimizer {
            genes,
            gene_domains,
            generations,
            population_size,
            population_matrix: None,
            rng: thread_rng(),
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

    pub fn initialize_population(&mut self) {
        let population_matrix: Vec<Vec<f64>> = vec![(0..self.genes as usize)
                                                        .map(|i| self.extract_gene_value(i))
                                                        .collect::<Vec<f64>>()
                                                    ; self.population_size as usize];
        self.population_matrix = Option::from(population_matrix);
    }
}