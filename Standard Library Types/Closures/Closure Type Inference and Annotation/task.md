### Closure Type Inference and Annotation

There are more differences between functions and closures. Closures don’t usually require you to annotate the types of the parameters or the return value like `fn` functions do. Type annotations are required on functions because they’re part of an explicit interface exposed to your users. Defining this interface rigidly is important for ensuring that everyone agrees on what types of values a function uses and returns. But closures aren’t used in an exposed interface like this: they’re stored in variables and used without naming them and exposing them to users of our library.

Closures are typically short and relevant only within a narrow context rather than in any arbitrary scenario. Within these limited contexts, the compiler can infer the types of the parameters and the return type, similar to how it’s able to infer the types of most variables (there are rare cases where the compiler needs closure type annotations too).

As with variables, we can add type annotations if we want to increase explicitness and clarity at the cost of being more verbose than is strictly necessary. Annotating the types for a closure would look like the definition shown below:

```rust
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
```

##### Example of adding optional type annotations of the parameter and return value types in the closure

With type annotations added, the syntax of closures looks more similar to the syntax of functions. The following is a vertical comparison of the syntax for the definition of a function that adds 1 to its parameter and a closure that has the same behavior. We’ve added some spaces to line up the relevant parts. This illustrates how closure syntax is similar to function syntax except for the use of pipes and the amount of syntax that is optional:

```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

The first line shows a function definition, and the second line shows a fully annotated closure definition. The third line removes the type annotations from the closure definition, and the fourth line removes the brackets, which are optional because the closure body has only one expression. These are all valid definitions that will produce the same behavior when they’re called. Calling the closures is required for `add_one_v3` and `add_one_v4` to be able to compile because the types will be inferred from their usage.

Closure definitions will have one concrete type inferred for each of their parameters and for their return value. For instance, the following shows the definition of a short closure that just returns the value it receives as a parameter. This closure isn’t very useful except for the purposes of this example. Note that we haven’t added any type annotations to the definition: if we then try to call the closure twice, using a `String` as an argument the first time and a `u32` the second time, we’ll get an error.

```rust
let example_closure = |x| x;

let s = example_closure(String::from("hello"));
let n = example_closure(5);
```

##### Example of attempting to call a closure whose types are inferred with two different types

The compiler gives us this error:

```console
$ cargo run
   Compiling closure-example v0.1.0 (file:///projects/closure-example)
error[E0308]: mismatched types
 --> src/main.rs:5:29
  |
5 |     let n = example_closure(5);
  |                             ^- help: try using a conversion method: `.to_string()`
  |                             |
  |                             expected struct `String`, found integer

For more information about this error, try `rustc --explain E0308`.
error: could not compile `closure-example` due to previous error
```

The first time we call `example_closure` with the `String` value, the compiler infers the type of `x` and the return type of the closure to be `String`. Those types are then locked into the closure in `example_closure`, and we get a type error if we try to use a different type with the same closure.