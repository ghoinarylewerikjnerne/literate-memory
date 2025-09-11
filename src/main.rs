// src/main.rs
use std::env;

#[macro_use]
pub mod define_object;
#[macro_use]
pub mod define_class;
#[macro_use]
pub mod macros;
pub mod experiments;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide an experiment name to run.");
        println!("Available experiments: inheritance_simulation, macro_inheritance, complete_class_experiment");
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
        "complete_class_experiment" => {
            experiments::complete_class_experiment::run_experiment();
        }
        _ => {
            println!("Experiment '{}' not found.", experiment_name);
            println!("Available experiments: inheritance_simulation, macro_inheritance, complete_class_experiment");
        }
    }
}
