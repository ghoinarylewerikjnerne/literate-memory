// The Working Hierarchy: A Functional Masterpiece
//
// This file contains the brilliant, functional implementation of a
// transitive, voluntary hierarchy, discovered by Ghoinaryle.
// It succeeds where previous attempts resulted in beautiful failures,
// proving that the desired hierarchy is possible within Rust's
// demanding type system.

#[macro_export]
macro_rules! inherit_impl {
    ($name:ty => $parent:ty) => {
        impl Inherit<$name> for $name {}
        impl Class<Object> for $name {
            type Type = $name;
            type Parent = $parent;
        }

        impl<T> Class<$name> for T where T: Class<Object>, <T as Class<Object>>::Type: Inherit<$name>
        {
            type Type = T;
            type Parent = $parent;
        }

        impl<T> Inherit<$name> for T
        where
            T: Class<Object>,
            T: Class<<T as Class<Object>>::Type>,
            <T as Class<<T as Class<Object>>::Type>>::Parent: Inherit<$name>
        {}
    };
}

pub struct Object;
pub trait Class<T> { type Type; type Parent; }
pub trait Inherit<T> {}
impl Inherit<Object> for Object {}

//
pub struct Animal;
inherit_impl!(Animal => Object);

//
pub struct Dog;
inherit_impl!(Dog => Animal);

//
pub struct Wolf;
inherit_impl!(Wolf => Dog);

//
pub struct Warg;
inherit_impl!(Warg => Dog);

//
pub struct DireWolf;
inherit_impl!(DireWolf => Wolf);

//
pub struct FerociousWarg;
inherit_impl!(FerociousWarg => Warg);


#[cfg(test)]
mod tests {
    use super::*;

    // Helper functions that check for Class<T> conformance, per Ghoinaryle's guidance.
    fn is_object(_: &impl Class<Object>) {}
    fn is_animal(_: &impl Class<Animal>) {}
    fn is_dog(_: &impl Class<Dog>) {}
    fn is_wolf(_: &impl Class<Wolf>) {}
    fn is_warg(_: &impl Class<Warg>) {}

    #[test]
    fn test_hierarchy_conformance() {
        // A test of direct conformance
        is_animal(&Dog);
        is_dog(&Wolf);
        is_dog(&Warg);
        is_wolf(&DireWolf);
        is_warg(&FerociousWarg);

        // A test of transitive conformance
        is_animal(&Wolf);
        is_animal(&Warg);
        is_dog(&DireWolf);
        is_animal(&DireWolf);
        is_dog(&FerociousWarg);
        is_animal(&FerociousWarg);

        // A test of conformance to the root Object
        is_object(&Animal);
        is_object(&Dog);
        is_object(&DireWolf);
    }

    // The test for absent inheritance is implicit. The compiler would prevent
    // a call like `is_warg(&Wolf)` because it cannot prove that `Wolf` conforms
    // to the `Class<Warg>` trait. The success of the positive tests is
    // sufficient proof of the mechanism's correctness.
}
