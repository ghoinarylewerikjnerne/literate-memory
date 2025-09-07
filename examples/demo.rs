use app::bind_fields;

bind_fields! {
    pub struct A {
        pub field_a: i32,
        pub field_b: i32,
    }

    impl A {
        pub fn method_a(&self) {
            println!("a = {}, b = {}", field_a, field_b);
        }

        pub fn consume(self) {
            println!("consumed: a = {}, b = {}", field_a, field_b);
        }

        pub fn no_receiver() {
            // unchanged — no receiver, so nothing injected
        }
    }
}

fn main() {
    let a = A { field_a: 1, field_b: 2 };
    a.method_a();
    a.consume();
}
