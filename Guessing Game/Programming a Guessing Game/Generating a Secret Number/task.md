Next, we need to generate a secret number that the user will try to guess. The secret number should be different every time so the game is fun to play more than once. Let’s use a random number between 1 and 100 so the game isn’t too difficult. Rust doesn’t yet include random number functionality in its standard library. However, the Rust team does provide a 'rand' [crate](https://crates.io/crates/rand).

### Using a Crate to Get More Functionality

Remember that a crate is a collection of Rust source code files. The project we’ve been building is a _binary crate_, which is an executable. The `rand` crate is a library crate, which contains code intended to be used in other programs.

Cargo’s use of external crates is where it really shines. Before we can write code that uses `rand`, we need to modify the _Cargo.toml_ file to include the `rand` crate as a dependency. Open that file now and add the following line to the bottom beneath the `[dependencies]` section header that Cargo created for you:

```toml
[dependencies]

rand = "0.3.14"
```

In the _Cargo.toml_ file, everything that follows a header is part of a section that continues until another section starts. The `[dependencies]` section is where you tell Cargo which external crates your project depends on and which versions of those crates you require. In this case, we’ll specify the `rand` crate with the semantic version specifier `0.3.14`. Cargo understands [Semantic Versioning](http://semver.org/) (sometimes called _SemVer_), which is a standard for writing version numbers. The number 0.3.14 is actually shorthand for `^0.3.14`, which means “any version that has a public API compatible with version 0.3.14.”

Now, without changing any of the code, let’s build the project, as shown in the example below. You could either type $ cargo build into the terminal, or select Build -> Build Project option from the menu. 

```text
$ cargo build
    Updating registry `https://github.com/rust-lang/crates.io-index`
 Downloading rand v0.3.14
 Downloading libc v0.2.14
   Compiling libc v0.2.14
   Compiling rand v0.3.14
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53 secs
```

##### Example of the output from running cargo build after adding the rand crate as a dependency

You may see different version numbers (but they will all be compatible with the code, thanks to SemVer!), and the lines may be in a different order.

Now that we have an external dependency, Cargo fetches the latest versions of everything from the _registry_, which is a copy of data from [Crates.io](https://crates.io/). Crates.io is where people in the Rust ecosystem post their open source Rust projects for others to use.

After updating the registry, Cargo checks the `[dependencies]` section and downloads any crates you don’t have yet. In this case, although we only listed `rand` as a dependency, Cargo also grabbed a copy of `libc`, because rand depends on `libc` to work. After downloading the crates, Rust compiles them and then compiles the project with the dependencies available.

If you immediately run `cargo build` again without making any changes, you won’t get any output aside from the `Finished` line. Cargo knows it has already downloaded and compiled the dependencies, and you haven’t changed anything about them in your _Cargo.toml_ file. Cargo also knows that you haven’t changed anything about your code, so it doesn’t recompile that either. With nothing to do, it simply exits.

If you open up the _src/main.rs_ file, make a trivial change, and then save it and build again, you’ll only see two lines of output:

```text
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53 secs
```

These lines show Cargo only updates the build with your tiny change to the _src/main.rs_ file. Your dependencies haven’t changed, so Cargo knows it can reuse what it has already downloaded and compiled for those. It just rebuilds your part of the code.

### Ensuring Reproducible Builds with the Cargo.lock File

Cargo has a mechanism that ensures you can rebuild the same artifact every time you or anyone else builds your code: Cargo will use only the versions of the dependencies you specified until you indicate otherwise. For example, what happens if next week version 0.3.15 of the `rand` crate comes out and contains an important bug fix but also contains a regression that will break your code?

The answer to this problem is the Cargo.lock file, which was created the first time you ran `cargo build` and is now in your guessing_game directory. When you build a project for the first time, Cargo figures out all the versions of the dependencies that fit the criteria and then writes them to the _Cargo.lock_ file. When you build your project in the future, Cargo will see that the _Cargo.lock_ file exists and use the versions specified there rather than doing all the work of figuring out versions again. This lets you have a reproducible build automatically. In other words, your project will remain at `0.3.14` until you explicitly upgrade, thanks to the _Cargo.lock_ file.

### Updating a Crate to Get a New Version

When you _do_ want to update a crate, Cargo provides another command, `update`, which will ignore the _Cargo.lock_ file and figure out all the latest versions that fit your specifications in _Cargo.toml_. If that works, Cargo will write those versions to the _Cargo.lock_ file.

But by default, Cargo will only look for versions greater than `0.3.0` and less than `0.4.0`. If the `rand` crate has released two new versions, `0.3.15` and `0.4.0`, you would see the following if you ran `cargo update`:

$ cargo update
    Updating registry `https://github.com/rust-lang/crates.io-index`
    Updating rand v0.3.14 -> v0.3.15

At this point, you would also notice a change in your Cargo.lock file noting that the version of the rand crate you are now using is `0.3.15`.

If you wanted to use rand version `0.4.0` or any version in the `0.4.x` series, you’d have to update the _Cargo.toml_ file to look like this instead:

```toml
[dependencies]

rand = "0.4.0"
```

The next time you run `cargo build`, Cargo will update the registry of crates available and reevaluate your `rand` requirements according to the new version you have specified.

There’s a lot more to say about [Cargo](http://doc.crates.io/) and [its ecosystem](http://doc.crates.io/crates-io.html), but for now, that’s all you need to know. Cargo makes it very easy to reuse libraries, so Rustaceans are able to write smaller projects that are assembled from a number of packages.

### Generating a Random Number

Now that you’ve added the `rand` crate to _Cargo.toml_, let’s start using `rand`. The next step is to update src/main.rs.

```rust
rand::thread_rng().gen_range(1, 101);
```

##### A code to generate a random number from 1 to 100

Also, we need to add a `use` line: `use rand::Rng`. The `Rng` trait defines methods that random number generators implement, and this trait must be in scope for us to use those methods.

The `rand::thread_rng` function will give us the particular random number generator that we’re going to use: one that is local to the current thread of execution and seeded by the operating system. Then we call the `gen_range` method on the random number generator. This method is defined by the `Rng` trait that we brought into scope with the `use rand::Rng` statement. The gen_range method takes two numbers as arguments and generates a random number between them. It’s inclusive on the lower bound but exclusive on the upper bound, so we need to specify `1` and `101` to request a number between 1 and 100. Let's store the number generated in a variable and print it after with a message: "The secret number is:&#160;".

  > Note: You won’t just know which traits to use and which methods and functions to call from a crate. Instructions for using a crate are in each crate’s documentation. Another neat feature of Cargo is that you can run the cargo doc --open command, which will build documentation provided by all of your dependencies locally and open it in your browser. If you’re interested in other functionality in the rand crate, for example, run cargo doc --open and click rand in the sidebar on the left. You can also move your courser on a method or a field and press ctrl + q to fetch the documentation inside the editor. 

The second line that we added to the middle of the code prints the secret number. This is useful while we’re developing the program to be able to test it, but we’ll delete it from the final version. It’s not much of a game if the program prints the answer as soon as it starts!

Try running the program a few times:

```text
Finished dev [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/processing_a_guess`
Guess the number!
The secret number is: 3
Please input your guess.
2
You guessed: 2

Finished dev [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/processing_a_guess`
Guess the number!
The secret number is: 11
Please input your guess.
7
You guessed: 7
```

You should get different random numbers, and they should all be numbers between 1 and 100. Great job!

_You can refer to the following chapter in the Rust Programming Language Book: [Generating a Secret Number](https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html#generating-a-secret-number)_