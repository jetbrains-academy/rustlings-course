### Creating Instances From Other Instances With Struct Update Syntax

It’s often useful to create a new instance of a struct that uses most of an old
instance’s values but changes some. You’ll do this using *struct update syntax*.

First, the listing below shows how we create a new `User` instance in `user2` without
the update syntax. We set new values for `email` and `username` but otherwise
use the same values from `user1` that we created in the second code snippet of this section.

```rust
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
```

#### Creating a new `User` instance using some of the values from `user1`

Using struct update syntax, we can achieve the same effect with less code, as
shown below. The syntax `..` specifies that the remaining fields not
explicitly set should have the same value as the fields in the given instance.

```rust
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
```

#### Using struct update syntax to set new `email` and `username` values for a `User` instance but use the rest of the values from the fields of the instance in the `user1` variable

The code above also creates an instance in `user2` that has a
different value for `email` and `username` but has the same values for the
`active` and `sign_in_count` fields from `user1`.
