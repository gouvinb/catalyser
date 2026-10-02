use catalyser_derive::overloads;

#[overloads]
fn connect(host: &str, #[default(5432)] port: u16, #[default(1)] retries: u8) -> String {
    format!("{host}:{port}/{retries}")
}

fn main() {
    let _ = connect!("h", 1, 2, 3);
}
