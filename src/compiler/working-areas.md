# Working Areas

Much of the ongoing work and initiatives from the compiler team are performed by groups of people interested in specific areas of work. These groups are a great way for new contributors to get involved as they provide a stream of tasks focused around one area and have designated channels for help and advice. Here is a list of areas where work is being carried on:

Name                              | Short Description                                                                                  | Code | Zulip Stream
----                              | -----------------                                                                                  | ---------- | ------------
Async-await Implementation        | Implementing async-await                                                                           | [Link][async-await_code] | [#wg-async][async-await_stream]
Diagnostics                       | Use crates.io crates for diagnostics rendering and make emitting diagnostics nicer.                | [rustc_errors], [rustc_lint], [annotate-snippets] | [#t-compiler/diagnostics][diagnostics_stream]
LLVM                              | Working with LLVM upstream to represent Rust in its development                                    | [rustc], [LLVM][llvm_code] | [#t-compiler/llvm][llvm_stream]
MIR Optimizations                 | Write MIR optimizations and refactor the MIR to be more optimizable.                               | [MIR transform](mir_transform_code) | [#t-compiler/mir-opts][mir-opts-stream]
Parallel-rustc                    | Making parallel compilation the default for rustc                                                  | [rustc] | [#t-compiler/parallel-rustc][parallel-rustc_stream]
Polonius                          | Exploring the integration of the "NLL 2.0"-like ["Polonius analysis"][Polonius] into rustc         | [borrow check][borrowck], [rust-lang/polonius][P], [rust-lang/datafrog][DF] |  [#t-types/polonius][polonius_stream]
RLS 2.0                           | Experimenting with a new compiler architecture tailored for IDEs                                   | [rust-analyzer][ra-repo] | [#t-compiler/rust-analyzer][rls20_stream]
Rustc Dev Guide                   | Make the compiler easier to learn by ensuring that rustc-dev-guide is "complete"                   | [rustc], [rustc-dev-guide][rustc-dev-guide-repo] | [#t-compiler/rustc-dev-guide][rustc-dev-guide_stream]

For historical record here's a list of Working Groups that are not active anymore (either because they reached their goals or because work stalled):

Name                             | Short Description                                                                                  | Zulip Stream
----                             | -----------------                                                                                  | ------------
Meta                             | How compiler team organizes itself                                                                 | [#z-archived-t-compiler/wg-meta][meta_stream]
Non-Lexical Lifetimes (NLL)      | Implementing non-lexical lifetimes                                                                 | [#z-archived-t-compiler/wg-nll][nll_stream]
Polymorphization                 | Implement an analysis to detect when functions can remain polymorphic during code generation.      | [#z-archived-t-compiler/wg-polymorphization][polymorphization_stream]
Prioritization                   | Triaging regressions, mainly deciding if there are potential release blockers                      | [#t-compiler/prioritization][prioritization_stream]
Profile-Guided Optimization      | Implementing profile-guided optimization for rustc                                                 | [#z-archived-t-compiler/wg-profile-guided-optimization][pgo_stream]
RFC 2229                         | Make a closure capture individual fields of the variable rather than the entire composite variable | [#z-archived-t-compiler/wg-rfc-2229][rfc-2229-stream]
Rustc pipelining                 | Enable Cargo to invoke rustc in a pipelined fashion, speeding up crate graph compiles.             | [#z-archived-t-compiler/wg-pipelining][pipelining-stream]
Self-Profile                     | Improving the `-Z self-profile` feature                                                            | [#z-archived-t-compiler/wg-self-profile][self-profile_stream]
Traits                           | Improving the trait-system design + implementation                                                 | [#z-archived-t-compiler/wg-traits][traits_stream]

[Weekly, in Zulip]: #meeting-calendar
[Polonius]: https://github.com/rust-lang/polonius
[rustc]: https://github.com/rust-lang/rust/tree/master/compiler
[async-await_stream]: https://rust-lang.zulipchat.com/#narrow/channel/187312-wg-async
[async-await_code]: https://github.com/rust-lang/wg-async-foundations
[diagnostics_stream]: https://rust-lang.zulipchat.com/#narrow/channel/147480-t-compiler.2Fdiagnostics
[llvm_stream]: https://rust-lang.zulipchat.com/#narrow/stream/187780-t-compiler.2Fwg-llvm
[llvm_code]: https://github.com/rust-lang/llvm-project
[meta_stream]: https://rust-lang.zulipchat.com/#narrow/channel/185694-z-archived-t-compiler.2Fwg-meta
[mir-opts-stream]: https://rust-lang.zulipchat.com/#narrow/stream/189540-t-compiler.2Fwg-mir-opt
[mir_transform_code]: https://github.com/rust-lang/rust/tree/master/compiler/rustc_mir_transform
[nll_stream]: https://rust-lang.zulipchat.com/#narrow/channel/122657-z-archived-t-compiler.2Fwg-nll
[parallel-rustc_stream]: https://rust-lang.zulipchat.com/#narrow/stream/187679-t-compiler.2Fwg-parallel-rustc
[pgo_stream]: https://rust-lang.zulipchat.com/#narrow/channel/187830-z-archived-t-compiler.2Fwg-profile-guided-optimization
[pipelining-stream]: https://rust-lang.zulipchat.com/#narrow/channel/195180-z-archived-t-compiler.2Fwg-pipelining
[polonius_stream]: https://rust-lang.zulipchat.com/#narrow/channel/186049-t-types.2Fpolonius
[polymorphization_stream]: https://rust-lang.zulipchat.com/#narrow/channel/216091-z-archived-t-compiler.2Fwg-polymorphization
[prioritization_stream]: https://rust-lang.zulipchat.com/#narrow/channel/227806-t-compiler.2Fprioritization
[rfc-2229-stream]: https://rust-lang.zulipchat.com/#narrow/channel/189812-z-archived-t-compiler.2Fwg-rfc-2229
[rls20_stream]: https://rust-lang.zulipchat.com/#narrow/channel/185405-t-compiler.2Frust-analyzer
[rustc-dev-guide_stream]: https://rust-lang.zulipchat.com/#narrow/stream/196385-t-compiler.2Fwg-rustc-dev-guide
[self-profile_stream]: https://rust-lang.zulipchat.com/#narrow/stream/187831-t-compiler.2Fwg-self-profile
[traits_stream]: https://rust-lang.zulipchat.com/#narrow/stream/144729-t-compiler.2Fwg-traits
[repo]: https://github.com/rust-lang/rust
[DF]: https://github.com/rust-lang/datafrog
[P]: https://github.com/rust-lang/polonius
[borrowck]: https://github.com/rust-lang/rust/tree/ae9173d7dd4a31806c950c90dcc331f1508b4d17/compiler/rustc_borrowck
[ra-repo]: https://github.com/rust-analyzer/rust-analyzer
[rustc-dev-guide-repo]: https://github.com/rust-lang/rustc-dev-guide
[rustc_errors]: https://github.com/rust-lang/rust/tree/master/compiler/rustc_errors
[rustc_lint]: https://github.com/rust-lang/rust/tree/master/compiler/rustc_lint
[annotate-snippets]: https://crates.io/crates/annotate-snippets
