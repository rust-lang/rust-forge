# Preparing Release Notes

This document discusses the process for preparing release notes (low level
changelog) and the blog post (high level changelog) announcing the Rust
releases.

## Automation/community: Step 1: PR or issue is labeled for release note consideration

This step happens for `relnotes`, `relnotes-perf`, and `finished-final-comment-period`
labels in rust-lang/rust and is managed by triagebot
([implementation](https://github.com/rust-lang/triagebot/blob/38b904f010338e3847bf1eba651356985c6f1df1/src/handlers/relnotes.rs#L67)).

Note that this step happens **on labeling**, not necessarily when the issue/PR
is merged/closed. (FIXME: Should we move this to close time, to make it more
likely that authors see the relnotes issue when it's warranted, particularly
for tracking issues?).

## Automation/community: Step 2: Blog post or release note text is proposed

Once labeled, a new issue is automatically filed with title "Tracking issue for
release notes of #xxxx: $original pr title". This is labeled with relnotes +
relnotes-tracking-issue.

The Rust team responsible for the area and/or contributors making the PR can
(should) use this issue to propose any release note text. The issue contains a
code block for the release note contents (to go into RELNOTES.md) and for the blog.

Ideally, text is directly edited into the code block, but if permissions aren't
available discussion can happen on the thread and will be absorbed by the
release team later.

The release note text is automatically pulled in subsequent steps, and should use headers from [this list] if possible:

[this list]: https://github.com/rust-lang/relnotes/blob/33e78d703a439c8721705b26e2613ec6dac0cb4f/src/main.rs#L444-L449

* Compatibility Notes
* Library
* Stabilized APIs
* Const Stabilized APIs
* Language
* Compiler
* Internal Changes
* Other

Stabilized APIs and Const Stabilized APIs should both be formatted roughly as follows:

```
- [`std::ptr::null_mut`](https://doc.rust-lang.org/std/ptr/fn.null_mut.html)
```

Note that:

* this is not a PR link, but directly links the standard library docs.
* the link is to stable docs (and so may not actually work at time of writing)
* the API is directly noted. Sometimes we compress APIs (e.g., `uN` for
  unsigned integers) to avoid too much text.

## Release team: Step 3: Confirm all issues/PRs needing relnotes are labeled `relnotes`

This steps should happen in the first 3 weeks of the beta period (earlier is
better). This can be done with help from the wider Rust project too.

[Search] for `is:pr milestone:1.85.0 is:merged -label:relnotes -label:relnotes-perf -label:finished-final-comment-period` in GitHub on rust-lang/rust PRs, updating the milestone appropriately.

This should find all merged PRs that haven't already been nominated. Typically
there are several hundred of them; the goal is to try to find anything that
jumps out as *should have been nominated* and nominate it by tagging with
relnotes. Scrolling through the list without clicking through and using the
GitHub checkbox UI to mass-label issues is a good strategy.

The goal here is mostly **catching obvious things**, not 100% exhaustiveness.
It's generally OK if we miss something. If there's a consistent pattern, note
it down for inclusion in triagebot's automatic issue filing.

[Search]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+milestone%3A1.85.0+is%3Amerged+-label%3Arelnotes+-label%3Arelnotes-perf+-label%3Afinished-final-comment-period

FIXME: This step may also need to include an attempt to milestone any
**issues** that got tagged relnotes and closed -- those currently don't get
milestones associated with them, which means their corresponding relnotes
tracking issue is probably never included (letting it get missed indefinitely).

FIXME: Process for Compatibility Notes?

## Release team: Step 4: Release notes generation starts

This steps should happen in the first 3 weeks of the beta period (earlier is
better). The release team owner for the current release will run the [relnotes]
tool.

```shell
cargo build
GITHUB_TOKEN=$(gh auth token) cargo run --bin relnotes -- 1.85.0 > relnotes.md
```

This produces console output (stderr) like this:

```text
Did not use "Libraries" from Tracking issue for release notes of #132515: Fix and undeprecate home_dir() <https://github.com/rust-lang/rust/issues/132650>
Did not use "Category (e.g. Language, Compiler, Libraries, Compatibility notes, ...)" from Tracking issue for release notes of #132187: Add Extend impls for tuples of arity 1 through 12 <https://github.com/rust-lang/rust/issues/133975>
```

These lines typically mean that someone either hasn't updated or used a keyword not in the list above, for example:

* Libraries should be Library
* Category ... needs updating.

The runner of the tool should go update these issues with the right contents
(e.g., renaming/categorizing). The **source of truth** is the issue, not the
local output of the tool -- this is important as we want to automate tool
execution in the long run, so try to minimize the amount of local edits done on
the resulting `relnotes.md`.

The tool's standard output is directed to `relnotes.md` above, which will
currently contain formatted headers for each of the sections with content
pulled from the "Tracking issue for ..." **or** standard content (PR/issue
title + link) if there is no "tracking issue for ..." found by the tool.

If an issue is missing, see Step 3 (tag it and re-run the tool). If an issue is
present *but shouldn't be*, the best thing is to tag it with relnotes (or find
the pre-existing relnotes tracking issue) and *close that tracking issue*. This
will also drop the item from the tool's output.

## Release team: Step 5: Publish relnotes PR

See example from 1.84: <https://github.com/rust-lang/rust/pull/134568>

Take the `relnotes.md` you have locally (typically without library
stabilizations in today's world, you'll add them at a later point -- we want
the copy without them as early as possible), and insert it at the top of
RELEASES.md in rust-lang/rust, and open a new PR with those contents. You can
`r?` the owner for actually publishing the release for this cycle and cc the
release team.

Include a link to this document (https://forge.rust-lang.org/release/release-notes.html)
in the PR description, pointing at step 6 (i.e., prefer suggesting updates not on the PR).

The next release team meeting should also discuss this PR for selecting blog
post topics (see below for blog post process).

### Pinging `relnotes-interest-group` for relnotes PR and release blog post

Contributors may be interested to help review the relnotes PRs and release
blog posts (e.g. on behalf of their team). They can opt-in to being pinged by
adding themselves to the
[`relnotes-interest-group` marker team][relnotes-interest-group].

When creating a relnotes PR and release blog post, please ping this
notification group via

```
@rustbot ping relnotes-interest-group
```

[relnotes-interest-group]: https://github.com/rust-lang/team/blob/master/teams/relnotes-interest-group.toml

## All: Step 6: Incorporate edits from relnotes PR

You'll typically get a lot (several dozen) of comments on the PR with typo
fixes, suggestions for alternative text, etc. A good strategy is to try to
update the originating issues for issues/PRs (or file them and update them),
essentially matching the iteration already done locally in step 4. The longer
state stays in issues the easier it is to notice and incorporate updates from
those into the PR (just rerun the tool).

Pushing edits into the issues helps bring the right people (e.g., PR
author/reviewer) into the loop on what is getting discussed.

## Release team: Step 7: Close tracking issues

At some point, the release team owner should declare the PR authoritative and
close all relnotes tracking issues associated with the current milestone ([sample search](https://github.com/rust-lang/rust/issues?q=is%3Aissue%20state%3Aopen%20milestone%3A1.85.0%20label%3Arelnotes-tracking-issue)). Doing this in the GitHub UI is easiest.

FIXME: Ideally those would all get linked from the relnotes PR, so it's easier
to go back and forth between them.

# Blog post process

After publishing the release notes PR, the release team's next meeting will
discuss what blog post topics we want. Typically this is a pretty informal
discussion and isn't necessarily final; the goal is to provide input into the
blog post author's choice by providing more perspectives.

The blog post author will then aim to post the blog post PR as soon as possible
(typically 3-7 days out from the release, though we've had shorter turn arounds
too), where it will get reviewed, edited, and finally merged on the release day.

[relnotes]: https://github.com/rust-lang/relnotes
