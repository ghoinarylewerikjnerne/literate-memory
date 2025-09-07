use std::ops::{Deref, DerefMut};
use crate::a::{A, AMethods};
use app::bind_fields;

bind_fields!{
    // Child struct and trait
    pub struct B {
        parent: A,
        pub field_b: i32,
    }

    pub trait BMethods: AMethods {
        fn method_b(&self);
    }

    impl AMethods for B {
        fn method_a(&self) {
            // This method call is on `self.parent`, not `self`,
            // so the macro does not affect it.
            self.parent.method_a(); // Delegate to parent
        }
    }

    impl BMethods for B {
        fn method_b(&self) {
            // `field_b` is a direct field of B, so the macro binds it.
            // `field_a` is from the parent A, accessed via Deref, so we must use `self.`
            println!("B's implementation: a = {}, b = {}", self.field_a, field_b);
        }
    }

    impl Deref for B {
        type Target = A;

        fn deref(&self) -> &A {
            &self.parent
        }
    }

    impl DerefMut for B {
        fn deref_mut(&mut self) -> &mut A {
            &mut self.parent
        }
    }

    impl B {
        pub fn new(field_a: i32, field_b: i32) -> Self {
            B {
                parent: A::new(field_a),
                field_b,
            }
        }
    }
}
