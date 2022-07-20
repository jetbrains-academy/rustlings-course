## Defining Modules to Control Scope and Privacy

_Modules_ let us organize code within a crate into groups for readability and easy reuse. Modules also control the _privacy_ of items, which is whether an item can be used by outside code (_public_) or is an internal implementation detail and not available for outside use (_private_).

As an example, let’s write a library crate that provides the functionality of a restaurant. We’ll define the signatures of functions but leave their bodies empty to concentrate on the organization of the code, rather than actually implement a restaurant in code.

In the restaurant industry, some parts of a restaurant are referred to as _front of house_ and others as _back of house_. Front of house is where customers are; this is where hosts seat customers, servers take orders and payment, and bartenders make drinks. Back of house is where the chefs and cooks work in the kitchen, dishwashers clean up, and managers do administrative work.

To structure our crate in the same way that a real restaurant works, we can organize the functions into nested modules. Create a new library named `restaurant` by running `cargo new --lib restaurant`; then put the code from the example below into _src/lib.rs_ to define some modules and function signatures.

```rust
    mod front_of_house {
        mod hosting {
            fn add_to_waitlist() {}

            fn seat_at_table() {}
        }

        mod serving {
            fn take_order() {}

            fn serve_order() {}

            fn take_payment() {}
        }
    }
```

##### A front_of_house module containing other modules that then contain functions

We define a module by starting with the `mod` keyword, and then specify the name of the module (in this case, `front_of_house`) and place curly brackets around the body of the module. Inside modules, we can have other modules, as in this case with the modules `hosting` and `serving`. Modules can also hold definitions for other items, such as structs, enums, constants, traits, or as in the code snippet above, functions.

By using modules, we can group related definitions together and name why they’re related. Programmers using this code would have an easier time finding the definitions they wanted to use because they could navigate the code based on the groups rather than having to read through all the definitions. Programmers adding new functionality to this code would know where to place the code to keep the program organized.

Earlier, we mentioned that _src/main.rs_ and _src/lib.rs_ are called _crate roots_. The reason for their name is that the contents of either of these two files form a module named `crate` at the root of the crate’s module structure, known as the _module tree_.

Below is the module tree for the structure in the snippet above.

    crate
     └── front_of_house
         ├── hosting
         │   ├── add_to_waitlist
         │   └── seat_at_table
         └── serving
             ├── take_order
             ├── serve_order
             └── take_payment

##### The module tree

This tree shows how some of the modules nest inside one another (for example, `hosting` nests inside `front_of_house`). The tree also shows that some modules are _siblings_ to each other, meaning they’re defined in the same module (`hosting` and `serving` are defined within `front_of_house`). To continue the family metaphor, if module A is contained inside module B, we say that module A is the _child_ of module B, and that module B is the _parent_ of module A. Notice that the entire module tree is rooted under the implicit module named `crate`.

The module tree might remind you of the filesystem’s directory tree on your computer; this is a very apt comparison! Just like directories in a filesystem, you use modules to organize your code. And just like files in a directory, we need a way to find our modules.

_You can refer to the following chapter in the Rust Programming Language Book: [Defining Modules to Control Scope and Privacy](https://doc.rust-lang.org/stable/book/ch07-02-defining-modules-to-control-scope-and-privacy.html)_