### Updating a String

A `String` can grow in size and its contents can change, just like the contents of a `Vec<T>`, if you push more data into it. In addition, you can conveniently use the `+` operator or the `format!` macro to concatenate `String` values.

#### Appending to a String with `push_str` and `push`

We can grow a `String` by using the `push_str` method to append a string slice, as shown in the code snippet below.

```rust
    let mut s = String::from("foo");
    s.push_str("bar");
```

##### Example of appending a string slice to a String using the push_str method

After these two lines, `s` will contain `foobar`. The `push_str` method takes a string slice because we don’t necessarily want to take ownership of the parameter. For example, the next code snippet shows that it would be unfortunate if we weren’t able to use `s2` after appending its contents to `s1`.

```rust
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);
```

##### Using a string slice after appending its contents to a String

If the `push_str` method took ownership of `s2`, we wouldn’t be able to print its value on the last line. However, this code works as we’d expect!

The `push` method takes a single character as a parameter and adds it to the `String`. The code snippet below shows code that adds the letter _l_ to a `String` using the `push` method.

```rust
    let mut s = String::from("lo");
    s.push('l');
```

##### Adding one character to a String value using push

As a result of this code, `s` will contain `lol`.

#### Concatenation with the `+` Operator or the format! Macro

Often, you’ll want to combine two existing strings. One way is to use the `+` operator, as shown below.

```rust
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
```

##### Using the + operator to combine two String values into a new String value

The string `s3` will contain `Hello, world!` as a result of this code. The reason `s1` is no longer valid after the addition and the reason we used a reference to `s2` has to do with the signature of the method that gets called when we use the `+` operator. The `+` operator uses the `add` method, whose signature looks something like this:

```rust
    fn add(self, s: &str) -> String {
```

This isn’t the exact signature that’s in the standard library: in the standard library, `add` is defined using generics. Here, we’re looking at the signature of `add` with concrete types substituted for the generic ones, which is what happens when we call this method with `String` values. We’ll discuss generics in the chapter "Generic Types, Traits and Lifetime". This signature gives us the clues we need to understand the tricky bits of the `+` operator.

First, `s2` has an `&`, meaning that we’re adding a _reference_ of the second string to the first string because of the `s` parameter in the `add` function: we can only add a `&str` to a `String`; we can’t add two `String` values together. But wait—the type of `&s2` is `&String`, not `&str`, as specified in the second parameter to `add`. So why does the listing "Using the + operator to combine two String values into a new String value" compile?

The reason we’re able to use `&s2` in the call to `add` is that the compiler can _coerce_ the `&String` argument into a `&str`. When we call the `add` method, Rust uses a _deref coercion_, which here turns `&s2` into `&s2[..]`. We’ll discuss deref coercion in more depth in Chapter 15 of the [Rust Book](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html). Because `add` does not take ownership of the `s` parameter, `s2` will still be a valid `String` after this operation.

Second, we can see in the signature that `add` takes ownership of `self`, because `self` does _not_ have an `&`. This means `s1` in the listing "Using the + operator to combine two String values into a new String value" will be moved into the `add` call and no longer be valid after that. So although `let s3 = s1 + &s2;` looks like it will copy both strings and create a new one, this statement actually takes ownership of `s1`, appends a copy of the contents of `s2`, and then returns ownership of the result. In other words, it looks like it’s making a lot of copies but isn’t; the implementation is more efficient than copying.

If we need to concatenate multiple strings, the behavior of the `+` operator gets unwieldy:

```rust
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
```

At this point, `s` will be `tic-tac-toe`. With all of the `+` and `"` characters, it’s difficult to see what’s going on. For more complicated string combining, we can use the `format!` macro:

```rust
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
```

This code also sets `s` to `tic-tac-toe`. The `format!` macro works in the same way as `println!`, but instead of printing the output to the screen, it returns a `String` with the contents. The version of the code using `format!` is much easier to read and doesn’t take ownership of any of its parameters.
