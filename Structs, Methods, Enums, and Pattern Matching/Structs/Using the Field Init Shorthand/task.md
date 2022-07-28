### Using the Field Init Shorthand when Variables and Fields Have the Same Name

Because the parameter names and the struct field names are exactly the same
in the code above, we can use the *field init shorthand* syntax to rewrite
`build_user` so that it behaves exactly the same but doesn’t have the
repetition of `email` and `username`, as shown below.

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

#### A `build_user` function that uses field init shorthand because the `email` and `username` parameters have the same name as struct fields

Here, we’re creating a new instance of the `User` struct, which has a field
named `email`. We want to set the `email` field’s value to the value in the
`email` parameter of the `build_user` function. Because the `email` field and
the `email` parameter have the same name, we only need to write `email` rather
than `email: email`.
