## Code without Duplications

In the next code snippet, we extracted the code that finds the largest number into a
function named `largest`. Unlike the code in the first listing of this section, which can find the
largest number in only one particular list, this program can find the largest
number in two different lists.


```rust
fn largest(list: &[i32]) -> &i32 {
  let mut largest = &list[0];

  for item in list {
    if item > largest {
      largest = item;
    }
  }

  largest
}

fn main() {
  let number_list = vec![34, 50, 25, 100, 65];
  let result = largest(&number_list);
  println!("The largest number is {}", result);

  let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
  let result = largest(&number_list);
  println!("The largest number is {}", result);
}
```

#### Abstracted code to find the largest number in two lists

The `largest` function has a parameter called `list`, which represents any
concrete slice of `i32` values that we might pass into the function. As a
result, when we call the function, the code runs on the specific values that we
pass in.

In sum, here are the steps we took to change the code from the second to the third listing:

1. Identify duplicate code.
2. Extract the duplicate code into the body of the function and specify the
   inputs and return values of that code in the function signature.
3. Update the two instances of duplicated code to call the function instead.

Next, we’ll use these same steps with generics to reduce code duplication in
different ways. In the same way that the function body can operate on an
abstract `list` instead of specific values, generics allow code to operate on
abstract types.

For example, say we had two functions: one that finds the largest item in a
slice of `i32` values and one that finds the largest item in a slice of `char`
values. How would we eliminate that duplication? Let’s find out!
