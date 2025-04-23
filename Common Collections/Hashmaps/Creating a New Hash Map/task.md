### Creating a New Hash Map

You can create an empty hash map with `new` and add elements with `insert`. In
the code below, we’re keeping track of the scores of two teams whose names are
Blue and Yellow. The Blue team starts with 10 points, and the Yellow team
starts with 50.

```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
```

#### Creating a new hash map and inserting some keys and values

Note that we need to first `use` the `HashMap` from the collections portion of
the standard library. Of our three common collections, this one is the least
often used, so it’s not included in the features brought into scope
automatically in the prelude. Hash maps also have less support from the
standard library; there’s no built-in macro to construct them, for example.

Just like vectors, hash maps store their data on the heap. This `HashMap` has
keys of type `String` and values of type `i32`. Like vectors, hash maps are
homogeneous: all of the keys must have the same type, and all of the values
must have the same type.

Another way of constructing a hash map is by using iterators and the `collect`
method on a vector of tuples, where each tuple consists of a key and its value.
We’ll be going into more detail about iterators and their associated methods in
the [Iterators](course://Standard%20Library%20Types/Iterators) lesson. The `collect` method gathers data into a number
of collection types, including `HashMap`. For example, if we had the team names
and initial scores in two separate vectors, we could use the `zip` method to
create a vector of tuples where “Blue” is paired with 10, and so forth. Then we
could use the `collect` method to turn that vector of tuples into a hash map,
as shown in the listing below.

```rust
    use std::collections::HashMap;

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();
```

#### Creating a hash map from a list of teams and a list of scores

The type annotation `HashMap<_, _>` is needed here because it’s possible to
`collect` into many different data structures and Rust doesn’t know which you
want unless you specify. For the parameters for the key and value types,
however, we use underscores, and Rust can infer the types that the hash map
contains based on the types of the data in the vectors. In the code above, the
key type will be `String` and the value type will be `i32`, just as the types
were in the first listing of this section.