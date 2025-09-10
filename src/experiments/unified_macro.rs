// Experiment for the unified class-definition macro

// --- Import shared definitions from the macro_inheritance experiment ---
use crate::experiments::macro_inheritance::{
    CanineBehavior, FoodPreference, HasMood, Warg, Wolf,
};


// --- Refactored Concrete Class Definitions using the new macro ---

define_concrete_class! {
    data_struct WargData,
    state: {
        pub mood: &'static str,
    },
    impls: {
        CanineBehavior => {
            fn howl(&self) -> &str { "a menacing, deep howl" }
        },
        FoodPreference => {
            fn preferred_food(&self) -> &str { "meat" }
        },
        HasMood => {
            fn get_mood(&self) -> &str { self.mood }
        }
    }
}

define_concrete_class! {
    data_struct WolfData,
    state: {
        pub mood: &'static str,
    },
    impls: {
        CanineBehavior => {
            fn howl(&self) -> &str { "a lonely, high-pitched howl" }
        },
        FoodPreference => {
            fn preferred_food(&self) -> &str { "rabbits" }
        },
        HasMood => {
            fn get_mood(&self) -> &str { self.mood }
        }
    }
}

// --- Experiment Runner ---
pub fn run_experiment() {
    println!("--- Running Unified Macro Experiment ---");

    // Create instances with different, personal moods
    let warg1 = Warg::new(WargData { mood: "grumpy" });
    let warg2 = Warg::new(WargData { mood: "excited" });
    let wolf1 = Wolf::new(WolfData { mood: "lonely" });

    // Demonstrate inherited methods
    println!("The first warg lets out {}", warg1.howl());
    println!("The second warg {}", warg2.eat());
    println!("The wolf {}", wolf1.eat());

    println!("\n--- Checking Personal Moods ---");
    // Demonstrate instance-specific mood
    println!("The first warg {}", warg1.check_mood());
    println!("The second warg {}", warg2.check_mood());
    println!("The wolf {}", wolf1.check_mood());


    println!("\n--- Experiment Finished ---");
}
