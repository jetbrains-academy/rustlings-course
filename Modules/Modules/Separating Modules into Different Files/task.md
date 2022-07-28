## Separating Modules into Different Files

So far, all the examples in this chapter defined multiple modules in one file. When modules get large, you might want to move their definitions to a separate file to make the code easier to navigate.

For example, let’s start from the code in one of the previous tasks and move the `front_of_house` module to its own file _src/front_of_house.rs_ by changing the crate root file so it contains the code shown below. In this case, the crate root file is _src/lib.rs_, but this procedure also works with binary crates whose crate root file is _src/main.rs_.

```rust
    mod front_of_house;

    pub use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
    }
```

##### Declaring the front_of_house module whose body will be in _src/front_of_house.rs_

And _src/front_of_house.rs_ gets the definitions from the body of the `front_of_house` module, as shown below.

<span class="filename">Filename: src/front_of_house.rs</span>

```rust
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
```

##### Definitions inside the front_of_house module in _src/front_of_house.rs_

Using a semicolon after `mod front_of_house` rather than using a block tells Rust to load the contents of the module from another file with the same name as the module. To continue with our example and extract the `hosting` module to its own file as well, we change _src/front_of_house.rs_ to contain only the declaration of the `hosting` module:

```rust
    pub mod hosting;
```

Then we create a _src/front_of_house_ directory and a file _src/front_of_house/hosting.rs_ to contain the definitions made in the `hosting` module:

```rust
    pub fn add_to_waitlist() {}
```

The module tree remains the same, and the function calls in `eat_at_restaurant` will work without any modification, even though the definitions live in different files. This technique lets you move modules to new files as they grow in size.

Note that the `pub use crate::front_of_house::hosting` statement in _src/lib.rs_ also hasn’t changed, nor does `use` have any impact on what files are compiled as part of the crate. The `mod` keyword declares modules, and Rust looks in a file with the same name as the module for the code that goes into that module.

