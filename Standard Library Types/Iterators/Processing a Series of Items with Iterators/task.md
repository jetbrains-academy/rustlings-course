## Processing a Series of Items with Iterators

The iterator pattern allows you to perform some task on a sequence of items in turn. An iterator is responsible for the logic of iterating over each item and determining when the sequence has finished. When you use iterators, you don’t have to reimplement that logic yourself.

In Rust, iterators are _lazy_, meaning they have no effect until you call methods that consume the iterator to use it up. For example, the code in the snippet below creates an iterator over the items in the vector `v1` by calling the `iter` method defined on `Vec<T>`. This code by itself doesn’t do anything useful.

```rust
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();
```

##### Creating an iterator

Once we’ve created an iterator, we can use it in a variety of ways. In "Common Programming Concepts/If", we used iterators with `for` loops to execute some code on each item, although we glossed over what the call to `iter` did until now.

The example in the next code snippet separates the creation of the iterator from the use of the iterator in the `for` loop. The iterator is stored in the `v1_iter` variable, and no iteration takes place at that time. When the `for` loop is called using the iterator in `v1_iter`, each element in the iterator is used in one iteration of the loop, which prints out each value.

```rust
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
```

##### Using an iterator in a for loop

In languages that don’t have iterators provided by their standard libraries, you would likely write this same functionality by starting a variable at index 0, using that variable to index into the vector to get a value, and incrementing the variable value in a loop until it reached the total number of items in the vector.

Iterators handle all that logic for you, cutting down on repetitive code you could potentially mess up. Iterators give you more flexibility to use the same logic with many different kinds of sequences, not just data structures you can index into, like vectors. Let’s examine how iterators do that.





