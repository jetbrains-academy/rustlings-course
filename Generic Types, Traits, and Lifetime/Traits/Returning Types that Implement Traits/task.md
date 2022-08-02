### Returning Types that Implement Traits

We can also use the `impl Trait` syntax in the return position to return a
value of some type that implements a trait, as shown here:

```rust,ignore
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
```

By using `impl Summary` for the return type, we specify that the
`returns_summarizable` function returns some type that implements the `Summary`
trait without naming the concrete type. In this case, `returns_summarizable`
returns a `Tweet`, but the code calling this function doesn’t know that.

The ability to return a type that is only specified by the trait it implements
is especially useful in the context of closures and iterators, which we cover
in Chapter 13. Closures and iterators create types that only the compiler knows
or types that are very long to specify. The `impl Trait` syntax lets you
concisely specify that a function returns some type that implements the
`Iterator` trait without needing to write out a very long type.

However, you can only use `impl Trait` if you’re returning a single type. For
example, this code that returns either a `NewsArticle` or a `Tweet` with the
return type specified as `impl Summary` wouldn’t work:

```rust,ignore,does_not_compile
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}
```

Returning either a `NewsArticle` or a `Tweet` isn’t allowed due to restrictions
around how the `impl Trait` syntax is implemented in the compiler. We’ll cover
how to write a function with this behavior in the [“Using Trait Objects That
Allow for Values of Different
Types”][using-trait-objects-that-allow-for-values-of-different-types]<!--
ignore --> section of Chapter 17 of the Rust Book.

[using-trait-objects-that-allow-for-values-of-different-types]:
https://doc.rust-lang.org/book/ch17-02-trait-objects.html
