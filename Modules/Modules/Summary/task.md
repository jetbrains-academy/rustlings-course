## Summary

Rust lets you split a package into multiple crates and a crate into modules so you can refer to items defined in one module from another module. You can do this by specifying absolute or relative paths. These paths can be brought into scope with a `use` statement so you can use a shorter path for multiple uses of the item in that scope. Module code is private by default, but you can make definitions public by adding the `pub` keyword.

_You can refer to the following chapter in the Rust Programming Language Book: [Separating Modules into Different Files](https://doc.rust-lang.org/stable/book/ch07-05-separating-modules-into-different-files.html#separating-modules-into-different-files)_