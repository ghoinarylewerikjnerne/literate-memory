use app::bind_fields;

bind_fields! {
    pub struct A {
        pub field_a: i32,
        pub field_b: i32,
    }

    impl A {
        pub fn method_a(&self) {
            // The `bind_fields!` macro injects a `let` binding for field_a and
            // field_b here. They are unused in this method body. The macro
            // also injects `#[allow(unused_variables)]`, so this should compile
            // without warnings.
        }
    }
}

fn main() {
    let a = A { field_a: 1, field_b: 2 };
    a.method_a();
}
