# Rustc Commit Tracking

Triagebot keeps a database of commits to the [`rust-lang/rust`](https://github.com/rust-lang/rust/) repository.
This is useful since the GitHub API for fetching this information can be slow.
For example, this is used by the [rustc-perf](https://github.com/rust-lang/rustc-perf) system.

## Usage

The top-level bors merge commits can be fetched from <https://triage.rust-lang.org/bors-commit-list>.

## Configuration

This has no configuration, it is processed automatically.

## Implementation

See [`src/db/rustc_commits.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/src/db/rustc_commits.rs) and
[`src/handlers/rustc_commits.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/src/handlers/rustc_commits.rs).
