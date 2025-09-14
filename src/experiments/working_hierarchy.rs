#![allow(dead_code)]
#![allow(unused_variables)]

use crate::inheritance::{*};
use std::marker::PhantomData;

// The Working Hierarchy: A Functional Masterpiece
//
// This file contains the brilliant, functional implementation of a
// transitive, voluntary hierarchy, discovered by Ghoinaryle.
// It succeeds where previous attempts resulted in beautiful failures,
// proving that the desired hierarchy is possible within Rust's
// demanding type system.

mod inheritance {
    pub struct Object;
    pub trait Class<T> { type Type; type Parent; }
    pub trait Inherit<T> {}
    impl Inherit<Object> for Object {}

    #[macro_export]
    macro_rules! inherit {
        // Tail class name
    
        // with generics before $class_name
        (< $add:tt $($rest:tt)*) => {
            inherit!(@munch_tail_name [$add] $($rest)*);
        };
        // no generics before $class_name
        ($class_name:ty => $($rest:tt)*) => {
            inherit!(@munch_parent_name [] [$class_name] $($rest)*);
        };
        // continue matching generics before $class_name (multi > case)
        (@munch_tail_name [$($tail_gens:tt)*] >> $($rest:tt)*) => {
            inherit!(@munch_tail_name [$($tail_gens)* >] > $($rest)*);
        };
        // finish matching generics before $class_name
        (@munch_tail_name [$($tail_gens:tt)*] > $class_name:ty => $($rest:tt)*) => {
            inherit!(@munch_parent_name [$($tail_gens)*] [$class_name] $($rest)*);
        };
        // continue matching generics before $class_name
        (@munch_tail_name [$($tail_gens:tt)*] $add:tt $($rest:tt)*) => {
            inherit!(@munch_tail_name [$($tail_gens)* $add] $($rest)*);
        };

        // Tail parent name
    
        // with generics before $parent_class_name
        (@munch_parent_name [$($tail_gens:tt)*] [$class_name:ty] < $add:tt $($rest:tt)*) => {
            inherit!(@munch_tail_parent_name [$($tail_gens)*] [$class_name] [$add] $($rest)*);
        };
        // no generics before $parent_class_name
        (@munch_parent_name [$($tail_gens:tt)*] [$class_name:ty] $parent_class_name:ty $(where $($where_clause:tt)*)?) => {
            inherit!(@inner [$($tail_gens)*] [$class_name] [] [$parent_class_name] [$($($where_clause)+)?]);
        };
        // continue matching generics before $parent_class_name (multi > case)
        (@munch_tail_parent_name [$($tail_gens:tt)*] [$class_name:ty] [$($tail_parent_gens:tt)*] >> $($rest:tt)*) => {
            inherit!(@munch_tail_parent_name [$($tail_gens)*] [$class_name] [$($tail_parent_gens)* >] > $($rest)*);
        };
        // finish matching generics before $parent_class_name
        (@munch_tail_parent_name [$($tail_gens:tt)*] [$class_name:ty] [$($tail_parent_gens:tt)*] > $parent_class_name:ty $(where $($where_clause:tt)*)?) => {
            inherit!(@inner [$($tail_gens)*] [$class_name] [$($tail_parent_gens)*] [$parent_class_name] [$($($where_clause)+)?]);
        };
        // continue matching generics before $parent_class_name
        (@munch_tail_parent_name [$($tail_gens:tt)*] [$class_name:ty] [$($tail_parent_gens:tt)*] $add:tt $($rest:tt)*) => {
            inherit!(@munch_tail_parent_name [$($tail_gens)*] [$class_name] [$($tail_parent_gens)* $add] $($rest)*);
        };
    
        // Munch complete
    
        (@inner
            [$($($tail_gens:tt)+)?]
            [$class_name:ty]
            [$($($tail_parent_gens:tt)+)?]
            [$parent_class_name:ty]
            [$($($where_clause:tt)+)?]
        ) => {
            impl $(<$($tail_gens)+>)? $crate::Inherit<$class_name> for $class_name $(where $($where_clause)+)? {}
            impl $(<$($tail_gens)+>)? $crate::Class<$crate::Object> for $class_name $(where $($where_clause)+)? {
                type Type = $class_name;
                type Parent = $parent_class_name;
            }
    
            impl<$($($tail_gens)+, )? $($($tail_parent_gens)+, )? ClassInheritorObject> $crate::Class<$class_name> for ClassInheritorObject
            where
                ClassInheritorObject: $crate::Class<$crate::Object>,
                <ClassInheritorObject as $crate::Class<$crate::Object>>::Type: $crate::Inherit<$class_name>
                $(, $($where_clause)+)?
            {
                type Type = ClassInheritorObject;
                type Parent = $parent_class_name;
            }
    
            impl<$($($tail_gens)+, )? ClassInheritorObject> $crate::Inherit<$class_name> for ClassInheritorObject
            where
                ClassInheritorObject: $crate::Class<$crate::Object>,
                ClassInheritorObject: $crate::Class<<ClassInheritorObject as $crate::Class<$crate::Object>>::Type>,
                <ClassInheritorObject as $crate::Class<<ClassInheritorObject as $crate::Class<$crate::Object>>::Type>>::Parent: $crate::Inherit<$class_name>
                $(, $($where_clause)+)?
            {}
        };
    }
}

