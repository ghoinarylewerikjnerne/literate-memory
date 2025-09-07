use app::bind_fields;

bind_fields! {
    pub struct A {
        pub field_a: i32,
        pub field_b: i32,
    }

    // This is a free-standing function, not part of the impl block.
    // The macro should ignore it.
    pub fn non_assoc_method(a: &A) {
        println!("non_assoc_method called with a.field_a = {}", a.field_a);
    }

    impl A {
        pub fn method_a(&self) {
            println!("method_a called: a = {}, b = {}", field_a, field_b);
        }

        pub fn consume(self) {
            println!("consumed: a = {}, b = {}", field_a, field_b);
        }

        pub fn no_receiver() {
            // unchanged — no receiver, so nothing injected
        }

        pub fn method_a2(&self) {
            // This call should be rewritten to `self.method_a()` by the macro.
            method_a();

            // This call to a free function should be ignored by the macro.
            non_assoc_method(self);

            // This call to an associated function should be ignored by the macro
            // because `method_a3` does not have a `self` receiver. It must be
            // called with `Self::`.
            Self::method_a3();
        }

        pub fn method_a3() {
            println!("method_a3 called");
        }
    }
}

fn main() {
    let a = A { field_a: 1, field_b: 2 };
    a.method_a2();
    a.consume();
}
