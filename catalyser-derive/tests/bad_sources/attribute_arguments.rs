use catalyser_derive::overloads;

#[overloads(foo)]
fn f(#[default(1)] a: u8) -> u8 {
    a
}

fn main() {}
