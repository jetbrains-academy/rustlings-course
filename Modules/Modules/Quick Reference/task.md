## Modules Quick Reference
Here’s how modules, paths, the `use` keyword, and the `pub` keyword work in the compiler, and how most developers organize their code. We’ll be going through examples of each of these rules, but this is a great place to look in the future as a reminder of how modules work.

- **Start from the crate root**: When compiling a crate, the compiler first looks in the crate root file (usually _src/lib.rs_ for a library crate or _src/main.rs_ for a binary crate).
- **Declaring modules**: In the crate root file, you can declare a new module named, say, “garden”, with `mod garden;`. The compiler will look for the code inside the module in these places:
  - Inline, directly following `mod garden`, within curly brackets instead of the semicolon
  - In the file _src/garden.rs_
  - In the file _src/garden/mod.rs_
- **Declaring submodules**: In any file other than the crate root that’s being compiled as part of the crate (for example, _src/garden.rs_), you can declare submodules (for example, `mod vegetables;`). The compiler will look for the code inside submodules in these places within a directory named for the parent module:
  - Inline, directly following `mod vegetables`, within curly brackets instead of the semicolon
  - In the file _src/garden/vegetables.rs_
  - In the file _src/garden/vegetables/mod.rs_
- **Paths to code in modules**: Once a module is being compiled as part of your crate, you can refer to code in that module (for example, an `Asparagus` type in the garden vegetables module) from anywhere else in this crate by using the path `crate::garden::vegetables::Asparagus` as long as the privacy rules allow.
- **Private vs public**: Code within a module is private from its parent modules by default. To make a module public, declare it with `pub mod` instead of `mod`. To make items within a public module public as well, use `pub` before their declarations.
- **The `use` keyword**: Within a scope, the `use` keyword creates shortcuts to items to reduce repetition of long paths. In any scope that can refer to `crate::garden::vegetables::Asparagus`, you can create a shortcut with `use crate::garden::vegetables::Asparagus;` and then only need to write `Asparagus` to make use of that type in the scope.

Here’s a binary crate named `backyard` that illustrates these rules. The crate’s directory, also named `backyard`, contains these files and directories:

```text
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```

The crate root file, in this case _src/main.rs_, contains:

Filename: src/main.rs

```rust
use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
```

The `pub mod garden;` means the compiler includes the code it finds in _src/garden.rs_, which is:

Filename: src/garden.rs

```rust
pub mod vegetables;
```

And `pub mod vegetables;` means the code in _src/garden/vegetables.rs_ is included too:

```rust
#[derive(Debug)]
pub struct Asparagus {}
```

Now let’s get into the details of these rules and demonstrate them in action!