use catalyser_derive::overloads;

#[overloads]
fn f(#[default(1)] #[default(2)] a: u8) -> u8 {
    a
}

fn main() {}
