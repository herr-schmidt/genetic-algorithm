extern crate core;

use crate::ga::{Domain, DomainCategory, GAOptimizer};

mod ga;

fn my_function() {
    // println!("HA!");
}

fn fitness(solution: &Vec<f64>) -> f64 {
    let mut solution_weight = 0.;
    let mut solution_profit = 0.;
    for (i, x) in solution.iter().enumerate() {
        solution_weight = solution_weight + x * WEIGHTS[i];
        solution_profit = solution_weight + x * PROFITS[i];
    }

    if solution_weight > WEIGHT_LIMIT {
        return - solution_weight
    }

    solution_profit
}

static WEIGHTS: [f64; 10] = [4., 3., 7., 2., 10., 1., 12., 2., 9., 17.];
static PROFITS: [f64; 10] = [12., 1., 6., 2., 7., 3., 5., 4., 8., 10.];
static WEIGHT_LIMIT: f64 = 100.;

fn main() {
    let mut domains = vec![];
    let genes = 10;
    for _ in 0..genes {
        domains.push(Domain::new(DomainCategory::Integers, Some(vec![0., 10.])));
    }

    let mut ga_optimizer = GAOptimizer::new(genes, domains, 500000, 10000, 0.4, fitness);

    // for domain in &ga_optimizer.gene_domains {
    //     println!("{}", domain)
    // }

    ga_optimizer.run();
    println!("{:?}", ga_optimizer.best_solution);
}
