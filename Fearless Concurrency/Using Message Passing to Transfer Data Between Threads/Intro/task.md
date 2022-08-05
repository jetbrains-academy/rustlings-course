## Using Message Passing to Transfer Data Between Threads

One increasingly popular approach to ensuring safe concurrency is _message passing_, where threads or actors communicate by sending each other messages containing data. Here’s the idea in a slogan from [the Go language documentation](http://golang.org/doc/effective_go.html): “Do not communicate by sharing memory; instead, share memory by communicating.”

One major tool Rust has for accomplishing message-sending concurrency is the _channel_, a programming concept that Rust’s standard library provides an implementation of. You can imagine a channel in programming as being like a channel of water, such as a stream or a river. If you put something like a rubber duck or boat into a stream, it will travel downstream to the end of the waterway.

A channel in programming has two halves: a transmitter and a receiver. The transmitter half is the upstream location where you put rubber ducks into the river, and the receiver half is where the rubber duck ends up downstream. One part of your code calls methods on the transmitter with the data you want to send, and another part checks the receiving end for arriving messages. A channel is said to be _closed_ if either the transmitter or receiver half is dropped.

Here, we’ll work up to a program that has one thread to generate values and send them down a channel, and another thread that will receive the values and print them out. We’ll be sending simple values between threads using a channel to illustrate the feature. Once you’re familiar with the technique, you could use channels to implement a chat system or a system where many threads perform parts of a calculation and send the parts to one thread that aggregates the results.




