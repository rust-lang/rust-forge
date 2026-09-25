# API stabilization

Whenever we make a new API guarantee, it needs to go through a [Final Comment Period (FCP)][FCP] to ensure that there are no pending issues. This will require approval from all but 2 members of the [FCP team] and no outstanding objections from anyone on the larger libs team.

[FCP]: ../membership.md#fcp-process
[FCP team]: ../membership.md#fcp-membership

## Tracking issues

Changes that are marked as *unstable* should all have tracking issues to indicate their full history and status. Unstable APIs, marked via the `#[unstable]` attribute, are generally usable on the beta and nightly channels of the compiler and can be tested out before their final stabilization. Standard library tracking issues can be created [via a template][tracking issue template].

[tracking issue template]: https://github.com/rust-lang/rust/issues/new?template=library-tracking-issue.md&title=Tracking+issue+for+XXX

In general, tracking issues should include a full list of PRs made when implementing an issue as well as a description of the public API being offered.

## Preparing for stabilization

There are no strict guidelines for stabilization, but generally, APIs should "cook" for some time in an unstable version and see some use by the community. Since many community members test changes on the nightly compiler channel, this will help justify APIs as useful and discover their shortcomings. This "cook" time generally resets whenever an API is changed, except in trivial cases like renaming where the extra time isn't useful.

Depending on the size of a particular API, some changes may be stabilized immediately without even creating a tracking issue. Historically, all trait implementations where stabilized due to a compiler limitation; even though this limitation has been lifted, many trait implementations still go immediately through the [FCP process] instead of creating a tracking issue. Similarly, changes to documentation which offer new guarantees about APIs are difficult to implement unstably and instead go through FCP immediately.

[FCP process]: ../membership.md#fcp-process

For large APIs, a proposal for stabilization should come with an associated stabilization report that summarizes the implementation history, API, and community desire for a feature in a more digestible format than the tracking issue summary. An example of a simple stabilization report can be found in [#88581], and a complicated example can be found in [#156882].

[#88581]: https://github.com/rust-lang/rust/issues/88581#issuecomment-1054642118
[#156882]: https://github.com/rust-lang/rust/pull/156882#issue-4512312004

## Scoping stabilization

Before stabilization, a decision should be made about whether the entire API is being stabilized, or only part of it. If only part of an API is being stabilized, the to-be-stable and to-remain-unstable APIs should be split into separate tracking issues, usually with a PR modifying the `#[unstable]` attributes. This can also be done as part of a stabilization PR, although this is not recommended.

Before proposing stabilization, it should be decided whether only the library team needs to FCP the change, or if other teams should participate in the FCP. Depending on the change, different teams may need to be involved:

* If the change requires a new compiler intrinsic or language feature, it needs `T-lang` approval.
* If the change involves new aspects of the trait solver or type system, it needs `T-types` approval.
* If the change affects the behavior of unsafe code or the language itself, it needs `T-opsem` approval.

If the involvement of a team in a proposal is ever unclear, you should seek additional guidance from that team or a libs team lead before stabilization. Specific rules for including teams may be included on the [changing APIs page](./changing.md).

## FCP proposal

If the [FCP team] (and other relevant teams) seem likely to accept a stabilization proposal, anyone (including non-team-members) can open a PR to stabilize the feature, which usually involves converting `#[unstable]` attributes into `#[stable]` ones. A stabilization PR will also need to remove `#![feature(...)]` attributes from standard library crates, compiler crates, and documentation tests.

Once a stabilization PR is opened, any member of the libs team can propose FCP to merge the feature. FCPs can be proposed by `@rfcbot fcp merge libs` for libs-only FCPs, or `libs` can be replaced with a comma-separated list of all necessary teams.

While FCPs can be proposed on tracking issues and were historically done there, all new FCPs should be proposed in stabilization PRs instead. This helps ensure that the documentation for stabilized APIs is accurate, since team members can check the code to verify the API changes.

Sometimes, an FCP for the standard library may be blocked by relevant documentation in unexpected places, and the author of the stabilization PR should be prepared to make these changes if necessary. This can include, but is not limited to:

* [The reference], if the stabilized API has an associated language feature or behavior.
* [The book], if the stabilized API adds new functionality for an edition or fundamental language feature.
* [Rust By Example], if relevant.

[The reference]: https://github.com/rust-lang/reference
[The book]: https://github.com/rust-lang/book
[Rust By Example]: https://github.com/rust-lang/rust-by-example

Additionally, the author of the PR should be expected to make any changes requested by the FCP team as needed, which are usually naming or other minor changes. Sometimes, an FCP proposal may also be premature and the PR may be closed instead, at which point future changes should point back to the tracking issue. People wishing to propose stabilization without the commitment of maintaining the stabilization PR can discuss the proposal on the tracking issue or in the [`#t-libs`] Zulip stream.

[`#t-libs`]: https://rust-lang.zulipchat.com/#narrow/channel/219381-t-libs
