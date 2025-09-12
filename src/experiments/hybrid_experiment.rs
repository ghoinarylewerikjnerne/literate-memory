use crate::macros::hybrid_macros::{Class, self};

// Define the hierarchy using the single, unified macro.
define_hybrid_class!(abstract Animal);
define_hybrid_class!(Dog);
define_hybrid_class!(Wolf, Dog);
define_hybrid_class!(Warg, Dog);
define_hybrid_class!(DireWolf, Wolf);
define_hybrid_class!(FerociousWarg, Warg);

#[allow(dead_code)]
fn test() {
    // Example of how to instantiate a concrete object:
    let dog = DogData::new(());
    let wolf = WolfData::new(dog); // WolfData extends DogData
    let direwolf = DireWolfData::new(wolf);

    // Basic test to ensure Deref works
    assert_eq!(direwolf.0.0.0, ());

    // Constraint tests will be added once the macros are fully debugged.
    // fn is_dog<T: DogConstraint>(_: T) {}
    // is_dog(direwolf);
}
