# Working Areas

Much of the ongoing work and initiatives from the compiler team are performed by groups of people interested in specific areas of work. These groups of are a great way for new contributors to get involved as they provide a stream of tasks all focused around one area and have designated channels for help and advice. Here is a list of areas where work is being carried on:

Name                                                      | Status       | Short Description                                                                                  | Zulip Stream
----                                                      | ------       | -----------------                                                                                  | ------------
[Async-await Implementation](working-groups/async-await/) | Active       | Implementing async-await                                                                           | [#wg-async][async-await_stream]
[Diagnostics](working-groups/diagnostics/)                | Active       | Use crates.io crates for diagnostics rendering and make emitting diagnostics nicer.                | [#t-compiler/diagnostics][diagnostics_stream]
[LLVM](working-groups/llvm/)                              | Active       | Working with LLVM upstream to represent Rust in its development                                    | [#t-compiler/llvm][llvm_stream]
[MIR Optimizations](working-groups/mir-opt/)              | Active       | Write MIR optimizations and refactor the MIR to be more optimizable.                               | [#t-compiler/mir-opt][mir-opt-stream]
[Parallel-rustc](working-groups/parallel-rustc/)          | Paused       | Making parallel compilation the default for rustc                                                  | [#t-compiler/parallel-rustc][parallel-rustc_stream]
[Polonius](working-groups/polonius/)                      | Active       | Exploring the integration of the "NLL 2.0"-like ["Polonius analysis"][Polonius] into rustc         | [#t-types/polonius][polonius_stream]
[RLS 2.0](working-groups/rls-2.0/)                        | Active       | Experimenting with a new compiler architecture tailored for IDEs                                   | [#t-compiler/rust-analyzer][rls20_stream]
[Rustc Dev Guide](working-groups/rustc-dev-guide/)        | Active       | Make the compiler easier to learn by ensuring that rustc-dev-guide is "complete"                   | [#t-compiler/rustc-dev-guide][rustc-dev-guide_stream]

For historical record here's a list of Working Groups that are not active anymore (either because they reached their goals or because work stalled):

Name                                                      | Status       | Short Description                                                                                  | Zulip Stream
----                                                      | ------       | -----------------                                                                                  | ------------
[Meta](working-groups/meta/)                              | Paused       | How compiler team organizes itself                                                                 | [#t-compiler/meta][meta_stream]
[Non-Lexical Lifetimes (NLL)](working-groups/nll/)        | Retired      | Implementing non-lexical lifetimes                                                                 | [#t-compiler/nll][nll_stream]
[Polymorphization](working-groups/polymorphization/)      | Active       | Implement an analysis to detect when functions can remain polymorphic during code generation.      | [#t-compiler/polymorphization][polymorphization_stream]
[Prioritization](working-groups/prioritization/)          | Active       | Triaging bugs, mainly deciding if bugs are critical (potential release blockers) or not.           | [#t-compiler/prioritization][prioritization_stream]
[Profile-Guided Optimization](working-groups/pgo/)        | Retired      | Implementing profile-guided optimization for rustc                                                 | [#t-compiler/profile-guided-optimization][pgo_stream]
[RFC 2229](working-groups/rfc-2229/)                      | Retired      | Make a closure capture individual fields of the variable rather than the entire composite variable | [#t-compiler/rfc-2229][rfc-2229-stream]
[Rustc pipelining](working-groups/pipelining/)            | Retired      | Enable Cargo to invoke rustc in a pipelined fashion, speeding up crate graph compiles.             | [#t-compiler/pipelining][pipelining-stream]
[Self-Profile](working-groups/self-profile/)              | Active       | Improving the `-Z self-profile` feature                                                            | [#t-compiler/self-profile][self-profile_stream]
[Traits](working-groups/traits/)                          | Active       | Improving the trait-system design + implementation                                                 | [#t-compiler/traits][traits_stream]

[Weekly, in Zulip]: #meeting-calendar
[nll_stream]: https://rust-lang.zulipchat.com/#narrow/stream/122657-t-compiler.2Fwg-nll
[llvm_stream]: https://rust-lang.zulipchat.com/#narrow/stream/187780-t-compiler.2Fwg-llvm
[meta_stream]: https://rust-lang.zulipchat.com/#narrow/stream/185694-t-compiler.2Fwg-meta
[rls20_stream]: https://rust-lang.zulipchat.com/#narrow/channel/185405-t-compiler.2Frust-analyzer
[traits_stream]: https://rust-lang.zulipchat.com/#narrow/stream/144729-t-compiler.2Fwg-traits
[async-await_stream]: https://rust-lang.zulipchat.com/#narrow/channel/187312-wg-async
[self-profile_stream]: https://rust-lang.zulipchat.com/#narrow/stream/187831-t-compiler.2Fwg-self-profile
[pgo_stream]: https://rust-lang.zulipchat.com/#narrow/stream/187830-t-compiler.2Fwg-profile-guided-optimization
[parallel-rustc_stream]: https://rust-lang.zulipchat.com/#narrow/stream/187679-t-compiler.2Fwg-parallel-rustc
[rfc-2229-stream]: https://rust-lang.zulipchat.com/#narrow/stream/189812-t-compiler.2Fwg-rfc-2229
[mir-opt-stream]: https://rust-lang.zulipchat.com/#narrow/stream/189540-t-compiler.2Fwg-mir-opt
[pipelining-stream]: https://rust-lang.zulipchat.com/#narrow/stream/195180-t-compiler.2Fwg-pipelining
[polonius_stream]: https://rust-lang.zulipchat.com/#narrow/channel/186049-t-types.2Fpolonius
[polymorphization_stream]: https://rust-lang.zulipchat.com/#narrow/stream/216091-t-compiler.2Fwg-polymorphization
[rustc-dev-guide_stream]: https://rust-lang.zulipchat.com/#narrow/stream/196385-t-compiler.2Fwg-rustc-dev-guide
[Polonius]: https://github.com/rust-lang/polonius
[diagnostics_stream]: https://rust-lang.zulipchat.com/#narrow/stream/147480-t-compiler.2Fwg-diagnostics
[prioritization_stream]: https://rust-lang.zulipchat.com/#narrow/stream/227806-t-compiler.2Fwg-prioritization
