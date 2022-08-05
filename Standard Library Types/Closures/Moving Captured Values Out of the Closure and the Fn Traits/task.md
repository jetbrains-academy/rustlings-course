### Moving Captured Values Out of the Closure and the `Fn` Traits

Once a closure has captured a reference or moved a value into the closure, the code in the body of the function also affects what happens to the references or values as a result of calling the function. A closure body can move a captured value out of the closure, can mutate the captured value, can neither move nor mutate the captured value, or can capture nothing from the environment. The way a closure captures and handles values from the environment affects which traits the closure implements. The traits are how functions and structs can specify what kinds of closures they can use.

Closures will automatically implement one, two, or all three of these `Fn` traits, in an additive fashion:

1.  `FnOnce` applies to closures that can be called at least once. All closures implement this trait, because all closures can be called. If a closure moves captured values out of its body, then that closure only implements `FnOnce` and not any of the other `Fn` traits, because it can only be called once.
2.  `FnMut` applies to closures that don’t move captured values out of their body, but that might mutate the captured values. These closures can be called more than once.
3.  `Fn` applies to closures that don’t move captured values out of their body and that don’t mutate captured values. These closures can be called more than once without mutating their environment, which is important in cases such as calling a closure multiple times concurrently. Closures that don’t capture anything from their environment implement `Fn`.