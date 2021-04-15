## Defining and Instantiating Structs

Structs are similar to tuples, which were discussed in the chapter "Common Programming Concepts". Like in the case of tuples,
the pieces of a struct can be of different types. Unlike the case with tuples, you’ll name
each piece of data so it’s clear what the values mean. As a result of assigning these
names, structs are more flexible than tuples: you don’t have to rely on the
order of the data to specify or access the values of an instance.

To define a struct, we enter the keyword `struct` and name the entire struct. A
struct’s name should describe the significance of the pieces of data being
grouped together. Then, inside curly brackets, we define the names and types of
the pieces of data, which we call *fields*. For example, the listing below shows a
struct that stores information about a user account.

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

#### A `User` struct definition

To use a struct after we’ve defined it, we create an *instance* of that struct
by specifying concrete values for each of the fields. We create an instance by
stating the name of the struct and then add curly brackets containing `key:
value` pairs, where the keys are the names of the fields and the values are the
data we want to store in those fields. We don’t have to specify the fields in
the same order in which we declared them in the struct. In other words, the
struct definition is like a general template for the type, and instances fill
in that template with particular data to create values of the type. For
example, we can declare a particular user as shown below.

```rust
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
```

#### Creating an instance of the `User` struct

To get a specific value from a struct, we can use dot notation. If we wanted
just this user’s email address, we could use `user1.email` wherever we wanted
to use this value. If the instance is mutable, we can change a value by using
the dot notation and assigning into a particular field. The code below shows how
to change the value in the `email` field of a mutable `User` instance.

```rust
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
```

#### Changing the value in the `email` field of a `User` instance

Note that the entire instance must be mutable; Rust doesn’t allow us to mark
only certain fields as mutable. As with any expression, we can construct a new
instance of the struct as the last expression in the function body to
implicitly return that new instance.

The code below shows a `build_user` function that returns a `User` instance with
the given email and username. The `active` field gets the value of `true`, and
the `sign_in_count` gets a value of `1`.

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
```

#### A `build_user` function that takes an email and username and returns a `User` instance

It makes sense to name the function parameters with the same name as the struct
fields, but having to repeat the `email` and `username` field names and
variables is a bit tedious. If the struct had more fields, repeating each name
would get even more annoying. Luckily, there’s a convenient shorthand!

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

### Using Tuple Structs without Named Fields to Create Different Types

You can also define structs that look similar to tuples, called *tuple
structs*. Tuple structs have the added meaning the struct name provides but
don’t have names associated with their fields; rather, they just have the types
of the fields. Tuple structs are useful when you want to give the whole tuple a
name and make the tuple be a different type from other tuples, and naming each
field as in a regular struct would be verbose or redundant.

To define a tuple struct, start with the `struct` keyword and the struct name
followed by the types in the tuple. For example, here are definitions and
usages of two tuple structs named `Color` and `Point`:

```rust
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
```

Note that the `black` and `origin` values are different types, because they’re
instances of different tuple structs. Each struct you define is its own type,
even though the fields within the struct have the same types. For example, a
function that takes a parameter of type `Color` cannot take a `Point` as an
argument, even though both types are made up of three `i32` values. Otherwise,
tuple struct instances behave like tuples: you can destructure them into their
individual pieces, you can use a `.` followed by the index to access an
individual value, and so on.

### Unit-Like Structs Without Any Fields

You can also define structs that don’t have any fields! These are called
*unit-like structs* because they behave similarly to `()`, the unit type.
Unit-like structs can be useful in situations in which you need to implement a
trait on some type but don’t have any data that you want to store in the type
itself. We’ll discuss traits in the chapter "Generic Types, Traits and Lifetime".

### Ownership of Struct Data

In the `User` struct definition in the first code snippet of this section, we used the owned `String`
type rather than the `&str` string slice type. This is a deliberate choice
because we want instances of this struct to own all of its data and for that
data to be valid for as long as the entire struct is valid.

It’s possible for structs to store references to data owned by something else,
but to do so requires the use of *lifetimes*, a Rust feature that we’ll
discuss in in the chapter "Generic Types, Traits and Lifetime". Lifetimes ensure that the data referenced by a struct
is valid for as long as the struct is. Let’s say you try to store a reference
in a struct without specifying lifetimes, like this, which won’t work:


```rust,ignore,does_not_compile
 struct User {
     username: &str,
     email: &str,
     sign_in_count: u64,
     active: bool,
 }

 fn main() {
     let user1 = User {
         email: "someone@example.com",
         username: "someusername123",
         active: true,
         sign_in_count: 1,
     };
 }
```

 The compiler will complain that it needs lifetime specifiers:

```console
 $ cargo run
    Compiling structs v0.1.0 (file:///projects/structs)
 error[E0106]: missing lifetime specifier
  --> src/main.rs:2:15
   |
 2 |     username: &str,
   |               ^ expected named lifetime parameter
   |
 help: consider introducing a named lifetime parameter
   |
 1 | struct User<'a> {
 2 |     username: &'a str,
   |

 error[E0106]: missing lifetime specifier
  --> src/main.rs:3:12
   |
 3 |     email: &str,
   |            ^ expected named lifetime parameter
   |
 help: consider introducing a named lifetime parameter
   |
 1 | struct User<'a> {
 2 |     username: &str,
 3 |     email: &'a str,
   |

 error: aborting due to 2 previous errors

 For more information about this error, try `rustc --explain E0106`.
 error: could not compile `structs`

 To learn more, run the command again with --verbose.
```

 In the chapter "Generic Types, Traits and Lifetime", we’ll discuss how to fix these errors, so you can store
 references in structs, but for now, we’ll fix errors like these using owned
 types like `String` instead of references like `&str`.


