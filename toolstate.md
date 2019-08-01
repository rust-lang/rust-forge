---
layout: default
title: Handling of tools embedded in the rustc repo ("toolstate")
---

# Handling of tools embedded in the rustc repo ("toolstate")

The Rust repository contains several external tools and documents as git
submodules (e.g. clippy, rls, the [Book], the [Reference]). Many of those are
very tightly coupled to the compiler and depend on internal APIs that change all
the time, but they are not actually essential to get the compiler itself to
work. To make API changes less painful, these tools are allowed to "break"
temporarily. PRs can still land and nightlies still get released even when some
tools are broken. Their current status is managed by the
[toolstate system][toolstate]. (Cargo is not subject to the toolstate system and
instead just has to always work.)

The three possible states of a "tool" (this includes the documentation managed
by the toolstate system, where we run doctests) are: `test-pass`, `test-fail`,
`build-fail`.

This page gives a rough overview how the toolstate system works, and what the
rules are for when which tools are (not) allowed to break.

## Toolstate Rules

* For all tools, if a PR changes that tool (if it changes the commit used by the
  submodule), the tool has to be in `test-pass` after this PR or else CI will
  fail.

* For all tools except for "nightly only" tools, the following extra rules are applied:
    * If a PR lands on the `beta` or `stable` branch, the tool has to be `test-pass`.
    * If a PR lands on `master` in the week before the beta is cut, and that PR
      regresses the tool (if it makes the state "worse"), CI fails. This is to
      help make sure all these tools become `test-pass` so that a beta can be
      cut. (See the [Forge index][forge] for when the next beta cutoff is
      happening.)

    At the time of writing, the following tools are "nightly only": rustc-guide,
    miri, embedded-book.

## Updating the toolstate repository

Updating the [toolstate repository] happens in two steps: when CI
runs on the `auto` branch (where bors moves a PR to test if it is good for
integration), the "tool" runners for the individual platforms (at the time of
writing, Linux and Windows) each submit a JSON file to the repository recording
the state of each tool for the commit they are testing. Later, if that commit
actually entirely passed CI and bors moves it to the `master` branch, the
"current tool status" in the toolstate repository is updated appropriately.

These scripts also automatically ping some people and create issues when tools
break.

For further details, see the comments in the involved files: [`checktools.sh`],
[`publish_toolstate.py`] as well as the other files mentioned there.

## Adding a tool

To add a new tool to be tracked, the following steps must be taken:

1. Create a PR to rust-lang/rust that adds the submodule along with any
   necessary build system / bootstrap updates. Be careful that the tests
   properly support `./x.py --no-fail-fast` to avoid
   [issues like this](https://github.com/rust-lang/rust/pull/63089).
2. Include changes to [`checktools.sh`]:
    - Build the tool at the top. This is the step that actually generates the
      JSON status for the tool. When `save-toolstates` is set in
      `config.toml`, the rust build system will write a JSON file with the
      status of each test.
    - Add the tool to `status_check` with whether it should be a beta blocker
      or not.
3. Update [`publish_toolstate.py`] to add the tool. This includes a list of
   people to ping if the tool is broken, and its source repo. (Note: At the
   time of this writing, these users must have permissions to be assignable on
   rust-lang/rust GitHub.)
4. Submit a PR to the [toolstate repository] to manually add the tool to the
   [`latest.json`] file.

[`checktools.sh`]: https://github.com/rust-lang/rust/blob/master/src/ci/docker/x86_64-gnu-tools/checktools.sh
[`publish_toolstate.py`]: https://github.com/rust-lang/rust/blob/master/src/tools/publish_toolstate.py
[`latest.json`]: https://github.com/rust-lang-nursery/rust-toolstate/blob/master/_data/latest.json
[Book]: https://doc.rust-lang.org/book/
[Reference]: https://doc.rust-lang.org/reference/
[toolstate]: https://rust-lang-nursery.github.io/rust-toolstate/
[toolstate repository]: https://github.com/rust-lang-nursery/rust-toolstate/
[forge]: index.html
