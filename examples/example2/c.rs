use std::ops::{Deref, DerefMut};
use crate::b::{B, BMethods};
use crate::a::AMethods;
use app::bind_fields;

bind_fields! {
    // Grandchild struct and trait
    pub struct C<'a> {
        parent: B,
        pub field_c: &'a str,
    }

    pub trait CMethods: BMethods {
        fn method_c(&self);
    }

    impl Deref for C<'_> {
        type Target = B;

        fn deref(&self) -> &B {
            &self.parent
        }
    }

    impl DerefMut for C<'_> {
        fn deref_mut(&mut self) -> &mut B {
            &mut self.parent
        }
    }

    impl<'a> C<'a> {
        pub fn new(field_a: i32, field_b: i32, field_c: &'a str) -> Self {
            C {
                parent: B::new(field_a, field_b),
                field_c,
            }
        }
    }

    impl AMethods for C<'_> {
        fn method_a(&self) {
            self.parent.method_a(); // Delegate to parent
        }
    }

    impl BMethods for C<'_> {
        fn method_b(&self) {
            // This call should be rewritten to `self.method_c()` by the macro.
            method_c();
        }
    }

    impl CMethods for C<'_> {
        fn method_c(&self) {
            // `field_c` is direct. `field_a` and `field_b` are via `Deref`.
            println!("C's implementation: a = {}, b = {}, c = {}", self.field_a, self.field_b, field_c);
        }
    }
}
