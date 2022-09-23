### Lifetime Annotations in Function Signatures

Now let’s examine lifetime annotations in the context of the `longest`
function. As with generic type parameters, we need to declare generic lifetime
parameters inside angle brackets between the function name and the parameter
list. The constraint we want to express in this signature is that all the
references in the parameters and the return value must have the same lifetime.
We’ll name the lifetime `'a` and then add it to each reference, as shown
in the code snippet below.


```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

#### The `longest` function definition specifying that all the references in the signature must have the same lifetime `'a`

This code should compile and produce the result we want when we use it with the
`main` function in the listing where it calls the `longest` function to find the longer of two string slices.

The function signature now tells Rust that for some lifetime `'a`, the function
takes two parameters, both of which are string slices that live at least as
long as lifetime `'a`. The function signature also tells Rust that the string
slice returned from the function will live at least as long as lifetime `'a`.
In practice, it means that the lifetime of the reference returned by the
`longest` function is the same as the smaller of the lifetimes of the
references passed in. These constraints are what we want Rust to enforce.
Remember, when we specify the lifetime parameters in this function signature,
we’re not changing the lifetimes of any values passed in or returned. Rather,
we’re specifying that the borrow checker should reject any values that don’t
adhere to these constraints. Note that the `longest` function doesn’t need to
know exactly how long `x` and `y` will live, only that some scope can be
substituted for `'a` that will satisfy this signature.

When annotating lifetimes in functions, the annotations go in the function
signature, not in the function body. Rust can analyze the code within the
function without any help. However, when a function has references to or from
code outside that function, it becomes almost impossible for Rust to figure out
the lifetimes of the parameters or return values on its own. The lifetimes
might be different each time the function is called. This is why we need to
annotate the lifetimes manually.

When we pass concrete references to `longest`, the concrete lifetime that is
substituted for `'a` is the part of the scope of `x` that overlaps with the
scope of `y`. In other words, the generic lifetime `'a` will get the concrete
lifetime that is equal to the smaller of the lifetimes of `x` and `y`. Because
we’ve annotated the returned reference with the same lifetime parameter `'a`,
the returned reference will also be valid for the length of the smaller of the
lifetimes of `x` and `y`.

Let’s look at how the lifetime annotations restrict the `longest` function by
passing in references that have different concrete lifetimes. The code below is
a straightforward example.


```rust
fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}
```

#### Using the `longest` function with references to `String` values that have different concrete lifetimes

In this example, `string1` is valid until the end of the outer scope, `string2`
is valid until the end of the inner scope, and `result` references something
that is valid until the end of the inner scope. Run this code, and you’ll see
that the borrow checker approves of this code; it will compile and print `The
longest string is long string is long`.

