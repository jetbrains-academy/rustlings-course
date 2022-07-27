## Bringing Paths into Scope with the use Keyword

It might seem like the paths we’ve written to call functions so far are inconveniently long and repetitive. For example, in one of the examples from the "Paths for Referring to an Item in the Module Tree", whether we chose the absolute or relative path to the `add_to_waitlist` function, every time we wanted to call `add_to_waitlist` we had to specify `front_of_house` and `hosting` too. Fortunately, there’s a way to simplify this process. We can bring a path into a scope once and then call the items in that path as if they’re local items with the `use` keyword.

In the example below, we bring the `crate::front_of_house::hosting` module into the scope of the `eat_at_restaurant` function so we only have to specify `hosting::add_to_waitlist` to call the `add_to_waitlist` function in `eat_at_restaurant`.

```rust
    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}
        }
    }

    use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
    }
```

##### Bringing a module into scope with use

Adding `use` and a path in a scope is similar to creating a symbolic link in the filesystem. By adding `use crate::front_of_house::hosting` in the crate root, `hosting` is now a valid name in that scope, just as though the `hosting` module had been defined in the crate root. Paths brought into scope with `use` also check privacy, like any other paths.

You can also bring an item into scope with `use`  and a relative path. The example below shows how to specify a relative path to get the same behavior as in the code above.

```rust
    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}
        }
    }

    use self::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
    }
```

##### Bringing a module into scope with use and a relative path starting with self

Note that using `self` in this way might not be necessary in the future; it’s an inconsistency in the language that Rust developers are working to eliminate.

### Creating Idiomatic use Paths

In the first code snippet, you might have wondered why we specified `use crate::front_of_house::hosting` and then called `hosting::add_to_waitlist` in `eat_at_restaurant` rather than specifying the `use` path all the way out to the `add_to_waitlist` function to achieve the same result, as in the snippet below.

```rust
    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}
        }
    }

    use crate::front_of_house::hosting::add_to_waitlist;

    pub fn eat_at_restaurant() {
        add_to_waitlist();
        add_to_waitlist();
        add_to_waitlist();
    }
```

##### Bringing the add_to_waitlist function into scope with use, which is unidiomatic

Although both snippets accomplish the same task, the first one is the idiomatic way to bring a function into scope with `use`. Bringing the function’s parent module into scope with `use` so we have to specify the parent module when calling the function makes it clear that the function isn’t locally defined while still minimizing repetition of the full path. The code in the last snippet is unclear as to where `add_to_waitlist` is defined.

On the other hand, when bringing in structs, enums, and other items with `use`, it’s idiomatic to specify the full path. The example below shows the idiomatic way to bring the standard library’s `HashMap` struct into the scope of a binary crate.

```rust
    use std::collections::HashMap;

    fn main() {
        let mut map = HashMap::new();
        map.insert(1, 2);
    }
```

##### Bringing HashMap into scope in an idiomatic way

There’s no strong reason behind this idiom: it’s just the convention that has emerged, and folks have gotten used to reading and writing Rust code this way.

The exception to this idiom is if we’re bringing two items with the same name into scope with `use` statements, because Rust doesn’t allow that. Listing The example below shows how to bring two `Result` types into scope that have the same name but different parent modules and how to refer to them.

```rust
    use std::fmt;
    use std::io;

    fn function1() -> fmt::Result {
    }

    fn function2() -> io::Result<()> {
    }
```

#####  Bringing two types with the same name into the same scope requires using their parent modules.

As you can see, using the parent modules distinguishes the two `Result` types. If instead we specified `use std::fmt::Result` and `use std::io::Result`, we’d have two `Result` types in the same scope and Rust wouldn’t know which one we meant when we used `Result`. Try it and see what compiler error you get!

