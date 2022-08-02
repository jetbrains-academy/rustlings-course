### Default Implementations

Sometimes it’s useful to have default behavior for some or all of the methods
in a trait instead of requiring implementations for all methods on every type.
Then, as we implement the trait on a particular type, we can keep or override
each method’s default behavior.

The code below shows how to specify a default string for the `summarize` method
of the `Summary` trait instead of only defining the method signature, as we did
in the first listing of this section.


```rust,noplayground
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```

#### Definition of a `Summary` trait with a default implementation of the `summarize` method

To use a default implementation to summarize instances of `NewsArticle` instead
of defining a custom implementation, we specify an empty `impl` block with
`impl Summary for NewsArticle {}`.

Even though we’re no longer defining the `summarize` method on `NewsArticle`
directly, we’ve provided a default implementation and specified that
`NewsArticle` implements the `Summary` trait. As a result, we can still call
the `summarize` method on an instance of `NewsArticle`, like this:

```rust,ignore
let article = NewsArticle {
    headline: String::from("Penguins win the Stanley Cup Championship!"),
    location: String::from("Pittsburgh, PA, USA"),
    author: String::from("Iceburgh"),
    content: String::from(
        "The Pittsburgh Penguins once again are the best \
        hockey team in the NHL.",
    ),
};

println!("New article available! {}", article.summarize());
```

This code prints `New article available! (Read more...)`.

Creating a default implementation for `summarize` doesn’t require us to change
anything about the implementation of `Summary` on `Tweet` in the second listing of this section. The
reason is that the syntax for overriding a default implementation is the same
as the syntax for implementing a trait method that doesn’t have a default
implementation.