//
pub struct Animal;
inherit!(Animal => Object);

//
pub struct Dog;
inherit!(Dog => Animal);

//
pub struct Wolf;
inherit!(Wolf => Dog);

//
pub struct Warg;
inherit!(Warg => Dog);

//
pub struct DireWolf<'a, T> { p: PhantomData<&'a T> }
inherit!(<'a, T> DireWolf<'a, T> => Wolf);

//
pub struct MightyWarg<T>(T);
inherit!(<T> MightyWarg<T> => Warg);

//
pub struct FerociousWarg<T> { p: PhantomData<T> }
inherit!(<T> FerociousWarg<T> => Warg);

//
pub struct LevelBoss<T, U>(T, U);
inherit!(<T, U> LevelBoss<T, U> => FerociousWarg<T>);

//
pub struct BranchBoss<T, U>(T, U);
inherit!(<'a, T, U> BranchBoss<MightyWarg<T>, LevelBoss<T, &'a U>> => FerociousWarg<T> where U: Class<Wolf>);


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_the_ultimate_hierarchy() {
        // Helper functions that check for Class<T> conformance, per Ghoinaryle's guidance.
        fn is_object(_: &impl Class<Object>) {}
        fn is_animal(_: &impl Class<Animal>) {}
        fn is_dog(_: &impl Class<Dog>) {}
        fn is_wolf(_: &impl Class<Wolf>) {}
        fn is_warg(_: &impl Class<Warg>) {}
        fn is_ferociouswarg<T>(_: &impl Class<FerociousWarg<T>>) {}
        fn is_levelboss<T, U>(_: &impl Class<LevelBoss<T, U>>) {}
        fn is_branchboss<T, U>(_: &impl Class<BranchBoss<T, U>>) {}

        // A test of direct conformance
        is_animal(&Dog);
        is_dog(&Wolf);
        is_dog(&Warg);
        is_wolf(&DireWolf::<'_, Object> { p: PhantomData::<&'_ Object> });
        is_warg(&FerociousWarg::<Wolf> { p: PhantomData::<Wolf> });
        is_ferociouswarg(&FerociousWarg::<Wolf> { p: PhantomData::<Wolf> });
        is_levelboss(&LevelBoss::<Dog, Animal>(Dog, Animal));
        is_ferociouswarg(&LevelBoss::<Wolf, Animal>(Wolf, Animal));
        is_branchboss(&BranchBoss::<MightyWarg<DireWolf<'_, Object>>, LevelBoss<DireWolf<'_, Object>, &Wolf>>(
            MightyWarg(DireWolf::<'_, Object> { p: PhantomData::<&'_ Object> }),
            LevelBoss::<DireWolf<'_, Object>, &Wolf>(
                DireWolf::<'_, Object> { p: PhantomData::<&'_ Object> },
                &Wolf
            )
        ));
        is_animal(&BranchBoss::<MightyWarg<DireWolf<'_, Object>>, LevelBoss<DireWolf<'_, Object>, &Wolf>>(
            MightyWarg(DireWolf::<'_, Object> { p: PhantomData::<&'_ Object> }),
            LevelBoss::<DireWolf<'_, Object>, &Wolf>(
                DireWolf::<'_, Object> { p: PhantomData::<&'_ Object> },
                &Wolf
            )
        ));

        // A test of transitive conformance
        is_animal(&Wolf);
        is_animal(&Warg);
        is_dog(&DireWolf::<'_, Object> { p: PhantomData::<&'_ Object> });
        is_animal(&DireWolf::<'_, Object> { p: PhantomData::<&'_ Object> });
        is_dog(&FerociousWarg::<Wolf> { p: PhantomData::<Wolf> });
        is_animal(&FerociousWarg::<Wolf> { p: PhantomData::<Wolf> });

        // A test of conformance to the root Object
        is_object(&Animal);
        is_object(&Dog);
        is_object(&DireWolf::<'_, Object> { p: PhantomData::<&'_ Object> });
    }
}
