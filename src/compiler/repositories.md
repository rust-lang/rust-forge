# Repositories maintained by the Compiler Team

While the [rust-lang/rust] repository has the majority of the code for the compiler, there are a handful of additional repositories with other crates/dependencies that the compiler team are responsible for:

To ensure that the team is able to respond to urgent issues that originate in any of these repositories, all members of the compiler team have access to create and merge pull requests. However, each repository typically has team members who are the most familiar with the repository and act as its primary maintainers.

- The [`rustc` development guide][rustc-dev-guide], the entrypoint documentation for those interested in hacking the compiler.
- The [repository][compiler-team] of the team itself, used to register proposals for changes to the compiler or adjacent tooling and components.
- A [`cranelift`][cranelift-backend] experimental rustc backend, based on [Cranelift]. This has the potential to improve compilation times in debug mode. Currently maintained by [bjorn3].
- A Rust port of the [`LLVM::APFloat`][rustc_apfloat] library, maintained by the compiler team. As a port of an LLVM library, this repository has subtly different licensing arrangements than our other repositories, see [rustc_apfloat#licensing].
- A fork of [Enzyme], a high-performance automatic differentiator of LLVM and MLIR (more info at this [link][enzyme-mit]). This fork is maintained by [ZuseZ4].
- [`thorin`][thorin], a DWARF packaging utility supporting GNU extension and DWARF 5 package formats. Primarily maintained by [davidtwco][davidtwco].
- The [Rust Forge], the documentation website you're reading. Co-maintained by the Rust project collectively.
- [`ar_archive_writer`][ar_archive_writer]: Like the other `LLVM::APFloat` library, its license is slightly different than our other repositories, see [ar_archive_writer#licensing].
- [`datafrog`][datafrog], a lightweight [Datalog] engine intended to be embedded in other Rust programs (TODO: status?)
- [`ena`][ena] is an implementation of union-find / congruence-closure in Rust, contains the underlying implementation of our inference variable tables: it's responsible to track the instantiation and merging of inference variables.
- [`literal-escaper`][literal-escaper] is a library to unescape string literals. It is used by [`rustc_lexer`][rustc_lexer] and [`proc_macro`][proc_macro].
- [`miri`][miri] is the interpreter for Rust's mid-level intermediate representation. Detects unsafe code that fails to uphold its safety requirements.
- [`measureme`][measureme] is a library for recording and serializing `rustc` events to a binary format. Currently only for internal use within `rustc` itself.
- [`odht`][odht] is a crate for hash tables that can be mapped from disk into memory without up-front decoding. Currently only for internal use within `rustc` itself.
- [`rustc-demangle`][rustc-demangle]: Demangling for `rustc` symbols ([documentation][rustc_demangle_docs]).
- [`rustc-hash`][rustc-hash] is a non-cryptographic hashing algorithm used by `rustc`
- [`rustc-rayon`][rustc-rayon] is a fork of the Rayon data parallelism library for Rust. This is part of an ongoing effort to parallelize the `rustc` compilation, see our [working areas].
- [`rustc-stable-hash`][rustc-stable-hash] is a cross-platform, deterministic and *not secure* hashing algorithm used by `rustc`.
- [`stacker`][stacker] a library to help grow the stack when it runs out of space, see [documentation][stacker_docs].

Other repositories with tools for internal use:

- [`cargo-bisect`][cargo-bisect]: a tool to bisect [regressions] in the rust compiler, very useful to find where a bug was introduced.
- We have a [calendar] where we all teams register their meetings. Calendar clients can pull the `.ics` files and receive updates.
- [`jobserver-rs`][jobserver-rs]: an implementation of the GNU Make jobserver for Rust, see [documentation][jobserver_docs].
- [`josh-sync`][josh-sync]: a library to perform [`Just One Single History`][josh] synchronizations (pull and push) of Josh subtrees in the rust-lang/rust repository.

If you want to start (or are already) contributing to the Rust project and you have expertise or interest in any of these repositories, feel free to get in touch!

[rust-lang/rust]: https://github.com/rust-lang/rust
[cranelift-backend]: https://github.com/rust-lang/rustc_codegen_cranelift
[cranelift]: https://github.com/bytecodealliance/wasmtime/blob/main/cranelift
[rustc_apfloat]: https://github.com/rust-lang/rustc_apfloat
[rustc_apfloat#licensing]: https://github.com/rust-lang/rustc_apfloat#licensing
[enzyme]: https://github.com/rust-lang/enzyme
[enzyme-mit]: https://enzyme.mit.edu/
[thorin]: https://github.com/rust-lang/thorin
[rust forge]: https://github.com/rust-lang/rust-forge
[other]: https://github.com/orgs/rust-lang/teams/compiler/repositories

[calendar]: https://github.com/rust-lang/calendar
[compiler-team]: https://github.com/rust-lang/compiler-team
[cargo-bisect]: https://github.com/rust-lang/cargo-bisect-rustc
[regressions]: https://github.com/rust-lang/rust/issues?q=sort%3Aupdated-desc%20is%3Aissue%20(label%3Aregression-from-stable-to-beta%20OR%20label%3Aregression-untriaged%20OR%20label%3Aregression-from-stable-to-stable)
[rustc-dev-guide]: https://github.com/rust-lang/rustc-dev-guide
[ar_archive_writer]: https://github.com/rust-lang/ar_archive_writer
[ar_archive_writer#licensing]: https://github.com/rust-lang/ar_archive_writer#licensing
[datafrog]: https://github.com/rust-lang/datafrog
[ena]: https://github.com/rust-lang/ena
[literal-escaper]: https://github.com/rust-lang/literal-escaper
[measureme]: https://github.com/rust-lang/measureme
[miri]: https://github.com/rust-lang/miri
[odht]: https://github.com/rust-lang/odht
[rustc-demangle]: https://github.com/rust-lang/rustc-demangle
[rustc-hash]: https://github.com/rust-lang/rustc-hash
[rustc-rayon]: https://github.com/rust-lang/rustc-rayon
[rustc-stable-hash]: https://github.com/rust-lang/rustc-stable-hash
[stacker]: https://github.com/rust-lang/stacker
[stacker_docs]: https://docs.rs/stacker
[datalog]: https://en.wikipedia.org/wiki/Datalog
[rustc_lexer]: https://rustc-dev-guide.rust-lang.org/the-parser.html
[proc_macro]: https://rustc-dev-guide.rust-lang.org/macro-expansion.html#procedural-macros
[rustc_demangle_docs]: https://docs.rs/rustc-demangle
[working areas]: ./working-areas.html#working-areas
[jobserver-rs]: https://github.com/rust-lang/jobserver-rs
[jobserver_docs]: https://docs.rs/jobserver
[josh]: https://github.com/josh-project/josh
[josh-sync]: https://github.com/rust-lang/josh-sync

[bjorn3]: https://github.com/bjorn3
[zusez4]: https://github.com/ZuseZ4
[davidtwco]: https://github.com/davidtwco
