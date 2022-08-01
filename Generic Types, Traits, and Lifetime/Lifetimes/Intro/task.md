## Validating References with Lifetimes

One detail we didn’t discuss in the [“References and
Borrowing” section of "Move Semantics"](course://Understanding Ownership/References and Borrowing) is
that every reference in Rust has a *lifetime*, which is the scope for which
that reference is valid. Most of the time, lifetimes are implicit and
inferred, just like most of the time, types are inferred. We must annotate
types when multiple types are possible. In a similar way, we must annotate
lifetimes when the lifetimes of references could be related in a few different
ways. Rust requires us to annotate the relationships using generic lifetime
parameters to ensure the actual references used at runtime will definitely be
valid.

The concept of lifetimes is somewhat different from tools in other programming
languages, arguably making lifetimes Rust’s most distinctive feature. Although
we won’t cover lifetimes in their entirety in this chapter, we’ll discuss
common ways you might encounter lifetime syntax so you can become familiar with
the concepts.
















