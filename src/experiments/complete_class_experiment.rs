// --- Using the complete_class! macro ---

// Define the abstract base class
complete_class!(pub abstract Object);

// Define the abstract Animal class
complete_class!(pub abstract Animal extends Object implements {
    fn speak(&self);
    fn preferred_food(&self) -> &str;
    // A default implementation, goes in trait definition
    fn is_animal() -> bool/* { true } - TODO: default trait impls not supported yet */;
});

// Define the concrete Dog class
complete_class!(pub Dog extends Animal implements {
    // These methods are abstract at the Dog level, to be provided by subclasses
    fn howl(&self) -> &str;
    fn get_mood(&self) -> &str;
} stores DogData {
    pub mood: &'static str,
} provides {
    // This provides the implementation for the 'speak' method from the Animal trait
    fn speak(&self) { println!("The dog lets out {}", self.howl()); }
    // This provides the implementation for the 'howl' method
    fn howl(&self) -> &str { "a generic dog howl" }
    // This provides the implementation for 'preferred_food' from Animal
    fn preferred_food(&self) -> &str { "kibble" }
    // This provides the implementation for 'get_mood'
    fn get_mood(&self) -> &str { self.mood }
});

// --- Experiment Runner ---
pub fn run_experiment() {
    println!("--- Running Complete Class Macro Experiment ---");

    let my_dog = Dog::new(DogData { mood: "happy" });
    my_dog.speak();
    println!("My dog is in a {} mood.", my_dog.get_mood());
    println!("My dog prefers to eat {}.", my_dog.preferred_food());

    println!("\n--- Experiment Finished ---");
}
