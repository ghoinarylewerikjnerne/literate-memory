use BindFields, bind_methods;

#[derive(BindFields)]
pub struct A {
    pub field_a: i32,
    pub field_b: i32,
}

#[bind_methods]
impl A {
    pub fn method_a(&self) {
        // the impl-attribute injected a call to the generated macro:
        // __bind_fields_for_A!(&self);
        // i.e. `let A { field_a, field_b } = *self;`
        println!("a = {}, b = {}", field_a, field_b);
    }

    pub fn consume(self) {
        // injected: __bind_fields_for_A!(self)
        println!("consumed: a = {}, b = {}", field_a, field_b);
    }

    pub fn no_receiver() {
        // unchanged — no receiver, so nothing injected
    }
}

fn main() {
    let a = A { field_a: 1, field_b: 2 };
    a.method_a();
    a.consume();
}
