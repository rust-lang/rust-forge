# Maintaining the standard library

> Everything I wish I knew before somebody gave me `r+`

This document is an effort to capture some of the context needed to develop and maintain the Rust standard library. It’s goal is to help members of the Libs team share the process and experience they bring to working on the standard library so other members can benefit. It’ll probably accumulate a lot of trivia that might also be interesting to members of the wider Rust community.

This document doesn't attempt to discuss best practices or good style. For that, see the [API Guidelines].

## Terms

- Libs. That's us! The team responsible for development and maintenance of the standard library (among other things).
- Pull request (PR). A regular GitHub pull request against [`rust-lang/rust`].
- Request for Comment (RFC). A formal document created in [`rust-lang/rfcs`] that introduces new features.
- Tracking Issue. A regular issue on GitHub that’s tagged with `C-tracking-issue`.
- Final Comment Period (FCP). Coordinated by [`rfcbot`] that gives relevant teams a chance to review RFCs and PRs.

## If you’re ever unsure…

Maintaining the standard library can feel like a daunting responsibility! Through the automated reviewer assignment you’ll find yourself dropped into a lot of new contexts.

Ping the `@rust-lang/libs` team on GitHub anytime. We’re all here to help!

If you don’t think you’re the best person to review a PR then use [`bors`] to assign it to somebody else.

## Finding reviews waiting for your input

Please remember to regularly check https://rfcbot.rs/. Click on any occurrence of your nickname to go to a page like https://rfcbot.rs/fcp/SimonSapin that only shows the reviews that are waiting for your input.

## Reviewing PRs

As a member of the Libs team you’ll find yourself assigned to PRs that need reviewing, and your input requested on issues in the Rust project.

### When is an RFC needed?

New unstable features don’t need a RFC before they can be merged. They do need one before they can be stabilized. New impls for already stable traits don't need a RFC. If a new unstable feature is large or has a lot of design space to explore then you might want to block merging it on a RFC.

### Is there any `unsafe`?

Unsafe code blocks in the standard library need a comment explaining why they're [ok](https://doc.rust-lang.org/nomicon). There's a `tidy` lint that checks this. The unsafe code also needs to actually be ok.

The rules around what's sound and what's not can be subtle. See the [Unsafe Code Guidelines WG] for current thinking, and consider pinging `@rust-lang/libs` and/or somebody from the WG if you're in any doubt.

### Is that `#[inline]` right?

Inlining is a trade-off between potential execution speed, compile time and code size.

You should add `#[inline]`:

- To public, small, non-generic functions.

You shouldn’t need `#[inline]`:

- On methods that have any generics in scope.
- On methods on traits that don’t have a default implementation.
- On `const` items.

#### What about `#[inline(always)]`?

You should just about never need `#[inline(always)]`. It may be beneficial for private helper methods that are used in a limited number of places or for trivial operators. A micro benchmark should justify the attribute.

### Is there any potential breakage?

Breaking changes should be avoided when possible. [RFC 1105] lays the foundations for what constitutes a breaking change. Breakage may be deemed acceptable or not based on its actual impact, which can be approximated with a `crater` run.

For changes where the value is high and the impact is high too, there are strategies for minimizing the impact:

- Using compiler lints to try phase out broken behavior.

### Could this affect inference?

New impls on public traits for public items may cause inference to break when there are generics involved.

#### Are there `#[fundamental]` items involved?

Blanket trait impls can't be added to `#[fundamental]` types because they have different coherence rules. That includes:

- `Box<T>`
- `Pin<T>`

Also see [RFC 1023] for details.

### Is there specialization involved?

We try to avoid leaning on specialization too heavily, limiting its use to optimizing specific implementations. Any use of specialization that changes how methods are dispatched for external callers should be carefully considered.

### Does this change drop order?

Changes to collection internals may affect the order their items are dropped in. This has been accepted in the past, but should be noted.

### Could `mem::replace` break assumptions?

Any value behind a `&mut` reference can be replaced with a new one.

### Could `mem::forget` break assumptions?

Rust doesn't guarantee destructors will run, so code should avoid relying on them for safety. Remember, [everybody poops][Everybody Poops].

### Is the commit log tidy?

PRs shouldn’t have merge commits in them. If they become out of date with `master` then they need to be rebased.

## Merging PRs

PRs to `rust-lang/rust` aren’t merged manually using GitHub’s UI or by pushing remote branches. Everything goes through [`bors`].

### When you’re confident it’ll build

Consider explicitly specifying `rollup`.

### When there’s new public items

If the feature is new, then a tracking issue should be opened for it. The `issue` field on `#[unstable]` attributes should be updated with the tracking issue number.

Unstable features can be merged as normal through [`bors`] once they look ready.

### When there’s new trait impls

There’s no way to make a trait impl `#[unstable]`, so **any PRs that add new impls for already `#[stable]` traits must go through a FCP before merging.**

### When a feature is being stabilized

Features can be stabilized in a PR that replaces `#[unstable]` attributes with `#[stable]` ones. The feature needs to have an accepted RFC before stabilizing. They also need to go through a FCP before merging.

[API Guidelines]: https://rust-lang.github.io/api-guidelines
[Unsafe Code Guidelines WG]: https://github.com/rust-lang/unsafe-code-guidelines
[`rust-lang/rust`]: https://github.com/rust-lang/rust
[`rust-lang/rfcs`]: https://github.com/rust-lang/rfcs
[`rfcbot`]: https://github.com/rust-lang/rfcbot-rs
[`bors`]: https://github.com/rust-lang/homu
[RFC 1023]: https://rust-lang.github.io/rfcs/1023-rebalancing-coherence.html
[RFC 1105]: https://rust-lang.github.io/rfcs/1105-api-evolution.html
[Everyone Poops]: http://cglab.ca/~abeinges/blah/everyone-poops
