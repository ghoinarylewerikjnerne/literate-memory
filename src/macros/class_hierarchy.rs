pub trait Class {
    type Type;
}

#[macro_export]
macro_rules! define_class {
    // Rule for the root abstract class (no parent)
    ($name:ident) => {
        paste::paste! {
            // - Begin [<$name:lower>] section - //

            pub trait [<$name Constraint>]: $crate::macros::class_hierarchy::Class {}
            pub trait [<$name Inherit>]<T> {}
            impl<T: [<$name Inherit>]<<T as $crate::macros::class_hierarchy::Class>::Type> + $crate::macros::class_hierarchy::Class> [<$name Constraint>] for T {}
            // This impl is slightly different for the root, it inherits from itself.
            impl<T: [<$name Constraint>]> [<$name Inherit>]<$name> for T {}

            pub struct $name; // Marker
            // No Data struct for abstract class

            // - End [<$name:lower>] section - //
        }
    };
    // Rule for child classes
    ($name:ident, $parent:ident) => {
        paste::paste! {
            // - Begin [<$name:lower>] section - //

            pub trait [<$name Constraint>]: $crate::macros::class_hierarchy::Class {}
            pub trait [<$name Inherit>]<T> {}
            impl<T: [<$name Inherit>]<<T as $crate::macros::class_hierarchy::Class>::Type> + $crate::macros::class_hierarchy::Class> [<$name Constraint>] for T {}
            impl<T: [<$name Constraint>]> [<$parent Inherit>]<$name> for T {}

            pub struct $name; // Marker
            pub struct [<$name Data>];
            impl $crate::macros::class_hierarchy::Class for [<$name Data>] {
                type Type = $name;
            }
            impl [<$name Inherit>]<$name> for [<$name Data>] {}

            // - End [<$name:lower>] section - //
        }
    };
}
