use catalyser_derive::overloads;

struct S;
impl S {
    #[overloads]
    fn m(&self, #[default(1)] a: u8) -> u8 {
        a
    }
}

fn main() {}
