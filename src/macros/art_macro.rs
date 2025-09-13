// This macro is the internal implementation. It is not intended for direct use.
// It contains the recursive muncher and generator logic.
// By keeping it separate, we avoid the public-facing macro from accidentally
// matching its own internal calls, which was the source of the infinite recursion.
#[macro_export]
macro_rules! define_artistic_hierarchy_impl {
    // --- Muncher Arms ---

    // Process an abstract class
    (@process_decls abstract $name:ident; $($rest:tt)*) => {
        define_artistic_hierarchy_impl!(@gen_abstract_class $name);
        define_artistic_hierarchy_impl!(@process_decls $($rest)*);
    };

    // Process a concrete class with a parent
    (@process_decls class $name:ident extends $parent:ident; $($rest:tt)*) => {
        define_artistic_hierarchy_impl!(@gen_concrete_class $name, $parent);
        define_artistic_hierarchy_impl!(@process_decls $($rest)*);
    };

    // End of recursion
    (@process_decls) => {};

    // --- Code Generation Arms ---

    // Generate code for an abstract class
    (@gen_abstract_class $name:ident) => {
        paste::paste! {
            pub trait [<$name Constraint>]: HierarchyClass {}
            pub trait [<$name Inherit>]<T> {}
            impl<T: [<$name Inherit>]< <T as HierarchyClass>::Type > + HierarchyClass> [<$name Constraint>] for T {}
            impl<T: [<$name Constraint>]> [<$name Inherit>]<$name> for T {}
            pub struct $name;
        }
    };

    // Generate code for a concrete class with a parent
    (@gen_concrete_class $name:ident, $parent:ident) => {
        paste::paste! {
            pub trait [<$name Constraint>]: [<$parent Constraint>] {}
            pub trait [<$name Inherit>]<T> {}
            impl<T: [<$name Inherit>]< <T as HierarchyClass>::Type > + HierarchyClass> [<$name Constraint>] for T {}

            // This is the key to transitive inheritance.
            // It says that if a type `T` inherits from a child (`$name`),
            // it also automatically inherits from the parent (`$parent`).
            impl<U, T: [<$name Inherit>]<U>> [<$parent Inherit>]<U> for T {}

            pub struct $name;
            pub struct [<$name Data>];
            impl HierarchyClass for [<$name Data>] {
                type Type = $name;
            }
            impl [<$name Inherit>]<$name> for [<$name Data>] {}
        }
    };
}

// This is the public-facing macro. It's the only one a user should call.
// It sets up the environment and then delegates to the implementation macro.
#[macro_export]
macro_rules! define_artistic_hierarchy {
    ( $( $decl:tt )* ) => {
        // Define our unique, public trait once to avoid conflicts and provide a base.
        pub trait HierarchyClass {
            type Type;
        }
        // Call the implementation macro to do the actual recursive work.
        // We kick it off by calling its entrypoint muncher rule.
        define_artistic_hierarchy_impl!(@process_decls $($decl)*);
    };
}
