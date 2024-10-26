extern crate core;

use crate::ga::{Domain, DomainCategory, GAOptimizer};

mod ga;

fn my_function() {
    println!("HA!");
}

fn fitness(solution: &Vec<f64>) -> f64 {
    println!("{:?}", solution);
    my_function();
    solution.iter().sum()
}

fn main() {
    let mut domains = vec![];
    for _ in 0..200 {
        domains.push(Domain::new(DomainCategory::Reals, None));
    }

    let mut ga_optimizer = GAOptimizer::new(3, domains, 10, 5, fitness);

    // for domain in &ga_optimizer.gene_domains {
    //     println!("{}", domain)
    // }

    ga_optimizer.run();
}
