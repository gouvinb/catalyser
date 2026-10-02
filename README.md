# Catalyser

**Catalyser** is a comprehensive collection of extensions designed to enhance
Rust's  standard library and other critical libraries, offering tools to
streamline common tasks and improve development efficiency. This library aims to
simplify development, improve expressiveness, and increase productivity while
maintaining Rust's idiomatic principles.

## Installation

Add the following dependencies to your Cargo.toml:

```toml
[dependencies]
catalyser = { version = "x.y.z", features = ["serde"] }
```

Replace `"x.y.z"` with the latest version of the crate.

### Features

- `serde`: Enhance common serialization and
  deserialization tasks by introducing new types, such as `NonEmptyString`,
  `BoundedI32`, and others, to streamline data handling and ensure type safety.
- Default and named parameters (`derive` feature): Rust has no default or named
  arguments. `#[overloads]` adds both to free functions, Kotlin-style:

  ```rust
  use catalyser::overloads;
  use std::time::Duration::{self, from_secs};
  
  #[overloads]
  pub fn connect(
      host: &str,
      #[default(5432)] port: u16,
      #[default(from_secs(5))] timeout: Duration,
  ) -> String {
      format!("{host}:{port} ({}s)", timeout.as_secs())
  }
  
  pub fn main() {
      // the plain function still works
      connect("localhost", 5432, from_secs(5));

      // `use` imports the function *and* its call macro.
      connect!("localhost");       // overloads for port and timeout
      connect!("localhost", 1234); // positional

      // with any order
      connect!(timeout = from_secs(1), host = "localhost");
      connect!("localhost", timeout = from_secs(1));
      connect!(timeout = from_secs(1), "localhost");
  }
  ```
  - A default is any expression. It is evaluated where the function is defined,
    so it can use that module's imports and the parameters declared before it
    (`#[default(http + 363)] https: u16`).
  - Positional arguments come first, then named ones. Misuse (missing, unknown,
    duplicated or too many arguments) is a compile error.
  - **Limitations**
    - Free functions only: no methods (`self`), `const fn`, `unsafe fn`,
      `extern` functions or `impl Trait` parameters with a default (use a named
      generic instead).
    - The call macro is crate-local (`pub(crate)`), so it cannot be used from
      another crate.
    - Import the function by name: `use path::connect as other;` breaks the call
      macro.
    - Arguments are evaluated in declaration order, not in the order written at
      the call site.

**Alternatives.** If you prefer the builder pattern,
[`bon`](https://crates.io/crates/bon) is an excellent, mature choice
(`#[builder]` on functions and structs, with `Option<T>` and
`#[builder(default)]` support). Catalyser targets a different ergonomic: a
Kotlin-like call syntax, `f!(a, b = 2)`.

## Contributing

Contributions are welcome! To get started:

1. Fork the repository.
2. Create a branch for your changes: `git checkout -b feature/my-new-feature`.
3. Make your changes and commit them: `git commit -m 'Add a new feature'`.
4. Push your changes: `git push origin feature/my-new-feature`.
5. Open a pull request.

### Project Structure

This project is organized as a workspace with the following two crates:

- `catalyser`: Contains the main extensions
    - `src/<module or module.rs>` : Contains the main extensions for a dedicated
      crate
- `catalyser-derive`: Contains custom derive macros for additional features
    - `src/<module or module.rs>` : Contains custom derive macros for a
      dedicated crate

## License

See the [LICENSE.md](LICENSE.md) file for more details.

## Acknowledgements

This library draws inspiration from Kotlin's KTX approach, which provides
extensions to simplify and extend standard functionality by adding concise and
expressive APIs. Similarly, **Catalyser** aims to enrich Rust development with
tools that improve usability and reduce boilerplate, while staying idiomatic to
the language.
