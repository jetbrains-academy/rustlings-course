## Computing the Largest Number

Consider a short program that finds the largest number in a list, as shown in
the code snippet below.

```rust
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}
```

#### Code to find the largest number in a list of numbers.

This code stores a list of integers in the variable `number_list` and places
the first number in the list in a variable named `largest`. Then it iterates
through all the numbers in the list, and if the current number is greater than
the number stored in `largest`, it replaces the number in that variable.
However, if the current number is less than or equal to the largest number seen
so far, the variable doesn’t change, and the code moves on to the next number
in the list. After considering all the numbers in the list, `largest` should
hold the largest number, which in this case is 100.