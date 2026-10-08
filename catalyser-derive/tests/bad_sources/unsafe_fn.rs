use catalyser_derive::overloads;

#[overloads]
unsafe fn f(#[default(1)] a: u8) -> u8 {
    a
}

fn main() {}
