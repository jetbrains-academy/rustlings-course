## Looping Through an Array with for

You could use the `while` construct to loop over the elements of a collection, such as an array. For example, let’s look at the following code:

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
```
##### Example of looping through each element of a collection using a while loop

Here, the code counts up through the elements in the array. It starts at index `0`, and then loops until it reaches the final index in the array (that is, when `index < 5` is no longer true). Running this code will print every element in the array:

```text
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32s
     Running `target/debug/loops`
the value is: 10
the value is: 20
the value is: 30
the value is: 40
the value is: 50
```

All five array values appear in the terminal, as expected. Even though `index` will reach a value of `5` at some point, the loop stops executing before trying to fetch a sixth value from the array.

But this approach is error-prone; we could cause the program to panic if the index length is incorrect. It’s also slow because the compiler adds runtime code to perform the conditional check on every element on every iteration through the loop.

As a more concise alternative, you can use a `for` loop and execute some code for each item in a collection. A for loop looks like this:

```rust
fn main() {
    let arr = [10, 20, 30, 40, 50];

    for element in arr {
        println!("the value is: {}", element);
    }
}
```
##### Example of looping through each element of a collection using a for loop

When we run this code, we’ll see the same output as in the previous code snippet. More importantly, we’ve now increased the safety of the code and eliminated the chance of bugs that might result from going beyond the end of the array or not going far enough and missing some items.

For example, in the previous code snippet, if you changed the definition of the `a` array to have four elements but forgot to update the condition to `while index < 4`, the code would panic. Using the `for` loop, you wouldn’t need to remember to change any other code if you changed the number of values in the array.

