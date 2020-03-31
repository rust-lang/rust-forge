# Maintaining the standard library

> Everything I wish I knew before somebody gave me `r+`

This document is an effort to capture some of the context needed to develop and maintain the Rust standard library. It’s goal is to help members of the Libs team share the process and experience they bring to working on the standard library so other members can benefit. It’ll probably accumulate a lot of trivia that might also be interesting to members of the wider Rust community.

This document doesn't attempt to discuss best practices or good style. For that, see the [API Guidelines].

## Contributing

If you spot anything that is outdated, under specified, missing, or just plain incorrect then feel free to open up a PR on the [`rust-lang/rust-forge`] repository!

## Terms

- Libs. That's us! The team responsible for development and maintenance of the standard library (among other things).
- Pull request (PR). A regular GitHub pull request against [`rust-lang/rust`].
- Request for Comment (RFC). A formal document created in [`rust-lang/rfcs`] that introduces new features.
- Tracking Issue. A regular issue on GitHub that’s tagged with `C-tracking-issue`.
- Final Comment Period (FCP). Coordinated by [`rfcbot`] that gives relevant teams a chance to review RFCs and PRs.

## If you’re ever unsure…

Maintaining the standard library can feel like a daunting responsibility! Through [`highfive`], the automated reviewer assignment, you’ll find yourself dropped into a lot of new contexts.

Ping the `@rust-lang/libs` team on GitHub anytime. We’re all here to help!

If you don’t think you’re the best person to review a PR then use [`highfive`] to assign it to somebody else.

## Finding reviews waiting for your input

Please remember to regularly check https://rfcbot.rs/. Click on any occurrence of your nickname to go to a page like https://rfcbot.rs/fcp/SimonSapin that only shows the reviews that are waiting for your input.

## Reviewing PRs

As a member of the Libs team you’ll find yourself assigned to PRs that need reviewing, and your input requested on issues in the Rust project.

### When is an RFC needed?

New unstable features don't need an RFC before they can be merged. If the feature is small, and the design space is straightforward, stabilizing it usually only requires the feature to go through FCP. Sometimes however, you may ask for an RFC before stabilizing.

### Is there any `unsafe`?

Unsafe code blocks in the standard library need a comment explaining why they're [ok](https://doc.rust-lang.org/nomicon). There's a `tidy` lint that checks this. The unsafe code also needs to actually be ok.

The rules around what's sound and what's not can be subtle. See the [Unsafe Code Guidelines WG] for current thinking, and consider pinging `@rust-lang/libs`, `@rust-lang/lang`, and/or somebody from the WG if you're in _any_ doubt. We love debating the soundness of unsafe code, and the more eyes on it the better!

### Is that `#[inline]` right?

Inlining is a trade-off between potential execution speed, compile time and code size.

You should add `#[inline]`:

- To public, small, non-generic functions.

You shouldn’t need `#[inline]`:

- On methods that have any generics in scope.
- On methods on traits that don’t have a default implementation.

#### What about `#[inline(always)]`?

You should just about never need `#[inline(always)]`. It may be beneficial for private helper methods that are used in a limited number of places or for trivial operators. A micro benchmark should justify the attribute.

### Is there any potential breakage?

Breaking changes should be avoided when possible. [RFC 1105] lays the foundations for what constitutes a breaking change. Breakage may be deemed acceptable or not based on its actual impact, which can be approximated with a [`crater`] run.

There are strategies for mitigating breakage depending on the impact.

For changes where the value is high and the impact is high too:

- Using compiler lints to try phase out broken behavior.

If the impact isn't too high:

- Looping in maintainers of broken crates and submitting PRs to fix them.

### Are there new impls for stable traits?

A lot of PRs to the standard library are adding new impls for already stable traits, which can break consumers in many weird and wonderful ways. The following sections gives some examples of breakage from new trait impls that may not be obvious just from the change made to the standard library.

#### Inference breaks when a second generic impl is introduced

Rust will use the fact that there's only a single impl for a generic trait during inference. This breaks once a second impl makes the type of that generic ambiguous. Say we have:

```rust
// in `std`
impl From<&str> for Arc<str> { .. }
```

```rust
// in an external `lib`
let b = Arc::from("a");
```

then we add:

```diff
impl From<&str> for Arc<str> { .. }
+ impl From<&str> for Arc<String> { .. }
```

then

```rust
let b = Arc::from("a");
```

will no longer compile, because we've previously been relying on inference to figure out the `T` in `Box<T>`.

This kind of breakage can be ok, but a [`crater`] run should estimate the scope.

#### Deref coercion breaks when a new impl is introduced

Rust will use deref coercion to find a valid trait impl if the arguments don't type check directly. This only seems to occur if there's a single impl so introducing a new one may break consumers relying on deref coercion. Say we have:

```rust
// in `std`
impl Add<&str> for String { .. }

impl Deref for String { type Target = str; .. }
```

```rust
// in an external `lib`
let a = String::from("a");
let b = String::from("b");

let c = a + &b;
```

then we add:

```diff
impl Add<&str> for String { .. }
+ impl Add<char> for String { .. }
```

then

```rust
let c = a + &b;
```

will no longer compile, because we won't attempt to use deref to coerce the `&String` into `&str`.

This kind of breakage can be ok, but a [`crater`] run should estimate the scope.

### Could an implementation use existing functionality?

