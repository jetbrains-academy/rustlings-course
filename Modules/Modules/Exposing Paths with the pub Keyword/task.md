### Exposing Paths with the pub Keyword

Let’s return to the error in the previous example that told us the `hosting` module is private. We want the `eat_at_restaurant` function in the parent module to have access to the `add_to_waitlist` function in the child module, so we mark the `hosting` module with the `pub` keyword as shown in the listing below.

```rust
    mod front_of_house {
        pub mod hosting {
            fn add_to_waitlist() {}
        }
    }

    pub fn eat_at_restaurant() {
        // Absolute path
        crate::front_of_house::hosting::add_to_waitlist();

        // Relative path
        front_of_house::hosting::add_to_waitlist();
    }
```

##### Declaring the hosting module as pub to use it from eat_at_restaurant

Unfortunately, the code from the snippet above still results in an error as shown below.

```text
Compiling Test_Rust_Project v0.1.0
error[E0603]: function `add_to_waitlist` is private
 --> src/main.rs:9:37
  |
9 |     crate::front_of_house::hosting::add_to_waitlist();
  |                                     ^^^^^^^^^^^^^^^

error[E0603]: function `add_to_waitlist` is private
  --> src/main.rs:12:30
   |
12 |     front_of_house::hosting::add_to_waitlist();
   |                              ^^^^^^^^^^^^^^^                            ^^^^^^^^^^^^^^^
```

##### Compiler errors from building the code above

What happened? Adding the `pub` keyword in front of `mod hosting` makes the module public. With this change, if we can access `front_of_house`, we can access `hosting`. But the _contents_ of `hosting` are still private; making the module public doesn’t make its contents public. The `pub` keyword on a module only lets code in its ancestor modules refer to it.

The errors say that the `add_to_waitlist` function is private. The privacy rules apply to structs, enums, functions, and methods as well as modules.

Let’s also make the `add_to_waitlist` function public by adding the `pub` keyword before its definition as shown below.

```rust
    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}
        }
    }

    pub fn eat_at_restaurant() {
        // Absolute path
        crate::front_of_house::hosting::add_to_waitlist();

        // Relative path
        front_of_house::hosting::add_to_waitlist();
    }
```

##### Adding the pub keyword to mod hosting and fn add_to_waitlist lets us call the function from eat_at_restaurant

Now the code will compile! Let’s look at the absolute and the relative path and double-check why adding the `pub` keyword lets us use these paths in `add_to_waitlist` with respect to the privacy rules.

In the absolute path, we start with `crate`, the root of our crate’s module tree. Then the `front_of_house` module is defined in the crate root. The `front_of_house` module isn’t public, but because the `eat_at_restaurant` function is defined in the same module as `front_of_house` (that is, `eat_at_restaurant` and `front_of_house` are siblings), we can refer to `front_of_house` from `eat_at_restaurant`. Next is the `hosting` module marked with `pub`. We can access the parent module of `hosting`, so we can access `hosting`. Finally, the `add_to_waitlist` function is marked with `pub` and we can access its parent module, so this function call works!

In the relative path, the logic is the same as in the absolute path except for the first step: rather than starting from the crate root, the path starts from `front_of_house`. The `front_of_house` module is defined within the same module as `eat_at_restaurant`, so the relative path starting from the module in which `eat_at_restaurant` is defined works. Then, because `hosting` and `add_to_waitlist` are marked with `pub`, the rest of the path works and this function call is valid!

