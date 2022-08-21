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
- Tail expression (return omitted):
  ```rust
  {return true}
  // is the same as
  {true}
  ```
- No support for named arguments at the moment, so arguments need to be supplied in the order they are defined in the function signature.
- Cargo has a built in linter that checks for common mistakes: `cargo clippy`.
- [crates.io](https://crates.io/) is Rust's package registry.
- Package dependencies are defined in the `[dependencies]` section of `Cargo.toml`.
- Integer literals are 32 bit (i32) by default.
- Floating point literals are 64 bit (f64) by default. It can also be very slow on 32 bit systems.
- A character (`char`) is always 4 bytes long (32 bits).
- Tuples have a limit of 12 elements (or at least if you intend to use their full functionality).
- Arrays are fixed size and live on the stack by default.