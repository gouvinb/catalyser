use catalyser_derive::overloads;

#[overloads]
fn f((a, b): (u8, u8)) -> u8 {
    a + b
}

fn main() {}
