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

[//]: # (- `derive`: Automate recurring tasks with custom derive macros.)
- `serde`: Enhance common serialization and
  deserialization tasks by introducing new types, such as `NonEmptyString`,
  `BoundedI32`, and others, to streamline data handling and ensure type safety.

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
