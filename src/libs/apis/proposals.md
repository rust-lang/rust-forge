# API Change Proposals (ACPs)

API Change Proposals are the recommended process for proposing new APIs to the library team. By investing a small amount of work first to discuss a problem, the library team can work together with contributors to ensure an API's best chance of success. Although an accepted ACP does not guarantee that an API will be accepted, it represents a signal from the team that a change can be implemented and merged into the nightly compiler so people can try it out.

ACPs are submitted via an [issue template][ACP template] on the [`libs-team`] repository, and anyone can submit an ACP. You don't have to fully sketch out a feature with an ACP, but it should at least include enough information for the teak to work with.

ACPs are also technically optional, even though they are recommended in most cases. New APIs can always be proposed directly with a pull request to the [`rust-lang/rust`] repository, but in general, you should only do this for small, uncontroversial changes where the effort to create the implementation is less than the effort to make the ACP. If an implementation is proposed for an API we aren't confident in, we may ask you to write an ACP anyway, or reject the change entirely. Even though ACP-accepted changes aren't guaranteed to make it to [stabilization], at least the chance of wasted work is much lower, and at that point, the "wasted work" might translate into valuable insights anyway.

[ACP template]: https://github.com/rust-lang/libs-team/issues/new?template=api-change-proposal.md&title=ACP:+Insert+Title+Here
[`libs-team`]: https://github.com/rust-lang/libs-team
[`rust-lang/rust`]: https://github.com/rust-lang/rust
[stabilization]: ./stabilization.md

## Choosing an API

Since the standard library has strong stability guarantees, proposed APIs should ideally be as simple as possible and unlikely to change. Similarly, if there are a lot of valid options for an API, the standard library is probably not a good fit; people can create separate crates and choose which option they'd like instead. We tend to prefer simpler, concrete APIs over complex, abstract APIs since crates can add new abstractions, but we can't remove abstractions.

You can already see this principle in action in the standard library itself: [it actually had `Num` and `Int` traits that were removed before 1.0][RFC 369: Num Reform]. While these traits are unquestionably useful, they lead to a very large number of questions that don't have good answers: where should the boundaries between traits be drawn, what methods should be required and which should be optional, should people be allowed to implement the trait for their own types, etc. Instead of the standard library making these decisions, we just use the same method names across primitive types and let crates implement their own traits instead. Sure, it's annoying, but it's a whole lot more annoying to make a decision you can't walk back.

[RFC 369: Num Reform]: https://rust-lang.github.io/rfcs/0369-num-reform.html

In general, the very first question that you should ask of any API proposal is why it shouldn't exist as a third-party crate, and why it should have standard library support. This is an annoying bar to pass, since things are generally nicer in the standard library, but our stability guarantees make us reluctant to change things by default.

## Portable APIs

Since the standard library supports many different platforms, it also has to take care when adding platform-specific APIs. If we're not careful, we can offer APIs that are easy to use, but which aren't clearly distinguished as not working on certain platforms, which can cause code to break in interesting ways.

One way we've decided to solve this in the standard library is by using platform-specific extension traits that must be manually imported, rather than as inherent methods on types. For example, instead of adding an inherent method to get the POSIX permissions ("mode") of a file, you need to explicitly import [`std::os::unix::fs::PermissionsExt`] to be able to call the method.

[`std::os::unix::fs::PermissionsExt`]: https://doc.rust-lang.org/nightly/std/os/unix/fs/trait.PermissionsExt.html

Done properly, libraries can easily be audited for non-portable code by scanning for [`std::os`] imports instead of having to know all the specific cases and methods that aren't portable.

[`std::os`]: https://doc.rust-lang.org/nightly/std/os/index.html

## Table-filling APIs

In a lot of cases, requests for new APIs can feel a lot like filling out a table. [`PartialEq`] is a notable example, implementing various combinations of equality between `&[T]`, `[T; N]`, `Box<[T]>`, `Vec<T>`, `Rc<[T]>`, `Arc<[T]>`, `VecDeque<T>`, and more. In general, the library team is reluctant to participate in table-filling exercises simply for the sake of it.

[`PartialEq`]: https://doc.rust-lang.org/nightly/std/cmp/trait.PartialEq.html

Note that this means that even aiming for symmetry between binary operators is not strictly necessary within the standard library: for example, if we have `impl PartialEq<T> for U`, that does not mean we need `impl PartialEq<U> for T`. In a lot of cases, users will naturally orient their comparisons so that the "more-complicated" type is on the left-hand side, and so, we don't strictly need to allow a ["yoda condition"] style.

["yoda condition"]: https://en.wikipedia.org/wiki/Yoda_conditions

In general, if any API change devolves down to a table-filling exercise, the libs team may turn you back and ask you to rethink your approach.

## Accepting APIs

After an ACP is submitted to the `libs-team` repo, any libs team member can approve the ACP. By this same logic, *unstable* changes to the standard library can be accepted by any libs team member without an ACP at their discretion, regardless of whether they're on the [review rotation](../membership.md#review-rotation). Nontrivial and/or potentially controversial API changes should go through the ACP process.

In general, if an API is significant enough to deserve an ACP, there should be at least ten (10) days for it to gather feedback before being accepted. ACPs are approved by adding the `ACP-accepted` label, although they should not be closed until a corresponding [tracking issue](./stabilization.md#tracking-issues) is opened in [`rust-lang/rust`] (or, rarely, another repo like [`rust-lang/stdarch`]).

[`rust-lang/stdarch`]: https://github.com/rust-lang/stdarch

Similarly, in order to merge a change to an unstable API without an ACP, it should have a tracking issue opened to track the unstable feature. After being implemented, the tracking issue should be represented in the `#[unstable]` attribute for the feature, alongside any other relevant stability attributes.

At any time, an author can choose to voluntarily withdraw their ACP by closing the issue. People are encouraged to file new ACPs if no open ACP exists for a proposal they'd like to make, although searching through the `libs-team` repository is recommended to avoid duplicating open ACPs.

## Controversial APIs

Any libs team member may object to an ACP, blocking its approval. Note that objections should only be for larger aspects like a proposal's structure, since smaller concerns like naming can be resolved before the final stabilization.

If any libs team member has a concern about a potential API change, including both PRs and ACPs, they can nominate it for discussion at a [libs team meeting](../meetings.md) by adding the `I-libs-nominated` label. In general, the participants at the meeting will decide upon the course of action at the next meeting, which could mean rejecting the API, gathering more feedback, or offering changes.

If there is difficulty in resolving concerns for an ACP, [the FCP team](../membership.md#fcp-membership) can override them.

Besides approval, the only valid reasons for closing an ACP are:

* The proposal was withdrawn by the author
* The proposal was accepted elsewhere, or made impossible due to another change
* An *effectively identical* proposal exists; competing proposals can coexist, but copies of the same proposal should join forces

While the libs FCP team can ultimately decide the process for closing ACPs they think will never be accepted, in general, the form of that process will depend on the specific rules of a specific FCP team and shouldn't be relied upon. An FCP team closing an ACP does not necessarily mean that an ACP will never be possible, just that it's unlikely to be accepted by that particular FCP team.
