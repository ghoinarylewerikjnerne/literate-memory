// This macro relies on the `class!` macro (from `define_object.rs`) being in scope,
// and the `paste` crate.

#[macro_export]
macro_rules! complete_class {
    // ----------------- Entry Points -----------------

    // Entry for: root abstract class, no implements
    ($vis:vis abstract $name:ident) => {
        class!($vis $name);
    };

    // Entry for: abstract class, with or without parent
    ($vis:vis abstract $name:ident $(extends $parent:ident)? implements { $($impls:tt)* }) => {
        $crate::complete_class!(@inner_abstract
            { vis: [$vis], name: [$name], extends: [$($parent)?] }
            // Pass the implements block twice for dual parsing
            { { $($impls)* } } // For item parsing
            { { $($impls)* } } // For name extraction
        );
    };

    // Entry for: concrete class
    ($vis:vis $name:ident extends $parent:ident
        $(implements { $($impls:tt)* })?
        stores $s_name:ident { $($s_fields:tt)* }
        provides { $($provs:tt)* }
    ) => {
        $crate::complete_class!(@inner_concrete
            { vis: [$vis], name: [$name], parent: [$parent], s_name: [$s_name], s_fields: [{$($s_fields)*}] }
            // Pass implements block twice if it exists, otherwise pass empty blocks
            { { $( $($impls)* )? } }
            { { $( $($impls)* )? } }
            // Pass provides block twice
            { { $($provs)* } }
            { { $($provs)* } }
        );
    };

    // ----------------- Inner Parsers & Generators -----------------

    // Parser/Generator for ABSTRACT classes
    (@inner_abstract
        { vis: [$vis:vis], name: [$name:ident], extends: [$($parent:ident)?] }
        { { $($imethod_item:item)* } }
        { { $(fn $imethod_name:ident ($($iargs:tt)*) $(-> $iret:ty)? ;)* } }
    ) => {
        ::paste::paste! {
            class!($vis $name $(extends $parent)?);
            $(
                #[allow(non_camel_case_types)]
                $vis trait [<Trait_ $imethod_name>] {
                    $imethod_item
                }
            )*
        }
    };

    // Parser/Generator for CONCRETE classes
    (@inner_concrete
        { vis: [$vis:vis], name: [$name:ident], parent: [$parent:ident], s_name: [$s_name:ident], s_fields: [{$($s_fields:tt)*}] }
        // Implements block (optional)
        { { $($imethod_item:item)* } }
        { { $(fn $imethod_name:ident ($($iargs:tt)*) $(-> $iret:ty)? ;)* } }
        // Provides block
        { { $($pmethod_item:item)* } }
        { { $(fn $pmethod_name:ident ($($pargs:tt)*) $(-> $pret:ty)? { $($pbody:tt)* } )* } }
    ) => {
        ::paste::paste! {
            // 0. Generate the class struct itself
            class!($vis $name extends $parent);

            // 1. Generate the data struct
            $vis struct $s_name {
                $($s_fields)*
            }

            // 2. Generate new traits if there's an `implements` block
            $(
                #[allow(non_camel_case_types)]
                $vis trait [<Trait_ $imethod_name>] {
                    fn $imethod_name($($iargs)*) $(-> $iret)?;
                }
            )*

            // 3. Implement the traits for the data struct.
            $(
                // This implements traits from the PARENT
                impl [<Trait_ $pmethod_name>] for $s_name {
                    fn $pmethod_name($($pargs)*) $(-> $pret)? {
                        $($pbody)*
                    }
                }
            )*
        }
    };
}
