# How to start contributing

Thank you for your interest in contributing to Rust! There are many ways to
contribute, and we appreciate all of them. This document describes how you can
get in touch with other Rust contributors and start contributing to Rust projects.

As a reminder, all contributors are expected to follow our [Code of Conduct][coc].

## Asking Questions

First, if you have any questions regarding your potential contributions, you can ask
other contributors on the following places:
- [Rust Zulip server][rust-zulip] is the primary communication space for most Rust
  teams and contributors. It is also a great place to observe on what is going on.
  - You can check out e.g. the compiler team (`t-compiler`) Zulip ["channel"][t-compiler-channel]
- [internals.rust-lang.org][internals] (IRLO) is a forum for discussing development of Rust.
- [Rust Discord][rust-discord] can be used to ask about [Rustup][rustup] contributions
  in the `#wg-rustup` channel.

See also the [list of teams and working groups][governance] and [the Community page][community] on the
official website for more resources.

[t-compiler-channel]: https://rust-lang.zulipchat.com/#narrow/stream/131828-t-compiler
[governance]: https://www.rust-lang.org/governance
[community]: https://www.rust-lang.org/community

**Please ask questions!** A lot of people report feeling that they are "wasting
expert time", but we do not feel that way. Contributors are important to us.

## How to start contributing?

The Rust project is quite large and it can be difficult to know which parts of the project need
help, or are a good starting place for beginners. Here is a (non-exhaustive) list of
`rust-lang` projects that could serve as starting places for you to contribute. Some of them
have contributor guides and issues that are marked as needing help or being good first issues.

| Project                            | Contribution guide                              | Good first issues                     | Description                             |
|------------------------------------|-------------------------------------------------|---------------------------------------|-----------------------------------------|
| [Compiler][rustc-repo]             | [rustc-dev-guide][rustc-guide]                  | [Good first issues][rustc-issues]     | Rust compiler and associated tooling    |
| [Standard library][std-repo]       | [std-dev-guide][std-guide]                      |                                       | Rust standard library                   |
| [Rustdoc][rustdoc-repo]            | [Rustdoc overview][rustdoc-guide]               |                                       | Rust documentation generator            |
| [Cargo][cargo-repo]                | [Cargo Contributor Guide][cargo-guide]          | [Good first issues][cargo-issues]     | Rust package manager and build system   |
| [Clippy][clippy-repo]              | [Clippy Contributor Guide][clippy-guide]        | [Good first issues][clippy-issues]    | Rust linter                             |
| [Rustfmt][rustfmt-repo]            | [Rustfmt Contributing Guide][rustfmt-guide]     | [Good first issues][rustfmt-issues]   | Rust formatter                          |
| [Rust analyzer][analyzer-repo]     | [Contributing Quick Start][analyzer-guide]      | [Good first issues][analyzer-issues]  | Rust compiler frontend and LSP for IDEs |
| [Miri][miri-repo]                  | [Miri Contribution Guide][miri-guide]           | [Good first issues][miri-issues]      | Rust interpreter and UB detector        |
| [Rustup][rustup-repo]              | [Rustup Developer Guide][rustup-guide]          | [Help wanted][rustup-issues]          | Rust toolchain installer                |
| [crates.io][crates-io-repo]        | [crates.io Contribution guide][crates-io-guide] | [Issue tracker][crates-io-issues]     | Rust package registry                   |
| [Bors][bors-repo]                  | [bors Development guide][bors-guide]            | [Help wanted][bors-issues]            | Rust CI merge bot                       |
| [rustc-perf][rustc-perf-repo]      |                                                 | [Help wanted][rustc-perf-issues]      | Rust compiler benchmark suite           |
| [Triagebot][triagebot-repo]        |                                                 | [Good first issues][triagebot-issues] | Rust automation bot                     |
| [Rust playground][playground-repo] |                                                 | [Help wanted][playground-issues]      | Rust online playground                  |
| [Rustlings][rustlings-repo]        | [Rustlings Contribution Guide][rustlings-guide] | [Issue tracker][rustlings-issues]     | Rust exercises                          |
| [MdBook][mdbook-repo]              | [MdBook Contribution Guide][mdbook-guide]       | [Good first issues][mdbook-issues]    | Book generator written in Rust          |

