use app::bind_fields;

bind_fields! {
    pub struct A {
        pub field_a: i32,
        pub field_b: i32,
    }

    impl A {
       pub fn no_receiver() {
            // This should fail to compile because `field_a` and `field_b` are
            // not in scope. The macro only injects bindings into methods with
            // a `self` receiver.
            println!("no_receiver called: a = {}, b = {}", field_a, field_b);
        }
    }
}

fn main() {}
