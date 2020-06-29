# rust

[Summer 2020] Learning Rust

Following [The Rust Programming Languague](https://doc.rust-lang.org/book/)

## Chapter 14: Cargo & Crate.io

### Release Profiles

1. `dev` profile: `cargo build`
2. `release` profile: `cargo build --release`

Could customise different profile setting by adding to `Cargo.toml`.

#### Example

`opt-level` is number of optimizations applied (0-3)

Default values:

```rust
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

### Documentation

+ Useful documentation comments (`//` for normal comment, `///` or `//!` for _documentation comment_ that support HTML Markdown)
+ Documentation sections: example, panics, errors, safety
+ Generate documentation by `cargo doc (--open)`
+ Code under `#Example` section will be run during `cargo test`, so need to ensure documentation example codes are updated

#### Example: function documentation (`///`)

```rust
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

#### Example: module/crate documentation (`//!`)

```rust
//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
// --snip--
```

### Export API structure

+ To avoid deeply nested structure that is inconvenient for users
+ Users can call re-exported items directly from the crate, rather than navigating through the nested path

#### Example: re-export items at the top level

```rust
//! # Art
//!
//! A library for modeling artistic concepts.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    // --snip--
}

pub mod utils {
    // --snip--
}
```

### Publish Crate to Crates.io

1. Set up Crates.io account
   1. Create account on crates.io
   2. `cargo login [API key]`
2. Add Metadata to new crate

    ```rust
    [package]
    name = "guessing_game"
    version = "0.1.0"
    authors = ["Your Name <you@example.com>"]
    edition = "2018"
    description = "A fun game where you guess what number the computer has chosen."
    license = "MIT OR Apache-2.0"
    ```

3. Publish new crate: `cargo publish`
4. Publish new versions of exisiting crate:
   1. Change `version` value in `Cargo.toml` (following **_Semantic Versioning rules_**)
   2. `cargo publish`
5. Remove versions: can only prevent future projects from adding as dependency, cannot delete
   1. `cargo yank --vers 1.0.1` to remove
   2. `cargo yank --vers 1.0.1 --undo` to undo

### Cargo Workspaces

#### Create workspace

```shell
mkdir add
cd add
```

Filename: Cargo.toml

```rust
[workspace]
members = [
    "adder",
    "add-one",
]
```

Create binary and library crates similar to normal:

+ `cargo new adder`
+ `cargo new add-one --lib`
+ update `adder/Cargo.toml`:

    ```rust
    [dependencies]

    add-one = { path = "../add-one" }
    ```

+ import `add-one` in `adder/src/main.rs` to use
+ `cargo build`
+ `cargo run -p adder` to run
+ same imported dependencies from different crates will be collated, ensured same and added **once** to the root `Cargo.lock`
+ add tests to crates and run like normal (`cargo test` or `cargo test -p add-one` for specific crate)
+ each crates need to be published separately to crates.io

### Install Binaries

+ to install tools that other developers have shared on crates.io
+ can only install packages with a binary target (`src/main.rs`), rather than a library target that cannot run on its own
+ `cargo install binary-name`
+ installed binaries are stored at `./.cargo/bin` (must include in `$PATH` in order to run after installation)

### Custom Commands

+ if a binary in `$PATH` is named `cargo-something`, can run with `cargo something`
+ to list custom commands: `cargo --list`
