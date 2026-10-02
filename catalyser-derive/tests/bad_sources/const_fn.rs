use catalyser_derive::overloads;

#[overloads]
const fn f(#[default(1)] a: u8) -> u8 {
    a
}

fn main() {}
