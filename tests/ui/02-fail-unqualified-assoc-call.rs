use app::bind_fields;

bind_fields! {
    pub struct A {
        pub field_a: i32,
        pub field_b: i32,
    }

    impl A {
        pub fn method_a2(&self) {
            // This call should fail because `method_a3` is an associated function
            // and must be called via `Self::method_a3()`. The macro should not
            // rewrite this call because `method_a3` does not have a `self` receiver.
            method_a3();
        }

        pub fn method_a3() {
            println!("method_a3 called");
        }
    }
}

fn main() {}
