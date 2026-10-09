# Target tiers

The standard library follows the compiler's [target tier policy], although there are a few extra things to note when maintaining standard library APIs.

[target tier policy]: https://doc.rust-lang.org/nightly/rustc/target-tier-policy.html

## Tier 1

Tier 1 targets are the most well-supported targets in Rust, and we test every tier 1 target on every change being merged into the `main` branch. However, due to CI limitations, many tier-1 targets are not tested on every new commit to a PR. Among the "big three" tier 1 platforms, Windows, Mac OS, and Linux, only Linux is actually tested in PR CI. This means that changes that contain Windows- or Mac-specific code should be either manually tested or run via [`@bors try`] to ensure that they pass basic tests before being [rolled up], where the tests will run and potentially block multiple changes.

[`@bors try`]: https://rustc-dev-guide.rust-lang.org/tests/ci.html#try-builds
[rolled up]: https://forge.rust-lang.org/compiler/reviews.html?highlight=rollup#rollups

Since tier 1 targets are important, we also want to go out of our way to ensure that the standard library's functionality works on all tier 1 targets. This generally means that if there are any blockers for supporting a proposed API on a tier 1 target, this should block that API from being stabilized. Additionally, we should expect the library team to have reasonable coverage of expertise on tier 1 targets; reviewers may wish to get acquainted with which team members are familiar with which targets if they ever need help reviewing for tier 1 targets.

## Tier 2

Tier 2 targets are supported by Rust's infrastructure, but there is a lower burden on the project itself to maintain them. In particular, tier 2 targets may not support the entire standard library (for some, just `core`), and they may skip some tests. Tier 2 targets are, in particular, *never* tested in PR CI, which means that changes affecting tier 2 targets should explicitly run [`@bors try`] on the relevant jobs before being [rolled up], where the tests will run and potentially block multiple changes.

Rather than being directly supported by the library team, we expect tier 2 targets to mostly be maintained by their [target maintainers], although the library team will help out wherever it's appropriate. Due to the reduced expertise and support for tier 2 targets, we generally prefer to apply a few ground rules for tier 2 changes:

[target maintainers]: https://doc.rust-lang.org/nightly/rustc/platform-support.html

* Whenever possible, *target-independent* code for tier 2 targets (data structures, etc.) should be built and tested on all targets. This makes it easier to maintain code which doesn't require the target-specific expertise.
* If a function for a tier 2 target is not implemented (unconditionally panics, returns an error with `io::ErrorKind::Unimplemented`, etc.) a comment should be left indicating whether this is simply not implemented, or if it's impossible to implement for the given target, and why.
* If a tier 2 target deviates from usual functionality, it should be clearly documented. (For example, if a target does not have symbolic links, it should explain that in code that normally would handle symbolic links.)

Note that we should expect people other than target maintainers to be regularly making changes to tier 2 targets. This is the main criteria that distinguishes tier 2 from tier 3: more people care about these targets than *just* the target maintainers. There may even be library team members that have experience with these targets, or at least compiler team members willing to help. However, in general, we should not be bending over backwards to support tier 2 targets; in many cases, it's perfectly reasonable to ignore a test or not implement a function for a tier 2 target and leave it for later, even potentially after a feature is stabilized.

## Secret tier 1.5

The compiler target tier policy aplies to targets, which are strictly a combination of operating system and architecture. In general, when talking about tier 2 targets, we usually talk about obscure operating systems. However, there are some cases where we do care about architectures *other* than the ones supported by tier 1 targets, to an extent greater than the usual tier 2 support, specifically for [`core::arch`].

[`core::arch`]: https://doc.rust-lang.org/nightly/core/arch/index.html

In general, we should have decent coverage among the library team on the architectures specifically supported by `core::arch`, and we probably do want to check each of the main supported architectures for coverage on low-level features added to `core`. This is technically separate from the target tier policy, but worth keeping in mind.

## Tier 3

Tier 3 targets are the lowest level of support, which is effectively… yeah, we'll accept your code. Unless they have overlap with higher-tier targets, tier 3 targets won't be tested at all in CI, and we should rely entirely on [target maintainers] to support them.

With that said, all of the notes from tier 2 apply, in particular that these targets should still be documented whenever they show up. Many reviewers suffer from the affliction of being too nice to people and will still review tier 3 changes, although this is strictly not required.

The main exception to tier 3 changes is, of course, tech debt: we should try as hard as possible to keep tier 3 code to a minimum, and if we end up just copying the same boilerplate for 50 different targets, that makes changing the standard library unreasonably difficult. We should try to consolidate code for multiple tier 3 targets whenever possible.
