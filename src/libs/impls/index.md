# Maintaining implementations

In addition the standard library's API surface, the library team also maintains the standard library for a growing number of [platforms] supported by Rust for an ever-increasing set of use cases. Rust is supposed to be blazingly fast, not set your computer ablaze.

[platforms]: https://doc.rust-lang.org/nightly/rustc/platform-support.html

Here are just a few ways the library team does that:

- [Code review]
  - *What does it mean to be a standard librarian?*
- [Crate maintenance]
  - *In addition to the standard library, we have other libraries too.*
- [Target tiers]
  - *How can I enable thread-local storage on my toaster?*
- [Testing and debugging]
  - *What are some of the issues with testing the standard library?*
- [Performance and benchmarking]
  - *Again, for the toaster, its CPU isn't very fast. My toast is blazing.*

[Code review]: ./review.md
[Crate maintenance]: ./crates.md
[Target tiers]: ./targets.md
[testing and debugging]: ./testing-debugging.md
[Performance and benchmarking]: ./perf-benchmarking.md
