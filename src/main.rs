extern crate core;

use crate::ga::{Domain, DomainCategory, GAOptimizer};

mod ga;

fn my_function() {
    // println!("HA!");
}

fn fitness(solution: &Vec<f64>) -> f64 {
    // println!("{:?}", solution);
    my_function();
    solution.iter().sum()
}

fn main() {
    let mut domains = vec![];
    let genes = 25;
    for _ in 0..genes {
        domains.push(Domain::new(DomainCategory::Integers, Some(vec![-3., 0.])));
    }

    let mut ga_optimizer = GAOptimizer::new(genes, domains, 10000, 500, 0.2, fitness);

    // for domain in &ga_optimizer.gene_domains {
    //     println!("{}", domain)
    // }

    ga_optimizer.run();
    println!("{:?}", ga_optimizer.best_solution);
}
