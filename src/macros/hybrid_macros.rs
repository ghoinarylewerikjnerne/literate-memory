pub trait Class {
    type Type;
}

#[macro_export]
macro_rules! declare_class_header {
    // Top-level arm for simple class (no parent)
    ($(#[$outer:meta])* $v:vis $object_name:ident) => {
        $(#[$outer])*
        $v struct $object_name<T>(pub T);

        impl<T> $object_name<T> {
            pub fn new(instance: T) -> Self {
                Self(instance)
            }
        }

        impl<T> ::std::ops::Deref for $object_name<T> {
            type Target = T;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<T> ::std::ops::DerefMut for $object_name<T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
    // Top-level arm for extended class
    ($(#[$outer:meta])* $v:vis $object_name:ident extends $parent_name:ident) => {
        $(#[$outer])*
        $v struct $object_name<T>($parent_name<T>);

        impl<T> $object_name<T> {
            pub fn new(instance: T) -> Self {
                Self($parent_name::new(instance))
            }
        }

        impl<T> ::std::ops::Deref for $object_name<T> {
            type Target = $parent_name<T>;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<T> ::std::ops::DerefMut for $object_name<T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

#[macro_export]
macro_rules! define_hybrid_class {
    // Rule for abstract class (no parent, no data)
    (abstract $name:ident) => {
        paste::paste! {
            declare_class_header!(pub $name);
            pub trait [<$name Constraint>]: $crate::macros::hybrid_macros::Class {}
            pub trait [<$name Inherit>]<T> {}
            impl<T: [<$name Inherit>]<<T as $crate::macros::hybrid_macros::Class>::Type> + $crate::macros::hybrid_macros::Class> [<$name Constraint>] for T {}
            impl<T: [<$name Constraint>]> [<$name Inherit>]<$name> for T {}
        }
    };
    // Rule for a concrete class with a parent
    ($name:ident, $parent:ident) => {
        paste::paste! {
            declare_class_header!(pub [<$name Data>] extends [<$parent Data>]);
            declare_class_header!(pub $name);

            pub trait [<$name Constraint>]: [<$parent Constraint>] {}
            pub trait [<$name Inherit>]<T> {}
            impl<T: [<$name Inherit>]<<T as $crate::macros::hybrid_macros::Class>::Type> + $crate::macros::hybrid_macros::Class> [<$name Constraint>] for T {}
            impl<T: [<$name Constraint>]> [<$parent Inherit>]<$name> for T {}

            impl<T> $crate::macros::hybrid_macros::Class for $name<T> {
                type Type = $name<T>;
            }
            impl<T> [<$name Inherit>]<$name> for [<$name Data>]<T> {}
        }
    };
    // Rule for the root concrete class (has data, no parent)
    ($name:ident) => {
        paste::paste! {
            declare_class_header!(pub [<$name Data>]);
            declare_class_header!(pub $name);

            pub trait [<$name Constraint>]: $crate::macros::hybrid_macros::Class {}
            pub trait [<$name Inherit>]<T> {}
            impl<T: [<$name Inherit>]<<T as $crate::macros::hybrid_macros::Class>::Type> + $crate::macros::hybrid_macros::Class> [<$name Constraint>] for T {}

            impl<T> $crate::macros::hybrid_macros::Class for $name<T> {
                type Type = $name<T>;
            }
            impl<T> [<$name Inherit>]<$name> for [<$name Data>]<T> {}
        }
    };
}
