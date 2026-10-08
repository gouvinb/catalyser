use catalyser_derive::overloads;

#[overloads]
fn connect(host: &str, #[default(5432)] port: u16, #[default(1)] retries: u8) -> String {
    format!("{host}:{port}/{retries}")
}

fn main() {
    let _ = connect!("h", port = 1, port = 2);
}
