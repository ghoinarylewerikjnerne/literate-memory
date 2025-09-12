// Only general declarations that don't mention the objects (Wolf, Warg, DireWolf, FerociousWarg) are allowed outside of the sections

trait Class {
    type Type;
}

// Root class - Animal: must not mention child (Dog), grandchildren (Wolf, Warg) or grandgrandchildren (Direfolf, FerociousWarg) and their data
// - Begin animal section - //

trait AnimalConstraint: Class {}
trait AnimalInherit<T> {}
impl<T: AnimalInherit<<T as Class>::Type> + Class> AnimalConstraint for T {}
impl<T: AnimalConstraint> AnimalInherit<Animal> for T {}

struct Animal; // Marker
// No AnimalData - abstract class

// - End animal section - //
// Base class - Dog: must not mention any children (Wolf, Warg) or grandchildren (Direfolf, FerociousWarg) and their data
// - Begin dog section - //

trait DogConstraint: Class {}
trait DogInherit<T> {}
impl<T: DogInherit<<T as Class>::Type> + Class> DogConstraint for T {}
impl<T: DogConstraint> AnimalInherit<Dog> for T {}

struct Dog; // Marker
struct DogData;
impl Class for DogData {
    type Type = Dog;
}
impl DogInherit<Dog> for DogData {}

// - End dog section - //
// Child - Wolf: must not mention sibling (Warg), child (DireWolf) or nephew (FerociousWarg), and not know DogData of parent
// - Begin wolf section - //

trait WolfConstraint: Class {}
trait WolfInherit<T> {}
impl<T: WolfInherit<<T as Class>::Type> + Class> WolfConstraint for T {}
impl<T: WolfConstraint> DogInherit<Wolf> for T {}

struct Wolf; // Marker
struct WolfData;
impl Class for WolfData {
    type Type = Wolf;
}
impl WolfInherit<Wolf> for WolfData {}

// - End wolf section - //
// Child - Warg: must not mention sibling (Wolf), child (FerociousWarg) or nephew (DireWolf), and not know DogData of parent
// - Begin warg section - //

trait WargConstraint: Class {}
trait WargInherit<T> {}
impl<T: WargInherit<<T as Class>::Type> + Class> WargConstraint for T {}
impl<T: WargConstraint> DogInherit<Warg> for T {}

struct Warg; // Marker
struct WargData;
impl Class for WargData {
    type Type = Warg;
}
impl WargInherit<Warg> for WargData {}

// - End warg section - //
// Grandchild - DireWolf: must not mention grandparent (Dog), uncle (Warg), or cousin (FerociousWarg), and not know WolfData of parent
// - Begin direwolf section - //

trait DireWolfConstraint: Class {}
trait DireWolfInherit<T> {}
impl<T: DireWolfInherit<<T as Class>::Type> + Class> DireWolfConstraint for T {}
impl<T: DireWolfConstraint> WolfInherit<DireWolf> for T {}

struct DireWolf; // Marker
struct DireWolfData;
impl Class for DireWolfData {
    type Type = DireWolf;
}
impl DireWolfInherit<DireWolf> for DireWolfData {}

// - End direwolf section - //
// Grandchild - FerociousWarg: must not mention grandparent (Dog), uncle (Wolf), or cousin (DireWolf), and not know WargData of parent
// - Begin ferociouswarg section - //

trait FerociousWargConstraint: Class {}
trait FerociousWargInherit<T> {}
impl<T: FerociousWargInherit<<T as Class>::Type> + Class> FerociousWargConstraint for T {}
impl<T: FerociousWargConstraint> WargInherit<FerociousWarg> for T {}

struct FerociousWarg; // Marker
struct FerociousWargData;
impl Class for FerociousWargData {
    type Type = FerociousWarg;
}
impl FerociousWargInherit<FerociousWarg> for FerociousWargData {}

// - End ferociouswarg section - //
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

    is_animal_animal_branch(DogData);
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

    // Fail as expected: hierarchy sorta works?
    // is_warg_branch(WolfData);
    // is_direwolf_branch(DogData);

    is_wolf_branch(DireWolfData);
    is_warg_branch(FerociousWargData);

    // Fail as expected: hierarchy sorta works?
    // is_wolf_branch(FerociousWargData);
    // is_warg_branch(DireWolfData);

    is_animal_constraint(DogData);
    is_dog_constraint(DogData);
    is_dog_constraint(WolfData);
    is_dog_constraint(WargData);

    is_wolf_constraint(WolfData);
    is_warg_constraint(WargData);

    // Fail as expected: hierarchy sorta works?
    // is_wolf_constraint(WargData);
    // is_warg_constraint(WolfData);

    is_wolf_constraint(DireWolfData);
    is_warg_constraint(FerociousWargData);

    // Fail as expected: hierarchy sorta works?
    // is_wolf_constraint(FerociousWargData);
    // is_warg_constraint(DireWolfData);

    is_direwolf_constraint(DireWolfData);
    is_ferocioswarg_constraint(FerociousWargData);

    // Fail as expected: hierarchy sorta works?
    // is_direwolf_constraint(FerociousWargData);
    // is_ferocioswarg_constraint(DireWolfData);

    // Broken
    // is_dog_constraint(DireWolfData);
    // is_dog_constraint(FerociousWargData);
}