Types like `String` are implemented in terms of `Vec<u8>` and can use methods on `str` through deref coersion. `Vec<T>` can use methods on `[T]` through deref coersion. When possible, methods on a wrapping type like `String` should defer to methods that already exist on their underlying storage or deref target.

### Are there `#[fundamental]` items involved?

Blanket trait impls can't be added to `#[fundamental]` types because they have different coherence rules. See [RFC 1023] for details. That includes:

- `&T`
- `&mut T`
- `Box<T>`
- `Pin<T>`

### Is specialization involved?

[#68358]: https://github.com/rust-lang/rust/pull/68358
[#67194]: https://github.com/rust-lang/rust/issues/67194
[lang_design_specialization]: https://paper.dropbox.com/doc/Specialization-Review-2020-02-03--AuBwAqGbsHDmBlBy~XUlmqUcAg-jFYgiknJi6j00SbN83dWX

**NOTE(2019-02-10):** Due to recent soundness holes introduced by specialization in the standard library (c.f. [#68358] and [#67194]) the language team decided on a design meeting to place a moratorium on new uses of specialization until we have some checks in place ensuring soundness for internal uses.

We try to avoid leaning on specialization too heavily, limiting its use to optimizing specific implementations. These specialized optimizations use a private trait to find the correct implementation, rather than specializing the public method itself. Any use of specialization that changes how methods are dispatched for external callers should be carefully considered.

### Are there public enums?

Public enums should have a `#[non_exhaustive]` attribute if there's any possibility of new variants being introduced, so that they can be added without causing breakage.

### Does this change drop order?

Changes to collection internals may affect the order their items are dropped in. This has been accepted in the past, but should be noted.

### How could `mem` break assumptions?

#### `mem::replace` and `mem::swap`

Any value behind a `&mut` reference can be replaced with a new one using `mem::replace` or `mem::swap`, so code shouldn't assume any reachable mutable references can't have their internals changed by replacing.

#### `mem::forget`

Rust doesn't guarantee destructors will run when a value is leaked (which can be done with `mem::forget`), so code should avoid relying on them for maintaining safety. Remember, [everyone poops][Everyone Poops].

It's ok not to run a destructor when a value is leaked because its storage isn't deallocated or repurposed. If the storage is initialized and is being deallocated or repurposed then destructors need to be run first, because [memory may be pinned][Drop guarantee]. Having said that, there can still be exceptions for skipping destructors when deallocating if you can guarantee there's never pinning involved.

### How is performance impacted?

Changes to hot code might impact performance in consumers, for better or for worse. Appropriate benchmarks should give an idea of how performance characteristics change. For changes that affect `rustc` itself, you can also do a [`rust-timer`] run.

### Is the commit log tidy?

PRs shouldn’t have merge commits in them. If they become out of date with `master` then they need to be rebased.

### Are functions const-stabilized or constified?

Please CC `@rust-lang/wg-const-eval`.

## Merging PRs

PRs to [`rust-lang/rust`] aren’t merged manually using GitHub’s UI or by pushing remote branches. Everything goes through [`bors`].

### When to `rollup`

For Libs PRs, rolling up is usually fine, in particular if it's only a new unstable addition or if it only touches docs (with the exception of intra doc links which complicates things while the feature has bugs...).

If a submodule is affected then probably don't `rollup`. If the feature affects perf then also avoid `rollup` -- mark it as `rollup=never`.

### When there’s new public items

If the feature is new, then a tracking issue should be opened for it. Have a look at some previous [tracking issues][Libs tracking issues] to get an idea of what needs to go in there. The `issue` field on `#[unstable]` attributes should be updated with the tracking issue number.

Unstable features can be merged as normal through [`bors`] once they look ready.

### When there’s new trait impls

There’s no way to make a trait impl for a stable trait unstable, so **any PRs that add new impls for already stable traits must go through a FCP before merging.** If the trait itself is unstable though, then the impl needs to be unstable too.

### When a feature is being stabilized

Features can be stabilized in a PR that replaces `#[unstable]` attributes with `#[stable]` ones. The feature needs to have an accepted RFC before stabilizing. They also need to go through a FCP before merging.

You can find the right version to use in the `#[stable]` attribute by checking the [Forge].

[API Guidelines]: https://rust-lang.github.io/api-guidelines
[Unsafe Code Guidelines WG]: https://github.com/rust-lang/unsafe-code-guidelines
[`rust-lang/rust`]: https://github.com/rust-lang/rust
[`rust-lang/rfcs`]: https://github.com/rust-lang/rfcs
[`rust-lang/rust-forge`]: https://github.com/rust-lang/rust-forge
[`rfcbot`]: https://github.com/rust-lang/rfcbot-rs
[`bors`]: https://github.com/rust-lang/homu
[`highfive`]: https://github.com/rust-lang/highfive
[`crater`]: https://github.com/rust-lang/crater
[`rust-timer`]: https://github.com/rust-lang-nursery/rustc-perf
[Libs tracking issues]: https://github.com/rust-lang/rust/issues?q=label%3AC-tracking-issue+label%3AT-libs
[Drop guarantee]: https://doc.rust-lang.org/nightly/std/pin/index.html#drop-guarantee
[Forge]: https://forge.rust-lang.org/
[RFC 1023]: https://rust-lang.github.io/rfcs/1023-rebalancing-coherence.html
[RFC 1105]: https://rust-lang.github.io/rfcs/1105-api-evolution.html
[Everyone Poops]: http://cglab.ca/~abeinges/blah/everyone-poops
