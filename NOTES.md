# Notes

- [`cargo`](https://doc.rust-lang.org/cargo/index.html) is a command line tool for building, testing and dependency management for Rust.
  It uses `Cargo.toml` as the configuration file.
- Pretty much every rust project uses cargo.
- Cargo projects usually have a simple structure: `src` for the source code, `target` for the compiled binaries, `tests` for the tests and `Cargo.toml` for the project configuration.
- Variables are unmutable by default. (safety, concurrency, speed)
- `const` variables should be screaming snake case.
- It allows for variable shadowing. We can shadow not only values but types too.
- Rust compiler is one of its biggest strengths. It tries to avoid a lof of runtime issues other languages have. The error messages are also very helpful and intuitive. For every error there's usually a suggestion.
- Rust doesn't have a GC (garbage collector). Instead it knows when a variable goes out of scope at compile time and inserts the necessary LLVM/assembly instructions to free the memory.
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
- `if`'s are expressions (they can return a value):
  ```rust
  num = if true { 1 } else { 0 };
  ```
- `loop` (infinite loop) can have labels to allow break and continue on nested loops.
  ```rust
  'outer:loop {
    loop {
      break 'outer;
    }
  }
  ```
- The `.` (dot) operator auto dereferences pointers.
- Manually dereferencing a pointer:
  ```rust
  (*p).method(); // this is optional
  *s = String::from("hello"); // when (re)assigning/using the pointer value we need to manually dereference it
  ```
- References: you can have either exactly one mutable reference or any number of immutable references.
- A closure has access to variables in the scope where it’s defined (pretty much like in any other language).
- The closest thing to a class in rust is a `struct`.
- When initializing a struct you need to specify all fields.
- `Self` is the type of the current object.
- `self` is syntax sugar for `self: Self` in function signatures. `&self` is `self: &Self` and `&mut self` is `self: &mut Self`. It can only be used as the first argument in a function signature.
- There's no struct inheritance in rust.
- Traits are similar to interfaces in other languages.
- Traits can inherit from other traits.
- When implementing a trait you need to implement all the methods of the trait.
- When implementing a trait you need to implement all parent traits.
- Traits can have default implementations.
- There's no fields in traits (at least yet). An alternative is to use getters and setters.
