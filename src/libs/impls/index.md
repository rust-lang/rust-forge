# Maintaining implementations

In addition the standard library's API surface, the library team also maintains the standard library for a growing number of [platforms] supported by Rust for an ever-increasing set of use cases. Rust is supposed to be blazingly fast, not set your computer ablaze.

[platforms]: https://doc.rust-lang.org/nightly/rustc/platform-support.html

While in general, maintaining the standard library just requires being good at Rust, there are a few important cases to note:

- [Documentation]
  - *How do we document the standard library?*
- [Testing and debugging]
  - *What are some of the issues with testing the standard library?*
- [Performance and benchmarking]
  - *Again, for the toaster, its CPU isn't very fast. My toast is blazing.*
- [Target tiers]
  - *How can I enable thread-local storage on my toaster?*
- [LLM usage]
  - *Unless otherwise stated, the standard library follows the `rust-lang/rust` LLM policy.*

[Documentation]: ./documentation.md
[Target tiers]: ./targets.md
[testing and debugging]: ./testing-debugging.md
[Performance and benchmarking]: ./perf-benchmarking.md
[LLM usage]: ../../policies/llm-usage.md
