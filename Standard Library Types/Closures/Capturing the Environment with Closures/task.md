### Capturing the Environment with Closures

The first aspect of closures we’re going to examine is that closures can capture values from the environment they’re defined in for later use. Here’s the scenario: A t-shirt company gives away a free shirt to someone on their mailing list every so often. People on the mailing list can optionally add their favorite color to their profile. If the person chosen to get the free shirt has their favorite color in their profile, they get that color shirt. If the person hasn’t specified a favorite color, they get the color that the company currently has the most of.

There are many ways to implement this. For this example, we’re going to use an enum called `ShirtColor` that has the variants `Red` and `Blue`. The company’s inventory is represented by an `Inventory` struct that has a field named `shirts` that contains a `Vec<ShirtColor>` representing the shirts currently in stock. The method `shirt_giveaway` defined on `Inventory` gets the optional shirt color preference of the person getting the free shirt, and returns the shirt color the person will get. This is shown in the following Listing:

```rust
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}
```

##### Shirt company giveaway

The `store` defined in `main` has two blue shirts and one red shirt in stock. Then it calls the `giveaway` method for a user with a preference for a red shirt and a user without any preference. Running this code prints:

```console
$ cargo run
   Compiling shirt-company v0.1.0 (file:///projects/shirt-company)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/shirt-company`
The user with preference Some(Red) gets Red
The user with preference None gets Blue
```

Again, this code could be implemented in many ways, but this way uses concepts you’ve already learned, except for the body of the `giveaway` method that uses a closure. The `giveaway` method takes the user preference `Option<ShirtColor>` and calls `unwrap_or_else` on it. The [`unwrap_or_else` method on `Option<T>`](https://doc.rust-lang.org/stable/std/option/enum.Option.html#method.unwrap_or_else) is defined by the standard library. It takes one argument: a closure without any arguments that returns a value `T` (the same type stored in the `Some` variant of the `Option<T>`, in this case, a `ShirtColor`). If the `Option<T>` is the `Some` variant, `unwrap_or_else` returns the value from within the `Some`. If the `Option<T>` is the `None` variant, `unwrap_or_else` calls the closure and returns the value returned by the closure.

This is interesting because we’ve passed a closure that calls `self.most_stocked()` on the current `Inventory` instance. The standard library didn’t need to know anything about the `Inventory` or `ShirtColor` types we defined, or the logic we want to use in this scenario. The closure captured an immutable reference to the `self` `Inventory` instance and passed it with the code we specified to the `unwrap_or_else` method. Functions are not able to capture their environment in this way.