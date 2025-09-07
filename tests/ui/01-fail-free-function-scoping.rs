use app::bind_fields;

bind_fields! {
    pub struct A {
        pub field_a: i32,
        pub field_b: i32,
    }

    // This is a free-standing function. The macro should not affect it.
    pub fn non_assoc_method(a: &A) {
        // This should fail to compile because `field_a` is not in scope.
        // It must be accessed via `a.field_a`.
        println!("non_assoc_method called with a.field_a = {}", field_a);
    }

    impl A {
        // ... impl block needed for macro to be valid ...
    }
}

fn main() {}
