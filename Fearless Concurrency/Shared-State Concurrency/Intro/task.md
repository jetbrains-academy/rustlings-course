## Shared-State Concurrency

Message passing is a fine way of handling concurrency, but it’s not the only one. Consider this part of the slogan from the Go language documentation again: “do not communicate by sharing memory.”

What would communicating by sharing memory look like? In addition, why would message-passing enthusiasts not use it and do the opposite instead?

In a way, channels in any programming language are similar to single ownership, because once you transfer a value down a channel, you should no longer use that value. Shared memory concurrency is like multiple ownership: multiple threads can access the same memory location at the same time. As you can see in [Chapter 15](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html) of the Rust Book, where smart pointers made multiple ownership possible, multiple ownership can add complexity because these different owners need managing. Rust’s type system and ownership rules greatly assist in getting this management correct. For an example, let’s look at mutexes, one of the more common concurrency primitives for shared memory.
