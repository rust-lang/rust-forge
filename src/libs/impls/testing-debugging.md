# Testing and debugging

In order to ensure the standard library works, we test it. But how do we test the tests? This is how.

## Test crates

The standard library is unique, because it's included in every crate by default. This means that unfortunately, it is also included in tests by default, and that makes testing a *particular version of it* kind of difficult.

To get around this, instead of directly testing the `core`, `alloc`, and `std` crates, we have separate `coretests`, `alloctests`, and `stdtests` crates which are compiled for tests instead. Inside of these crates, we explicitly make sure that the version of the crates being tested represents the one for the standard library actually being built, instead of the one used by the bootstrap compiler.

This is particularly relevant because it means that `#[cfg(test)]` inside the ordinary standard library crates is effectively equivalent to `#[cfg(false)]`, since these crates are never built for testing. At time of writing, this isn't linted, but it probably should be.

## Incoherent implementations

One other quirk of the standard library is the concept of *incoherent impls*, which are the way we poorly pretend that `core`, `alloc`, and `std` are just different parts of the same crate. This allows us to, for example, add separate methods that convert slices into `Vec`s and string slices into `String`s.

Because allocating is just so dang useful, we've decided to just leave most slice and string tests in `alloctests` instead of `coretests`, since even though we *could* test them without allocating, why would we want to do that?

## Constant assertions

Sometimes, instead of writing runtime tests, we store our tests in constants that fail to compile when something is broken. These sometimes have annoying UX implications, but they're what the ecosystem likes using, so, shouldn't we use them too?

In general, we try to keep `const _` "tests" in the standard library to a minimum since the compiler actually has a much more powerful kind of test we can use instead: [UI tests]. These let us check that certain code compiles, or fails with a specific set of errors. These are *so* much nicer to use than constant assertions and should probably be used instead, just because we can. It also means that people developing Rust can put off fixing them until later, rather than having to delete them just so they can figure out how much they broke.

[UI tests]: https://rustc-dev-guide.rust-lang.org/tests/ui.html

## Codegen tests

See the section on [performance and benchmarking] for more information on testing code generation.

[performance and benchmarking]: ./perf-benchmarking.md
