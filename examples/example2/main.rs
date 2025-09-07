// Declare the modules
pub mod a;
pub mod b;
pub mod c;

// Use items from the modules
use c::C;
use crate::a::AMethods;
use crate::b::BMethods;

fn main() {
    let c = C::new(16, 32, "c");
    c.method_a();
    c.method_b();
    println!("C's fields: a = {}, b = {}, c = {}", c.field_a, c.field_b, c.field_c);
}
