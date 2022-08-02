## Duplicated Code Fragments

Before diving into generics syntax, let’s first look at how to remove
duplication that doesn’t involve generic types by extracting a function. Then
we’ll apply this technique to extract a generic function! In the same way that
you recognize duplicated code to extract into a function, you’ll start to
recognize duplicated code that can use generics.

To find the largest number in two different lists of numbers, we can duplicate
the code above and use the same logic at two different places in the
program, as shown below.


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

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}
```

### Code to find the largest number in *two* lists of numbers

Although this code works, duplicating code is tedious and error prone. We also
have to update the code in multiple places when we want to change it.