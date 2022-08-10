### Step 2: Defining a Procedural Macro

The next step is to define the procedural macro. At the time of this writing, procedural macros need to be in their own crate. Eventually, this restriction might be lifted. The convention for structuring crates and macro crates is as follows: for a crate named `foo`, a custom derive procedural macro crate is called `foo_derive`. 

Here we have our new crate `hello_macro_derive` defined in `Cargo.toml`.

Our two crates are tightly related, so we create the procedural macro crate within the directory of our `hello_macro` crate. If we change the trait definition in `hello_macro`, we’ll have to change the implementation of the procedural macro in `hello_macro_derive` as well. The two crates will need to be published separately, and programmers using these crates will need to add both as dependencies and bring them both into scope. We could instead have the `hello_macro` crate use `hello_macro_derive` as a dependency and re-export the procedural macro code. However, the way we’ve structured the project makes it possible for programmers to use `hello_macro` even if they don’t want the `derive` functionality.

We need to declare the `hello_macro_derive` crate as a procedural macro crate. We’ll also need functionality from the `syn` and `quote` crates, as you’ll see in a moment, so we need to add them as dependencies. Add the following to the _Cargo.toml_ file for `hello_macro_derive`:

```toml
    [lib]
    proc-macro = true

    [dependencies]
    syn = "1.0"
    quote = "1.0"
```

To start defining the procedural macro, place the code in the snippet below into your _src/lib.rs_ file for the `hello_macro_derive` crate. Note that this code won’t compile until we add a definition for the `impl_hello_macro` function.

```rust
    extern crate proc_macro;

    use crate::proc_macro::TokenStream;
    use quote::quote;
    use syn;

    #[proc_macro_derive(HelloMacro)]
    pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
        // Construct a representation of Rust code as a syntax tree
        // that we can manipulate
        let ast = syn::parse(input).unwrap();

        // Build the trait implementation
        impl_hello_macro(&ast)
    }
```

##### Code that most procedural macro crates will require in order to process Rust code

Notice that we’ve split the code into the `hello_macro_derive` function, which is responsible for parsing the `TokenStream` and the `impl_hello_macro` function, which is responsible for transforming the syntax tree: this makes writing a procedural macro more convenient. The code in the outer function (`hello_macro_derive` in this case) will be the same for almost every procedural macro crate you see or create. The code you specify in the body of the inner function (`impl_hello_macro` in this case) will be different depending on your procedural macro’s purpose.

