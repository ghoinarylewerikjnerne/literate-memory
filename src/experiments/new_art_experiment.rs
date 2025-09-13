// The exhibition space for our new creation.
// Here we will use our macro and test its output.

// This macro call will expand to generate all the traits and structs for our hierarchy.
// We are using the improved version of the macro that correctly establishes
// the transitive relationship between constraints (e.g., DireWolf is a Dog).
define_artistic_hierarchy! {
    abstract Animal;
    class Dog extends Animal;
    class Wolf extends Dog;
    class Warg extends Dog;
    class DireWolf extends Wolf;
    class FerociousWarg extends Warg;
}

// The test function, adapted and improved from the blueprint.
// It now checks for the full, correct hierarchy.
#[allow(dead_code)]
fn test() {
    // Helper functions to test constraints. These act as our assertions.
    // If the code compiles, the assertion passes.
    fn is_animal_constraint<T: AnimalConstraint>(_: T) {}
    fn is_dog_constraint<T: DogConstraint>(_: T) {}
    fn is_wolf_constraint<T: WolfConstraint>(_: T) {}
    fn is_warg_constraint<T: WargConstraint>(_: T) {}
    fn is_direwolf_constraint<T: DireWolfConstraint>(_: T) {}
    fn is_ferocioswarg_constraint<T: FerociousWargConstraint>(_: T) {}

    // --- Positive Constraint Hierarchy Tests ---
    // These should all compile, proving the macro works correctly.

    // A Dog is an Animal.
    is_dog_constraint(DogData);
    is_animal_constraint(DogData);

    // A Wolf is a Dog and an Animal.
    is_wolf_constraint(WolfData);
    is_dog_constraint(WolfData);
    is_animal_constraint(WolfData);

    // A Warg is a Dog and an Animal.
    is_warg_constraint(WargData);
    is_dog_constraint(WargData);
    is_animal_constraint(WargData);

    // A DireWolf is a Wolf, a Dog, and an Animal.
    is_direwolf_constraint(DireWolfData);
    is_wolf_constraint(DireWolfData);
    is_dog_constraint(DireWolfData); // This was "Broken" in the blueprint.
    is_animal_constraint(DireWolfData);

    // A FerociousWarg is a Warg, a Dog, and an Animal.
    is_ferocioswarg_constraint(FerociousWargData);
    is_warg_constraint(FerociousWargData);
    is_dog_constraint(FerociousWargData); // This was "Broken" in the blueprint.
    is_animal_constraint(FerociousWargData);

    // --- Negative Constraint Tests (SHOULD NOT COMPILE) ---
    // The following lines are commented out because they represent invalid relationships.
    // If the macro is correct, uncommenting them would cause a compile-time error,
    // which is the desired behavior.

    // is_warg_constraint(WolfData); // A Wolf is not a Warg.
    // is_wolf_constraint(WargData); // A Warg is not a Wolf.
    // is_direwolf_constraint(FerociousWargData); // A DireWolf is not a FerociousWarg.
}

// A public function to be called from main.rs, allowing us to run this experiment.
pub fn run_experiment() {
    println!("Running new art experiment...");
    // We just need to call the test function. The magic happens at compile time.
    test();
    println!("New art experiment completed successfully (compiled).");
}
