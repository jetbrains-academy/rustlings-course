## Use Rc

In this exercise, we want to express the concept of multiple owners via the Rc<T> type.
This is a model of our solar system - there is a Sun type and multiple Planets.
The Planets take ownership of the sun, indicating that they revolve around the sun.

Make this code compile by using the proper `Rc` primitives to express that the sun has multiple owners.
