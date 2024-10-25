extern crate core;

use crate::ga::{Domain, DomainCategory, GAOptimizer};

mod ga;

fn main() {
    let mut domains = vec![];
    for _ in 0..500000 {
        domains.push(Domain::new(DomainCategory::Integers, None));
    }

    let mut ga_optimizer = GAOptimizer::new(500000, domains, 0, 40000);

    // for domain in &ga_optimizer.gene_domains {
    //     println!("{}", domain)
    // }

    ga_optimizer.initialize_population();
}
