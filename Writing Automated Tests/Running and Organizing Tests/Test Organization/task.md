## Test Organization

As mentioned at the start of the chapter, testing is a complex discipline, and
different people use different terminology and organization. The Rust community
thinks about tests in terms of two main categories: *unit tests* and
*integration tests*. Unit tests are small and more focused, testing one module
in isolation at a time, and can test private interfaces. Integration tests are
entirely external to your library and use your code in the same way any other
external code would, using only the public interface and potentially exercising
multiple modules per test.

Writing both kinds of tests is important to ensure that the pieces of your
library are doing what you expect them to, separately and together.
