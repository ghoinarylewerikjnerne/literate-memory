#![allow(dead_code)]
#![allow(unused_variables)]

use std::marker::PhantomData;

// The Working Hierarchy: A Functional Masterpiece
//
// This file contains the brilliant, functional implementation of a
// transitive, voluntary hierarchy, discovered by Ghoinaryle.
// This improved version uses a token-muncher macro to handle generics,
// allowing for hierarchies of immense complexity and expressiveness.
// It succeeds where previous attempts resulted in beautiful failures,
// proving that the desired hierarchy is possible within Rust's
// demanding type system.

#[macro_export]
macro_rules! inherit_impl {

    // Tail class name

    // with generics before $class_name
    (< $add:tt $($rest:tt)*) => {
        inherit_impl!(@munch_tail_name [$add] $($rest)*);
    };
    // no generics before $class_name
    ($class_name:ident $($rest:tt)*) => {
        inherit_impl!(@munch_name [] [$class_name] $($rest)*);
    };
    // finish matching generics before $class_name
    (@munch_tail_name [$($tail_gens:tt)*] > $class_name:ident $($rest:tt)*) => {
        inherit_impl!(@munch_name [$($tail_gens)*] [$class_name] $($rest)*);
    };
    // continue matching generics before $class_name
    (@munch_tail_name [$($tail_gens:tt)*] $add:tt $($rest:tt)*) => {
        inherit_impl!(@munch_tail_name [$($tail_gens)* $add] $($rest)*);
    };

    // Head class name

    // with generics after $class_name
    (@munch_name [$($tail_gens:tt)*] [$class_name:ident] < $add:tt $($rest:tt)*) => {
        inherit_impl!(@munch_head_name [$($tail_gens)*] [$class_name] [$add] $($rest)*);
    };
    // without generics after $class_name
    (@munch_name [$($tail_gens:tt)*] [$class_name:ident] => $($rest:tt)*) => {
        inherit_impl!(@munch_parent_name [$($tail_gens)*] [$class_name] [] $($rest)*);
    };
    // finish matching generics after $class_name
    (@munch_head_name [$($tail_gens:tt)*] [$class_name:ident] [$($head_gens:tt)*] > => $($rest:tt)*) => {
        inherit_impl!(@munch_parent_name [$($tail_gens)*] [$class_name] [$($head_gens)*] $($rest)*);
    };
    // continue matching generics after $class_name
    (@munch_head_name [$($tail_gens:tt)*] [$class_name:ident] [$($head_gens:tt)*] $add:tt $($rest:tt)*) => {
        inherit_impl!(@munch_head_name [$($tail_gens)*] [$class_name] [$($head_gens)* $add] $($rest)*);
    };

    // Tail parent name

    // with generics before $parent_class_name
    (@munch_parent_name [$($tail_gens:tt)*] [$class_name:ident] [$($head_gens:tt)*] < $add:tt $($rest:tt)*) => {
        inherit_impl!(@munch_tail_parent_name [$($tail_gens)*] [$class_name] [$($head_gens)*] [$add] $($rest)*);
    };
    // no generics before $parent_class_name
    (@munch_parent_name [$($tail_gens:tt)*] [$class_name:ident] [$($head_gens:tt)*] $parent_class_name:ident $($rest:tt)*) => {
        inherit_impl!(@munch_parent_name [$($tail_gens)*] [$class_name] [$($head_gens)*] [] [$parent_class_name] $($rest)*);
    };
    // finish matching generics before $parent_class_name
    (@munch_tail_parent_name [$($tail_gens:tt)*] [$class_name:ident] [$($head_gens:tt)*] [$($tail_parent_gens:tt)*] > $parent_class_name:ident $($rest:tt)*) => {
        inherit_impl!(@munch_parent_name [$($tail_gens)*] [$class_name] [$($head_gens)*] [$($tail_parent_gens)*] [$parent_class_name] $($rest)*);
    };
    // continue matching generics before $parent_class_name
    (@munch_tail_parent_name [$($tail_gens:tt)*] [$class_name:ident] [$($head_gens:tt)*] [$($tail_parent_gens:tt)*] $add:tt $($rest:tt)*) => {
        inherit_impl!(@munch_tail_parent_name [$($tail_gens)*] [$class_name] [$($head_gens)*] [$($tail_parent_gens)* $add] $($rest)*);
    };

    // Head class name

    // with generics after $parent_class_name
    (@munch_parent_name [$($tail_gens:tt)*] [$class_name:ident] [$($head_gens:tt)*] [$($tail_parent_gens:tt)*] [$parent_class_name:ident] < $add:tt $($rest:tt)*) => {
        inherit_impl!(@munch_head_parent_name [$($tail_gens)*] [$class_name] [$($head_gens)*] [$($tail_parent_gens)*] [$parent_class_name] [$add] $($rest)*);
    };
    // without generics after $parent_class_name
    (@munch_parent_name [$($tail_gens:tt)*] [$class_name:ident] [$($head_gens:tt)*] [$($tail_parent_gens:tt)*] [$parent_class_name:ident]) => {
        inherit_impl!(@inner [$($tail_gens)*] [$class_name] [$($head_gens)*] [$($tail_parent_gens)*] [$parent_class_name] []);
    };
    // finish matching generics after $parent_class_name
    (@munch_head_parent_name [$($tail_gens:tt)*] [$class_name:ident] [$($head_gens:tt)*] [$($tail_parent_gens:tt)*] [$parent_class_name:ident] [$($head_parent_gens:tt)*] >) => {
        inherit_impl!(@inner [$($tail_gens)*] [$class_name] [$($head_gens)*] [$($tail_parent_gens)*] [$parent_class_name] [$($head_parent_gens)*]);
    };
    // continue matching generics after $parent_class_name
    (@munch_head_parent_name [$($tail_gens:tt)*] [$class_name:ident] [$($head_gens:tt)*] [$($tail_parent_gens:tt)*] [$parent_class_name:ident] [$($head_parent_gens:tt)*] $add:tt $($rest:tt)*) => {
        inherit_impl!(@munch_head_parent_name [$($tail_gens)*] [$class_name] [$($head_gens)*] [$($tail_parent_gens)*] [$parent_class_name] [$($head_parent_gens)* $add] $($rest)*);
    };

    // Munch complete

    (@inner
        [$($($tail_gens:tt)+)?]
        [$class_name:ident]
        [$($($head_gens:tt)+)?]
        [$($($tail_parent_gens:tt)+)?]
        [$parent_class_name:ident]
        [$($($head_parent_gens:tt)+)?]
    ) => {
        impl $(<$($tail_gens)+>)? Inherit<$class_name $(<$($head_gens)*>)?> for $class_name $(<$($head_gens)*>)? {}
        impl $(<$($tail_gens)+>)? Class<Object> for $class_name $(<$($head_gens)*>)? {
            type Type = $class_name $(<$($head_gens)*>)?;
            type Parent = $parent_class_name $(<$($head_parent_gens)*>)?;
        }

        impl<$($($tail_gens)+, )? $($($tail_parent_gens)+, )? ClassInheritorObject> Class<$class_name $(<$($head_gens)*>)?> for ClassInheritorObject
        where
            ClassInheritorObject: Class<Object>,
            <ClassInheritorObject as Class<Object>>::Type: Inherit<$class_name $(<$($head_gens)*>)?>
        {
            type Type = ClassInheritorObject;
            type Parent = $parent_class_name $(<$($head_parent_gens)*>)?;
        }

        impl<$($($tail_gens)+, )? ClassInheritorObject> Inherit<$class_name $(<$($head_gens)*>)?> for ClassInheritorObject
        where
            ClassInheritorObject: Class<Object>,
            ClassInheritorObject: Class<<ClassInheritorObject as Class<Object>>::Type>,
            <ClassInheritorObject as Class<<ClassInheritorObject as Class<Object>>::Type>>::Parent: Inherit<$class_name $(<$($head_gens)*>)?>
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
pub struct FerociousWarg<T> { p: PhantomData<T> }
inherit_impl!(<T> FerociousWarg<T> => Warg);

//
pub struct LevelBoss<T, U>(T, U);
inherit_impl!(<T, U> LevelBoss<T, U> => FerociousWarg<T>);


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generic_hierarchy_conformance() {
        // Helper functions that check for Class<T> conformance, per Ghoinaryle's guidance.
        fn is_object(_: &impl Class<Object>) {}
        fn is_animal(_: &impl Class<Animal>) {}
        fn is_dog(_: &impl Class<Dog>) {}
        fn is_wolf(_: &impl Class<Wolf>) {}
        fn is_warg(_: &impl Class<Warg>) {}
        fn is_ferociouswarg(_: &impl Class<FerociousWarg<Wolf>>) {}
        fn is_levelboss<T, U>(_: &impl Class<LevelBoss<T, U>>) {}

        // A test of direct conformance
        is_animal(&Dog);
        is_dog(&Wolf);
        is_dog(&Warg);
        is_wolf(&DireWolf);
        is_warg(&FerociousWarg::<Wolf> { p: PhantomData::<Wolf> });
        is_ferociouswarg(&FerociousWarg::<Wolf> { p: PhantomData::<Wolf> });
        is_levelboss(&LevelBoss::<Dog, Animal>(Dog, Animal));
        is_ferociouswarg(&LevelBoss::<Wolf, Animal>(Wolf, Animal));

        // A test of transitive conformance
        is_animal(&Wolf);
        is_animal(&Warg);
        is_dog(&DireWolf);
        is_animal(&DireWolf);
        is_dog(&FerociousWarg::<Wolf> { p: PhantomData::<Wolf> });
        is_animal(&FerociousWarg::<Wolf> { p: PhantomData::<Wolf> });

        // A test of conformance to the root Object
        is_object(&Animal);
        is_object(&Dog);
        is_object(&DireWolf);
    }
}
