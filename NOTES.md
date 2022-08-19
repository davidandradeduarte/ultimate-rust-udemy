# Notes

- [`cargo`](https://doc.rust-lang.org/cargo/index.html) is a command line tool for building, testing and dependency management for Rust.
  It uses `Cargo.toml` as the configuration file.
- Pretty much every rust project uses cargo.
- Cargo projects usually have a simple structure: `src` for the source code, `target` for the compiled binaries, `tests` for the tests and `Cargo.toml` for the project configuration.
- Variables are unmutable by default. (safety, concurrency, speed)
- Compile error messages and suggestions are really helpful.
- `const` variables should be screaming snake case.
- Variable shadowing is a thing. We can shadow not only variable values but also variable types.
- The compiler wont let you use variables that are possibily not initialized.
- tail expression (return omitted):
  ```rust
  {return true}
  // is the same as
  {true}
  ```
- no support for named arguments at the moment, so arguments need to be supplied in the order they are defined in the function signature.
- cargo has a built in linter that checks for common mistakes: `cargo clippy`.
- [crates.io](https://crates.io/) is Rust's package registry.
- package dependencies are defined in the `[dependencies]` section of `Cargo.toml`.