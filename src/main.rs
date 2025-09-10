// src/main.rs
use std::env;

#[macro_use]
pub mod define_object;
#[macro_use]
pub mod define_class;
pub mod experiments;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide an experiment name to run.");
        println!("Available experiments: inheritance_simulation, macro_inheritance");
        return;
    }

    let experiment_name = &args[1];

    match experiment_name.as_str() {
        "inheritance_simulation" => {
            experiments::inheritance_simulation::run_experiment();
        }
        "macro_inheritance" => {
            experiments::macro_inheritance::run_experiment();
        }
        _ => {
            println!("Experiment '{}' not found.", experiment_name);
            println!("Available experiments: inheritance_simulation, macro_inheritance");
        }
    }
}
