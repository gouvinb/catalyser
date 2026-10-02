use catalyser_derive::overloads;

#[overloads]
fn connect(#[default(5432)] port: u16, host: &str) {}

fn main() {}
