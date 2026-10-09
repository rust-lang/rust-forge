# Maintaining APIs

Compared to third-party crates on crates.io, the Rust standard library has much stronger stability guarantees. Any stable API that's added to the standard library can *never* be removed or modified in a backwards-incompatible way, even if we do have [a few ways to get around this](./changing.md).

In some cases, APIs will need special language or compiler support, and these have to exist in the standard library. For example, the `include!` macro is a standard library API, but the API exposes a feature specified in the language and implemented in the compiler. The library team works closely with the language and compiler teams on these kinds of features.

This also indicates a few more cases where standard library support makes sense, since the compiler has very robust testing infrastructure: if an API is performance-sensitive, we can add additional compiler intrinsics to support it, or codegen tests to verify its correctness. We also run tests for several different systems and generally have the capacity to maintain cross-platform functionality better than most crates.

APIs can also be substantially more ergonomic in the standard library. If you want to use a new method on a primitive type like `u32` without importing a separate trait, it has to live in the standard library.

As nice as standard library APIs can be, as mentioned earlier, they represent a very strong commitment that has to be taken seriously. And here are some of the ways we do that:

- [API Change Proposals (ACPs)](./proposals.md)
  - *Most API changes start with an API Change Proposal.*
- [Changing APIs](./changing.md)
  - *Not all API changes are obvious, or even API-related.*
- [API stabilization](./stabilization.md)
  - *Every new guarantee requires team consensus.*
- [Fixing APIs](./fixing.md)
  - *We can't break APIs, but can we fix them?*
