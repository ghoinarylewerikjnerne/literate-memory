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

// Implement a method on the Dog class
impl<T: HasMood> Dog<T> {
    pub fn check_mood(&self) -> String {
        format!("seems to be in a {} mood.", self.get_mood())
    }
}

// Trait to define mood
pub trait HasMood {
    fn get_mood(&self) -> &str;
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
pub struct WargData {
    pub mood: &'static str,
}
pub struct WolfData {
    pub mood: &'static str,
}

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

impl HasMood for WargData {
    fn get_mood(&self) -> &str {
        self.mood
    }
}

impl HasMood for WolfData {
    fn get_mood(&self) -> &str {
        self.mood
    }
}

pub fn run_experiment() {
    println!("--- Running Macro-Powered Inheritance Experiment ---");

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
