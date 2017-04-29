---
layout: default
title: How to prepare Rust release notes &middot; The Rust Programming Language
---

_[Originally posted to the "milestone predictions" thread on internals.rlo](https://internals.rust-lang.org/t/rust-release-milestone-predictions/4591/33)._

Ideally the notes for the next release are compiled at the beginning
of the beta cycle, 6 weeks ahead of the release.

Copy the previous version's section in README.md to a new section and
strip out all the items. Reuse the existing section structure, but
look out for themes in the release that might warrant other
sections. e.g. sometimes there's a lot of interesting perf commits, or
rustdoc commits.

Before I start I find the date ranges the release was on master, for
making GitHub queries, and the commit ranges for the entire release,
as well as commit ranges for just the releases backports.

I only pick out PRs that impact stable features. Currently, nightly
development does not have a place in the release notes. Generally,
more exciting items go toward the top of sections. Most items are
simply links to the PR that landed them; some that need more
explanation have additional, unlinked text; anything supported by an
RFC has an additional RFC link. I reuse the PR titles or write my
own descriptions as needed for clarity.

The main query I do is against GitHub merges for rust-lang/rust for
the correct date range, e.g.:

    https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Amerged+merged%3A2017-02-02..2017-03-16

Pick out interesting PRs, especially those tagged 'relnotes'. Because
the date ranges are imprecise, I'll verify commits at the edges of the
dates against git, using a command like:

    git log `git merge-base origin/stable 1.16.0`..origin/stable | grep $commit

Note that the branches might be different depending on when you
compile the notes.

Then I look for the "libs stabilization" PR, which contains a bunch
of newly-stable APIs.

Then I check git for the range of backports. Sometimes there are
things there worth mentioning that don't show up in the GitHub query,
but most backports are to fix regressions that never hit stable, so
aren't worth mentioning. The query is something like

    git log `git merge-base origin/stable origin/beta`..origin/stable

Again the branch names may be different.

Then I do the same with rust-lang/cargo. When RLS is in-tree we'll do
the same with rust-lang/rls.

There may be a GitHub milestone for the release. Check that to see if
there's anything worth talking about.

Finally, compare the results to this milestone prediction thread and make
sure they agree, updating the predictions as necessary.

## Example data for relnotes prep

- release: 2017-04-27
- on master: 2017-02-02 - 2017-03-16
- all commits: git log `git merge-base origin/stable 1.16.0`..origin/stable
- just backports: git log `git merge-base origin/stable origin/beta`..origin/stable

- checklist
  - [ ] rust prs
    - https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Amerged+merged%3A2017-02-02..2017-03-16
  - [ ] rust backports
  - [ ] libs stabilization pr
  - [ ] cargo prs
    - https://github.com/rust-lang/cargo/pulls?q=is%3Apr+is%3Amerged+merged%3A2017-02-02..2017-03-16
  - [ ] cargo backports
  - [ ] check on GitHub release milestone
  - [ ] compare to on-thread milestone predictions

## How to prepare milestone predictions

Maintaining the [milestone predictions] is a less exact process. It's
best to always be on the lookout for new information about what's in
the pipeline, and update the thread incrementally.

[milestone predictions]: https://internals.rust-lang.org/t/rust-release-milestone-predictions/4591/33

Once a release though I do sit down and look for upcoming features. Generally
what I do is:

- go through the existing predictions and re-estimate them
- look for final-comment-period tags on rust-lang/rust and rust-lang/rfcs and
  try to divine new estimates from that
- look for B-unstable tags on rust-lang/rust and see if there's any
  notable activity
- compare the predictions to the actual release notes and bump features
  as appropriate

When I post updates to the predictions I also post a message
indicating what was updated.
