# How the Rust CI works

Continuous integration (CI) workflows on the `rust-lang/rust` repository ensure that the default branch
is always in a valid state.

The CI infrastructure is described in detail in the [rustc-dev-guide][rustc-dev-guide].

## `rust-lang/rust` CI invariant

Changes to the default and `beta`/`stable` release branches of the [rust-lang/rust] repository go
through two sets of CI jobs:

1. **PR CI**. This is a fast and restricted set of CI jobs that run against PRs to provide faster
   feedback.
2. **Merge CI**: This is the full set of CI jobs that all changes to [rust-lang/rust] must pass.
   This includes, but are not limited to, e.g. testing jobs for all Tier 1 targets.

We enforce an [rust-lang/rust] CI invariant where **PR CI jobs are a subset of Merge CI jobs modulo
carve outs**. The carve out differences between PR CI and Merge CI jobs are tracked in
<https://github.com/rust-lang/rust/issues/144259>. The reason for this subset invariant is because
if you have PR-only CI jobs, a PR can fail the PR-only jobs but pass the Merge CI. This can cause
all subsequent PR CI to fail, in _completely unrelated PRs_, which is very frustrating for other
contributors.


[rust-lang/rust]: https://github.com/rust-lang/rust
[rustc-dev-guide]: https://rustc-dev-guide.rust-lang.org/tests/ci.html
