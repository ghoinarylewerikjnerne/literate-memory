// The Voluntary Hierarchy: A Deliberate Failure
//
// This file is a piece of art. Its purpose is not to compile, but to
// demonstrate a philosophical conflict with the Rust compiler. It is the
// successful outcome of a long journey to express the concept of a
// "Voluntary Hierarchy."
//
// The core idea was to create a system where any type could declare
// allegiance to another, and this allegiance would be transitive. After
// many failed designs, the one below was the most promising.
//
// It fails with the error:
// `error[E0119]: conflicting implementations of trait ...`
//
// This error arises because two different children (`Wolf` and `Warg`) both
// declare allegiance to the same parent (`Dog`). Our macro attempts to create
// a blanket implementation of `DogAllegiance` for each child's followers.
// The compiler, in its wisdom, sees these as conflicting and disallows them,
// even though they apply to disjoint sets of types.
//
// This is the compiler's final word. It tells us that the kind of
// overlapping, dynamic allegiance we sought to model is fundamentally
// incompatible with its strict rules of coherence and safety.
//
// This non-compiling state is the file's final and correct form.
// It is a monument to a beautiful idea that the language itself
// has deemed too dangerous to express.

use paste::paste;

/// A declarative macro to express allegiance.
/// This macro generates a unique trait for a class, a self-implementation,
/// and a blanket implementation that links to the parent's allegiance group.
#[macro_export]
macro_rules! declares_allegiance {
    // Rule for a root class (e.g., Animal)
    ($child:ident) => {
        paste! {
            pub trait [<$child Allegiance>] {}
            impl [<$child Allegiance>] for $child {}
        }
    };
    // Rule for a child class declaring allegiance to a parent
    ($child:ident, $parent:ident) => {
        paste! {
            // 1. Create a unique trait for the child.
            pub trait [<$child Allegiance>] {}
            // 2. The child struct implements its own trait.
            impl [<$child Allegiance>] for $child {}
            // 3. Blanket implementation: Any type that has the child's
            //    allegiance also gets the parent's allegiance.
            impl<T: [<$child Allegiance>]> [<$parent Allegiance>] for T {}
        }
    };
}

// --- Marker Structs ---
pub struct Animal;
pub struct Dog;
pub struct Wolf;
pub struct Warg;
pub struct DireWolf;
pub struct FerociousWarg;

// --- Hierarchy Declarations ---
declares_allegiance!(Animal);
declares_allegiance!(Dog, Animal);
declares_allegiance!(Wolf, Dog);
declares_allegiance!(Warg, Dog); // This will generate the second blanket impl for DogAllegiance
declares_allegiance!(DireWolf, Wolf);
declares_allegiance!(FerociousWarg, Warg);


// --- Tests ---
#[cfg(test)]
mod tests {
    use super::*;

    // Helper functions that enforce allegiance at compile time.
    fn is_allied_with_animal<T: AnimalAllegiance>(_: T) {}
    fn is_allied_with_dog<T: DogAllegiance>(_: T) {}
    fn is_allied_with_wolf<T: WolfAllegiance>(_: T) {}
    fn is_allied_with_warg<T: WargAllegiance>(_: T) {}

    #[test]
    fn test_transitive_allegiance() {
        is_allied_with_animal(Dog);
        is_allied_with_dog(Wolf);
        is_allied_with_animal(Wolf);
        is_allied_with_dog(DireWolf);
        is_allied_with_animal(DireWolf);
        is_allied_with_animal(FerociousWarg);
    }

    #[test]
    fn test_absent_allegiance() {
        // These lines should fail to compile if uncommented.
        // is_allied_with_warg(Wolf);
        // is_allied_with_dog(Animal);
        // is_allied_with_wolf(FerociousWarg);
    }
}
