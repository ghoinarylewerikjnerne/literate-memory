use app::bind_fields;

bind_fields! {
    // Parent struct and trait
    pub struct A {
        pub field_a: i32,
    }

    pub trait AMethods {
        fn method_a(&self);
    }

    impl AMethods for A {
        fn method_a(&self) {
            // `self.` is removed here, `field_a` is bound by the macro.
            println!("A's implementation: a = {}", field_a);
        }
    }

    impl A {
        pub fn new(field_a: i32) -> Self {
            A {
                field_a,
            }
        }
    }
}
