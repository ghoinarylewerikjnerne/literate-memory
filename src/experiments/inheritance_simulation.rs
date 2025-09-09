// experiments/inheritance_simulation.rs

// Base struct
#[repr(C)]
pub struct Animal {
    pub name: &'static str,
}

impl Animal {
    pub fn make_sound(&self) {
        println!("{} makes a sound.", self.name);
    }
}

// "Derived" struct
#[repr(C)]
pub struct Dog {
    pub base: Animal,
    pub breed: &'static str,
}

impl Dog {
    pub fn wag_tail(&self) {
        println!("The {} dog wags its tail.", self.breed);
    }
}

pub fn run_experiment() {
    println!("--- Running Inheritance Simulation Experiment ---");
    let dog = Dog {
        base: Animal { name: "Generic Animal" },
        breed: "Golden Retriever",
    };

    dog.base.make_sound(); // This is the "safe" way via composition
    dog.wag_tail();

    println!("\nNow for the 'unsafe' part to simulate inheritance:");

    // Here we "break the rules". We treat a `&Dog` as if it were an `&Animal`.
    // This only works because `Dog` has `Animal` as its first field and we use `#[repr(C)]`.
    let animal_ref: &Animal = unsafe {
        // Cast the dog pointer to an animal pointer.
        &*( (&dog as *const Dog) as *const Animal )
    };

    // Now we can call Animal's methods on what is actually a Dog.
    animal_ref.make_sound();

    // We can't call dog.wag_tail() through animal_ref, demonstrating a limitation.
    // animal_ref.wag_tail(); // This would not compile.

    println!("--- Experiment Finished ---");
}
