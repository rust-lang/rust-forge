# rustc-dev-guide

The rustc-dev-guide working group is responsible for maintaining the rustc-dev-guide (located at 
[rust-lang/rustc-dev-guide]). This includes things such
as: 
- Performing subtree syncs between the main rust repo and the rustc-dev-guide repo
- Triaging the state of the guide to look for out of date information or missing information
- Miscellaneous editorial work of the pages in the guide or fixing links that have bitrot
- Reviewing simple PRs to the guide that don't need domain-specific expertise
- Connect domain-specific doc changes with domain expert reviewers

## Review policy

The dev guide has a much lower bar for changes to merged compared to the compiler itself. Incomplete
and/or WIP documentation is preferred over no documentation. Stubbed out TODOs with issues tracking 
the missing pieces for sub-parts of chapters are useful to indicate known gaps in documentation that
may otherwise leave the reader confused.

Contributors confident in the area being documented should also feel free to merge their own
documentation without review. For this reason the dev guide does not have a reviewer automatically
assigned to each PR, instead PR authors can manually request a PR reviewer if they believe it necessary.

A reviewer can be assigned by writing a comment containing: `r? rustc-dev-guide` or `r? @username`. 

A good rule of thumb for whether you should feel comfortable merging your own rustc-dev-guide PR is whether
you would feel comfortable approving an involved PR touching the relevant area of the compiler. See the
compiler [review policy](../compiler/reviews.md).

## Where to contribute `rustc-dev-guide` changes

If your change **only involves the documentation content of rustc-dev-guide** and **does not accompany `rust-lang/rust` code changes**,
please submit your changes and PRs directly to the [rust-lang/rustc-dev-guide] repository.

There are some benefits following this rule:
- Changes to `rustc-dev-guide` repo can be immediately reflected in [the live rustc-dev-guide](https://rustc-dev-guide.rust-lang.org/).
- Changes to `rustc-dev-guide` repo do not need to go through bors CI in `rust-lang/rust`.
- Less burden on bors queue in `rust-lang/rust`.

## Subtree syncs

The dev guide is a [josh](https://josh-project.github.io/josh/intro.html) subtree of the main rust-lang/rust
repo. This makes it easier for compiler contributors to update documentation in the dev guide in tandem with
changes to the compiler itself.

This necessitates a manual process of syncing any changes made to the dev guide between its own repo, and the
copy maintained in the rust-lang/rust repo.

1. Wait at least a week since the last subtree sync took place.
2. There will either be an automatically opened rustc-pull PR, or a manual rustc-pull will be needed if there are merge 
conflicts. If there is a rustc-pull PR open, merge it[^1], otherwise a manual rustc-pull should be performed.
See [rustc-dev-guide#2451] for an example rustc-pull.
3. Post in the [zulip channel for coordinating subtree syncs][subtree_coordination] that you're doing a sync
4. Do a manual rustc-push and open the PR. The PR should be assigned to a rustc-dev-guide lead
(currently @BoxyUwU or @jieyouxu). We tend to assign rustc-push PRs to people instead of self
approving like some other teams. This is because there were some accidents when the subtree was originally
setup so it felt scary to not have someone check things over.
5. Post the rustc-push PR in the [zulip channel for coordinating subtree syncs][subtree_coordination] so everyone is aware that a subtree
sync has been performed. See [rust-lang/rust#141962] for an example rustc-push.


Documentation about *how* to perform a rustc-pull or rustc-push can be found in the dev guide's README.

[^1]: If the automatically opened rustc-pull doesn't contain any actual changes to the dev
guide, then the subtree sync can be delayed until there are actual changes to sync.

[rust-lang/rust#141962]: https://github.com/rust-lang/rust/pull/141962
[rustc-dev-guide#2451]: https://github.com/rust-lang/rustc-dev-guide/pull/2451
[subtree_coordination]: https://rust-lang.zulipchat.com/#narrow/channel/196385-t-compiler.2Frustc-dev-guide/topic/Subtree.20sync.20automation/with/522133712
[rust-lang/rustc-dev-guide]: https://github.com/rust-lang/rustc-dev-guide
