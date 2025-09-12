// Only general declarations that don't mention the objects (Wolf, Warg, DireWolf, FerociousWarg) are allowed outside of the sections

trait Class { type Type; }

// Base class - Dog: must not mention any children (Wolf, Warg) or grandchildren (Direfolf, FerociousWarg) and their data
// - Begin dog section - //

trait DogConstraint: Class {}
trait DogInherit<T> {}
impl<T: DogInherit<<T as Class>::Type> + Class> DogConstraint for T {}

struct Dog; // Marker
struct DogData;
impl Class for DogData { type Type = Dog; }
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
impl Class for WolfData { type Type = Wolf; }
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
impl Class for WargData { type Type = Warg; }
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
impl Class for DireWolfData { type Type = DireWolf; }
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
impl Class for FerociousWargData { type Type = FerociousWarg; }
impl FerociousWargInherit<FerociousWarg> for FerociousWargData {}

// - End ferociouswarg section - //
// Only tests allowed below this line

#[allow(dead_code)]
fn test() {
    // These tests check the branch split, not the hierarchy yet.
    fn is_dog_branch<T: DogInherit<Dog>>(_: T) {}
    fn is_wolf_branch<T: DogInherit<Wolf>>(_: T) {}
    fn is_warg_branch<T: DogInherit<Warg>>(_: T) {}

    is_dog_branch(DogData);
    is_wolf_branch(WolfData);
    is_warg_branch(WargData);

    // These will fail until hierarchy is re-introduced, which is expected.
    // fn is_dog_constraint<T: DogConstraint>(_: T) {}
    // is_dog_constraint(WolfData);
}
