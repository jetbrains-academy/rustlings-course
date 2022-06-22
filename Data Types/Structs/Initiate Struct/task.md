## Initiate Struct

Complete all TODO fields to make the tests pass!
You should define a Classic Struct with two fields: "name" and "hex" and then create an instance of this struct with values "green" and "#00FF00" corresponding to these fields. 
You should also define a Tuple Struct with two string-type fields and instantiate one with the same values as before. 
Finally, you need to instantiate a Unit Struct and make sure our program prints the message "UnitStructs are fun!" :)

**Note**: all fields in the first two structs have to be [made public](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#exposing-paths-with-the-pub-keyword) using the `pub` keyword.
This is needed for proper testing and for `main.rs` to work. The public modifier will be explained in detail in the [Modules](course://Modules/Modules) lesson.

<div class="hint">
Rust has more than one type of struct. Three actually, all variants are used to package related data together.
There are normal (or classic) structs. These are named collections of related data stored in fields.
Tuple structs are basically just named tuples.
Finally, Unit structs. These don't have any fields and are useful for generics.
</div>

<div class="hint">
In this exercise you need to complete and implement one of each kind.
Read more about structs in <a href="https://doc.rust-lang.org/book/ch05-01-defining-structs.html">The Book</a>.
</div>