[rustc-repo]: https://github.com/rust-lang/rust
[rustc-issues]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AE-help-wanted+no%3Aassignee
[rustc-guide]: https://rustc-dev-guide.rust-lang.org
[std-repo]: https://github.com/rust-lang/rust/tree/master/library
[std-guide]: https://github.com/rust-lang/std-dev-guide
[rustdoc-repo]: https://github.com/rust-lang/rust/tree/master/src/librustdoc
[rustdoc-guide]: https://rustc-dev-guide.rust-lang.org/rustdoc.html
[cargo-repo]: https://github.com/rust-lang/cargo
[cargo-issues]: https://github.com/rust-lang/cargo/issues?q=is%3Aopen+is%3Aissue+label%3AS-accepted+no%3Aassignee
[cargo-guide]: https://doc.crates.io/contrib
[clippy-repo]: https://github.com/rust-lang/rust-clippy
[clippy-guide]: https://github.com/rust-lang/rust-clippy/blob/master/CONTRIBUTING.md
[clippy-issues]: https://github.com/rust-lang/rust-clippy/issues?q=is%3Aopen%20is%3Aissue%20no%3Aassignee%20label%3A%22good%20first%20issue%22
[rustfmt-repo]: https://github.com/rust-lang/rustfmt
[rustfmt-guide]: https://github.com/rust-lang/rustfmt/blob/master/Contributing.md
[rustfmt-issues]: https://github.com/rust-lang/rustfmt/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22
[analyzer-repo]: https://github.com/rust-lang/rust-analyzer
[analyzer-guide]: https://rust-analyzer.github.io/book/contributing
[analyzer-issues]: https://github.com/rust-lang/rust-analyzer/issues?q=is%3Aissue%20state%3Aopen%20label%3A%22good%20first%20issue%22
[miri-repo]: https://github.com/rust-lang/miri
[miri-guide]: https://github.com/rust-lang/miri/blob/master/CONTRIBUTING.md
[miri-issues]: https://github.com/rust-lang/miri/issues?q=is%3Aissue%20state%3Aopen%20label%3AE-good-first-issue
[rustup-repo]: https://github.com/rust-lang/rustup
[rustup-guide]: https://rust-lang.github.io/rustup/dev-guide
[rustup-issues]: https://github.com/rust-lang/rustup/issues?q=is%3Aissue%20state%3Aopen%20label%3A%22help%20wanted%22
[crates-io-repo]: https://github.com/rust-lang/crates.io
[crates-io-guide]: https://github.com/rust-lang/crates.io/blob/main/docs/CONTRIBUTING.md
[crates-io-issues]: https://github.com/rust-lang/crates.io/issues
[bors-repo]: https://github.com/rust-lang/bors
[bors-guide]: https://github.com/rust-lang/bors/blob/main/docs/development.md
[bors-issues]: https://github.com/rust-lang/bors/issues?q=is%3Aissue%20state%3Aopen%20label%3A%22good%20first%20issue%22
[rustc-perf-repo]: https://github.com/rust-lang/rustc-perf
[rustc-perf-issues]: https://github.com/rust-lang/rustc-perf/issues?q=is%3Aissue%20state%3Aopen%20label%3A%22help%20wanted%22
[triagebot-repo]: https://github.com/rust-lang/triagebot
[triagebot-issues]: https://github.com/rust-lang/triagebot/issues?q=is%3Aissue%20state%3Aopen%20label%3A%22good%20first%20issue%22
[playground-repo]: https://github.com/rust-lang/rust-playground
[playground-issues]: https://github.com/rust-lang/rust-playground/issues?q=is%3Aissue%20state%3Aopen%20label%3A%22help%20wanted%22
[rustlings-repo]: https://github.com/rust-lang/rustlings
[rustlings-guide]: https://github.com/rust-lang/rustlings/blob/main/CONTRIBUTING.md
[rustlings-issues]: https://github.com/rust-lang/rustlings/issues
[mdbook-repo]: https://github.com/rust-lang/mdBook
[mdbook-guide]: https://github.com/rust-lang/mdBook/blob/master/CONTRIBUTING.md
[mdbook-issues]: https://github.com/rust-lang/mdBook/labels/E-Help-wanted

If you want to get inspired, check all the [rust-lang][rust-lang-repos] repositories!

[rust-lang-repos]: https://github.com/orgs/rust-lang/repositories?type=all&q=sort%3Astars

### Different kinds of contributions

There are various ways in which you can contribute to Rust projects:

- Writing code is the most obvious one. However, it does not have to be only in Rust! Even though
  most of our projects are of course written in Rust, we also use other technologies. For example,
  you can help improve our GitHub CI workflows, automation Python scripts or contribute to web
  frontends with HTML/CSS/JS (e.g. [Rustdoc][rustdoc-ui] or [Benchmark suite website][rustc-perf-ui]).
  Play to your strengths!

- Improving documentation is possibly one of the easiest starting points for contributions.
  Did you find a typo, something that was unclear, or do you miss some useful information on this
  page, elsewhere in the Forge or in some other Rust (user-facing) documentation? Great, then send
  a pull request and you'll become a contributor! :)

  Please notice that at this time **we don't accept typography/spellcheck fixes to internal
  documentation** (usually not worth the churn or the review time) **or in our testsuite** (they
  could inadvertently cause code regressions)

- Improving tests is also very valuable, as there is never enough of them. This can include also
  documenting existing tests or writing tests for already fixed issues that lack a proper test.

- You can also help with improving issue trackers of our repositories, by helping with [triaging issues][issue-triage]
  or by reproducing old issues to find out whether they are still valid or not.

- Or, if you like programming language discussions, you could participate in our [RFC process](https://github.com/rust-lang/rfcs).

- You can also answer questions to help other Rust users, in the _Get Help!_ channels on the [Rust Discord
  server][rust-discord], on [users.rust-lang.org][users] (URLO), or on [StackOverflow][so].

[rustdoc-ui]: https://rustc-dev-guide.rust-lang.org/rustdoc-internals.html
[rustc-perf-ui]: https://github.com/rust-lang/rustc-perf/tree/master/site
[rust-discord]: https://discord.gg/rust-lang
[users]: https://users.rust-lang.org/
[so]: http://stackoverflow.com/questions/tagged/rust
[rustup]: https://github.com/rust-lang/rustup
[internals]: https://internals.rust-lang.org
[rust-discord]: http://discord.gg/rust-lang
[rust-zulip]: https://rust-lang.zulipchat.com
[coc]: https://www.rust-lang.org/policies/code-of-conduct
[issue-triage]: ./release/issue-triaging.md
