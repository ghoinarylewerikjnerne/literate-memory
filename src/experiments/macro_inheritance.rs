// Experiment for macro-powered inheritance

// The `class!` macro is brought into scope by the `#[macro_use]`
// attribute on the `define_object` module in `main.rs`.

// Define the class hierarchy
class!(pub Object);
class!(pub Animal extends Object);
class!(pub Dog extends Animal);
class!(pub Warg extends Dog);
class!(pub Wolf extends Dog);

// Implement a method on the Animal class
impl<T: FoodPreference> Animal<T> {
    pub fn eat(&self) -> String {
        format!("is eating {}.", self.preferred_food())
    }
}

// Trait to define food preference
pub trait FoodPreference {
    fn preferred_food(&self) -> &str;
}

// Define behavior with a trait
pub trait CanineBehavior {
    fn howl(&self) -> &str;
}

// Define the concrete data structs
pub struct WargData;
pub struct WolfData;

// Implement the behavior for the data structs
impl CanineBehavior for WargData {
    fn howl(&self) -> &str {
        "a menacing, deep howl"
    }
}

impl CanineBehavior for WolfData {
    fn howl(&self) -> &str {
        "a lonely, high-pitched howl"
    }
}

impl FoodPreference for WargData {
    fn preferred_food(&self) -> &str {
        "meat"
    }
}

impl FoodPreference for WolfData {
    fn preferred_food(&self) -> &str {
        "rabbits"
    }
}

pub fn run_experiment() {
    println!("--- Running Macro-Powered Inheritance Experiment ---");

    let warg = Warg::new(WargData);
    let wolf = Wolf::new(WolfData);

    // Thanks to the `Deref` trait implementations in the macro,
    // we can call the `howl` method directly on the `Warg` and `Wolf` objects.
    // The calls are "statically dispatched" to the correct implementation at compile time.
    println!("The warg lets out {}", warg.howl());
    println!("The wolf lets out {}", wolf.howl());

    // Now, let's call the `eat` method defined on the `Animal` class.
    // This works because `Warg` and `Wolf` deref to `Dog`, which derefs to `Animal`.
    println!("The warg {}", warg.eat());
    println!("The wolf {}", wolf.eat());

    println!("--- Experiment Finished ---");
}
