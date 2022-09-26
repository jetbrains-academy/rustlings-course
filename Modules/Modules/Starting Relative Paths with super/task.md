### Starting Relative Paths with super

We can also construct relative paths that begin in the parent module by using `super` at the start of the path. This is like starting a filesystem path with the `..` syntax. Why would we want to do this?

Consider the code below, which models the situation in which a chef fixes an incorrect order and personally brings it out to the customer. The function `fix_incorrect_order` calls the function `serve_order` by specifying the path to `serve_order` starting with `super`:

```rust
    fn serve_order() {}

    mod back_of_house {
        fn fix_incorrect_order() {
            cook_order();
            super::serve_order();
        }

        fn cook_order() {}
    }
```

##### Calling a function using a relative path starting with super

The `fix_incorrect_order` function is in the `back_of_house` module, so we can use `super` to go to the parent module of `back_of_house`, which in this case is `crate`, the root. From there, we look for `serve_order` and find it. Success! We think the `back_of_house` module and the `serve_order` function are likely to stay in the same relationship to each other and get moved together should we decide to reorganize the crate’s module tree. Therefore, we used `super` so we’ll have fewer places to update code in the future if this code gets moved to a different module.