We’ve introduced three new crates: `proc_macro`, [`syn`](https://crates.io/crates/syn), and [`quote`](https://crates.io/crates/quote). The `proc_macro` crate comes with Rust, so we didn’t need to add that to the dependencies in _Cargo.toml_. The `proc_macro` crate is the compiler’s API that allows us to read and manipulate Rust code from our code.

The `syn` crate parses Rust code from a string into a data structure that we can perform operations on. The `quote` crate turns `syn` data structures back into Rust code. These crates make it much simpler to parse any sort of Rust code we might want to handle: writing a full parser for Rust code is no simple task.

The `hello_macro_derive` function will be called when a user of our library specifies `#[derive(HelloMacro)]` on a type. This is possible because we’ve annotated the `hello_macro_derive` function here with `proc_macro_derive` and specified the name, `HelloMacro`, which matches our trait name; this is the convention most procedural macros follow.

The `hello_macro_derive` function first converts the `input` from a `TokenStream` to a data structure that we can then interpret and perform operations on. This is where `syn` comes into play. The `parse` function in `syn` takes a `TokenStream` and returns a `DeriveInput` struct representing the parsed Rust code. The example below shows the relevant parts of the `DeriveInput` struct we get from parsing the `struct Pancakes;` string:

```rust
    DeriveInput {
        // --snip--

        ident: Ident {
            ident: "Pancakes",
            span: #0 bytes(95..103)
        },
        data: Struct(
            DataStruct {
                struct_token: Struct,
                fields: Unit,
                semi_token: Some(
                    Semi
                )
            }
        )
    }
```

##### The DeriveInput instance we get when parsing the code that has the macro’s attribute in "The code a user of our crate will be able to write when using our procedural macro" snippet above

The fields of this struct show that the Rust code we’ve parsed is a unit struct with the `ident` (identifier, meaning the name) of `Pancakes`. There are more fields on this struct for describing all sorts of Rust code; check the [`syn` documentation for `DeriveInput`](https://docs.rs/syn/0.14.4/syn/struct.DeriveInput.html) for more information.

Soon we’ll define the `impl_hello_macro` function, which is where we’ll build the new Rust code we want to include. But before we do, note that the output for our derive macro is also a `TokenStream`. The returned `TokenStream` is added to the code that our crate users write, so when they compile their crate, they’ll get the extra functionality that we provide in the modified `TokenStream`.

You might have noticed that we’re calling `unwrap` to cause the `hello_macro_derive` function to panic if the call to the `syn::parse` function fails here. It’s necessary for our procedural macro to panic on errors because `proc_macro_derive` functions must return `TokenStream` rather than `Result` to conform to the procedural macro API. We’ve simplified this example by using `unwrap`; in production code, you should provide more specific error messages about what went wrong by using `panic!` or `expect`.

Now that we have the code to turn the annotated Rust code from a `TokenStream` into a `DeriveInput` instance, let’s generate the code that implements the `HelloMacro` trait on the annotated type, as shown in the code snippet below.

```rust
    fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
        let name = &ast.ident;
        let gen = quote! {
            impl HelloMacro for #name {
                fn hello_macro() {
                    println!("Hello, Macro! My name is {}", stringify!(#name));
                }
            }
        };
        gen.into()
    }
```

##### Implementing the HelloMacro trait using the parsed Rust code

We get an `Ident` struct instance containing the name (identifier) of the annotated type using `ast.ident`. The struct in the snippet with DeriveInput instance shows that when we run the `impl_hello_macro` function on the code in the example with the procedural macro, the `ident` we get will have the `ident` field with a value of `"Pancakes"`. Thus, the `name` variable in the snippet above will contain an `Ident` struct instance that, when printed, will be the string `"Pancakes"`, the name of the struct in the example with the procedural macro.

The `quote!` macro lets us define the Rust code that we want to return. The compiler expects something different to the direct result of the `quote!` macro’s execution, so we need to convert it to a `TokenStream`. We do this by calling the `into` method, which consumes this intermediate representation and returns a value of the required `TokenStream` type.

The `quote!` macro also provides some very cool templating mechanics: we can enter `#name`, and `quote!` will replace it with the value in the variable `name`. You can even do some repetition similar to the way regular macros work. Check out [the `quote` crate’s docs](https://docs.rs/quote) for a thorough introduction.

We want our procedural macro to generate an implementation of our `HelloMacro` trait for the type the user annotated, which we can get by using `#name`. The trait implementation has one function, `hello_macro`, whose body contains the functionality we want to provide: printing `Hello, Macro! My name is` and then the name of the annotated type.

The `stringify!` macro used here is built into Rust. It takes a Rust expression, such as `1 + 2`, and at compile time turns the expression into a string literal, such as `"1 + 2"`. This is different than `format!` or `println!`, macros which evaluate the expression and then turn the result into a `String`. There is a possibility that the `#name` input might be an expression to print literally, so we use `stringify!`. Using `stringify!` also saves an allocation by converting `#name` to a string literal at compile time.

At this point, `cargo build` should complete successfully in both `hello_macro` and `hello_macro_derive`. Let’s hook up these crates to the code in the example with the procedural macro to see the procedural macro in action! Create a new binary project in your _projects_ directory using `cargo new pancakes`. We need to add `hello_macro` and `hello_macro_derive` as dependencies in the `pancakes` crate’s _Cargo.toml_. If you’re publishing your versions of `hello_macro` and `hello_macro_derive` to _https://crates.io/_, they would be regular dependencies; if not, you can specify them as `path` dependencies as follows:

```toml
    [dependencies]
    hello_macro = { path = "../hello_macro" }
    hello_macro_derive = { path = "../hello_macro/hello_macro_derive" }
```

Put the code in the example with the procedural macro into _src/main.rs_, and run `cargo run`: it should print `Hello, Macro! My name is Pancakes!` The implementation of the `HelloMacro` trait from the procedural macro was included without the `pancakes` crate needing to implement it; the `#[derive(HelloMacro)]` added the trait implementation.

Next, let’s explore how the other kinds of procedural macros differ from custom derive macros.
