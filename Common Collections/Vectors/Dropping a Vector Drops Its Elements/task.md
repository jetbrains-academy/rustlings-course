### Dropping a Vector Drops Its Elements

Like any other `struct`, a vector is freed when it goes out of scope, as
annotated below.

```rust
    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here
```

#### Showing where the vector and its elements are dropped

When the vector gets dropped, all of its contents are also dropped, meaning
those integers it holds will be cleaned up. This may seem like a
straightforward point but can get a bit more complicated when you start to
introduce references to the elements of the vector. Let’s tackle that next!
