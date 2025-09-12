use crate::macros::class_hierarchy::Class;

// General declarations
define_class!(Animal);

// Base class
define_class!(Dog, Animal);

// Children
define_class!(Wolf, Dog);
define_class!(Warg, Dog);

// Grandchildren
define_class!(DireWolf, Wolf);
define_class!(FerociousWarg, Warg);

// Only tests allowed below this line
#[allow(dead_code)]
fn test() {
    // These tests check the branch split, not the hierarchy yet.
    fn is_animal_animal_branch<T: AnimalInherit<Animal>>(_: T) {}
    fn is_animal_dog_branch<T: AnimalInherit<Dog>>(_: T) {}
    fn is_animal_wolf_branch<T: AnimalInherit<Wolf>>(_: T) {}
    fn is_animal_warg_branch<T: AnimalInherit<Warg>>(_: T) {}
    fn is_animal_direwolf_branch<T: AnimalInherit<DireWolf>>(_: T) {}
    fn is_animal_ferocioswarg_branch<T: AnimalInherit<FerociousWarg>>(_: T) {}

    fn is_dog_branch<T: DogInherit<Dog>>(_: T) {}
    fn is_wolf_branch<T: DogInherit<Wolf>>(_: T) {}
    fn is_warg_branch<T: DogInherit<Warg>>(_: T) {}

    fn is_animal_constraint<T: AnimalConstraint>(_: T) {}
    fn is_dog_constraint<T: DogConstraint>(_: T) {}
    fn is_wolf_constraint<T: WolfConstraint>(_: T) {}
    fn is_warg_constraint<T: WargConstraint>(_: T) {}
    fn is_direwolf_constraint<T: DireWolfConstraint>(_: T) {}
    fn is_ferocioswarg_constraint<T: FerociousWargConstraint>(_: T) {}

    // is_animal_animal_branch(AnimalData); // Animal is abstract
    is_animal_dog_branch(DogData);
    is_animal_dog_branch(WolfData);
    is_animal_dog_branch(WargData);

    // Broken
    // is_animal_wolf_branch(WolfData);
    // is_animal_warg_branch(WargData);
    // is_animal_wolf_branch(DireWolfData);
    // is_animal_warg_branch(FerociousWargData);
    // is_animal_direwolf_branch(DireWolfData);
    // is_animal_ferocioswarg_branch(FerociousWargData);

    is_dog_branch(DogData);
    is_wolf_branch(WolfData);
    is_warg_branch(WargData);
    is_wolf_branch(DireWolfData);
    is_warg_branch(FerociousWargData);

    is_animal_constraint(DogData);
    is_dog_constraint(DogData);
    is_dog_constraint(WolfData);
    is_dog_constraint(WargData);

    is_wolf_constraint(WolfData);
    is_warg_constraint(WargData);

    is_wolf_constraint(DireWolfData);
    is_warg_constraint(FerociousWargData);

    is_direwolf_constraint(DireWolfData);
    is_ferocioswarg_constraint(FerociousWargData);

    // Broken
    // is_dog_constraint(DireWolfData);
    // is_dog_constraint(FerociousWargData);
}
