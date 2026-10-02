use catalyser_derive::overloads;

#[overloads]
fn f(#[default(1)] a: impl Into<u8>) -> u8 {
    a.into()
}

fn main() {}
