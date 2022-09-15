### Re-exporting Names with `pub use`

When we bring a name into scope with the `use` keyword, the name available in the new scope is private. In order to be able to refer to that name from some other code (as if the name has been defined in that code’s scope), we can combine `pub` and `use`.
This technique is called _re-exporting_ because we’re bringing an item into scope but also making that item available for others to bring into their scope.

The next example shows the code from the beginning of the task with `use` in the root module changed to `pub use`.

```rust
    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}
        }
    }

    pub use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
    }
```

##### Making a name available for any code to use from a new scope with pub use

By using `pub use`, external code can now call the `add_to_waitlist` function using `hosting::add_to_waitlist`. If we hadn’t specified `pub use`, the `eat_at_restaurant` function could call `hosting::add_to_waitlist` in its scope but external code couldn’t take advantage of this new path.

Re-exporting is useful when the internal structure of your code is different from how programmers calling your code would think about the domain. For example, in this restaurant metaphor, the people running the restaurant think about “front of house” and “back of house.” But customers visiting a restaurant probably won’t think about the parts of the restaurant in those terms. With `pub use`, we can write our code with one structure but expose a different structure. Doing so makes our library well organized for programmers working on the library and programmers calling the library.
