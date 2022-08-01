### Defining a Trait

A type’s behavior consists of the methods we can call on that type. Different
types share the same behavior if we can call the same methods on all of those
types. Trait definitions are a way to group method signatures together to
define a set of behaviors necessary to accomplish some purpose.

For example, let’s say we have multiple structs that hold various kinds and
amounts of text: a `NewsArticle` struct that holds a news story filed in a
particular location and a `Tweet` that can have at most 280 characters along
with metadata that indicates whether it was a new tweet, a retweet, or a reply
to another tweet.

We want to make a media aggregator library that can display summaries of data
that might be stored in a `NewsArticle` or `Tweet` instance. To do this, we
need a summary from each type, and we need to request that summary by calling a
`summarize` method on an instance. The code snippet below shows the definition of a
`Summary` trait that expresses this behavior.


```rust,noplayground
pub trait Summary {
    fn summarize(&self) -> String;
}
```

#### A `Summary` trait that consists of the behavior provided by a `summarize` method

Here, we declare a trait using the `trait` keyword and then the trait’s name,
which is `Summary` in this case. Inside the curly brackets, we declare the
method signatures that describe the behaviors of the types that implement this
trait, which in this case is `fn summarize(&self) -> String`.

After the method signature, instead of providing an implementation within curly
brackets, we use a semicolon. Each type implementing this trait must provide
its own custom behavior for the body of the method. The compiler will enforce
that any type that has the `Summary` trait will have the method `summarize`
defined with this signature exactly.

A trait can have multiple methods in its body: the method signatures are listed
one per line and each line ends in a semicolon.