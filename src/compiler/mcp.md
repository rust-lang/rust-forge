# Major Change Proposals

Introduced in [RFC 2904], a "major change proposal" is a lightweight
form of RFC that the compiler team uses for architectural changes that
are not end-user facing. (It can also be used for small user-facing
changes like adding new compiler flags, though in that case we also
require an `rfcbot fcp` to get full approval from the team.) Larger
changes or modifications to the Rust language itself require a full
RFC (the latter fall under the lang team's purview).

[RFC 2904]: https://github.com/rust-lang/rfcs/blob/master/text/2904-compiler-major-change-process.md

## Motivation
[motivation]: #motivation

As the compiler grows in complexity, it becomes harder and harder to track what's going on. We don't currently have a clear channel for people to signal their intention to make "major changes" that may impact other developers in a lightweight way (and potentially receive feedback).

Our goal is to create a channel for signaling intentions that lies somewhere between opening a PR (and perhaps cc'ing others on that PR) and creating a compiler team design meeting proposal or RFC.

## Goals

Our goals with the MCP are as follows:

* Encourage people making a major change to write at least a few paragraphs about what they plan to do.
* Ensure that folks in the compiler team are aware the change is happening and given a chance to respond.
* Ensure that every proposal has a "second", meaning some expert from the team who thinks it's a good idea.
* Ensure that major changes have an assigned and willing reviewer.
* Avoid the phenomenon of large, sweeping PRs landing "out of nowhere" onto someone's review queue.
* Avoid the phenomenon of PRs living in limbo because it's not clear what level of approval is required for them to land.

## Major Change Proposals

If you would like to make a [major change] to the compiler, the process is as follows:

[major change]: #What-constitutes-a-major-change

* Open a tracking issue on the [rust-lang/compiler-team] repo using the [major change template].
    * A Zulip topic in the stream `#t-compiler/major changes` will automatically be created for you by a bot.
    * If concerns are raised, you may want to modify the proposal to address those concerns.
    * Alternatively, you can submit a [design meeting proposal] to have a longer, focused discussion.
* To be accepted, a major change proposal needs three things:
    * One or more **reviewers**, who commit to reviewing the work. This can be the person making the proposal, if they intend to mentor others.
    * A **second**, a member of the compiler team or a contributor who approves of the idea, but is not the one originating the proposal.
    * A **final comment period** (a 10 day wait to give people time to comment).
        * The FCP can be skipped if the change is easily reversed and/or further objections are considered unlikely. This often happens if there has been a lot of prior discussion, for example.
* Once the FCP completes, if there are no outstanding concerns, PRs can start to land.
    * If those PRs make outward-facing changes that affect stable
      code, then either the MCP or the PR(s) must be approved with a
      `rfcbot fcp merge` comment.

[major change template]: https://github.com/rust-lang/compiler-team/issues/new?assignees=&labels=major-change%2C+T-compiler&template=major_change.md&title=%28My+major+change+proposal%29

## Conditional acceptance

Some major change proposals will be conditionally accepted. This indicates that we'd like to see the work land, but we'd like to re-evaluate the decision of whether to commit to the design after we've had time to gain experience. We should try to be clear about the things we'd like to evaluate, and ideally a timeline.

## Deferred or not accepted

Some proposals will not be accepted. Some of the possible reasons:

* You may be asked to do some prototyping or experimentation before a final decision is reached
* The idea might be reasonable, but there may not be bandwidth to do the reviewing, or there may just be too many other things going on.
* The idea may be good, but it may be judged that the resulting code would be too complex to maintain, and not worth the benefits.
* There may be flaws in the idea or it may not sufficient benefit.

[rust-lang/compiler-team]: https://github.com/rust-lang/compiler-team
[design meeting proposal]: https://forge.rust-lang.org/compiler/steering-meeting.html

## What happens if someone opens a PR that seems like a major change *without* doing this process?

The PR should be closed or marked as blocked, with a request to create
a major change proposal first.

If the PR description already contains suitable text that could serve
as an MCP, then simply copy and paste that into an MCP issue. Using an
issue consistently helps to ensure that the tooling and process works
smoothly.

## Can I work on code experimentally before a MCP is accepted?

Of course!  You are free to work on PRs or write code. But those PRs should be marked as experimental and they should not land, nor should anyone be expected to review them (unless folks want to).

## What constitutes a major change?

The rough intuition is "something that would require updates to the [rustc-dev-guide] or the [rustc book]". In other words:

* Something that alters the architecture of some part(s) of the compiler, since this is what the rustc-dev-guide aims to document.
* A simple change that affects a lot of people, such as altering the names of very common types or changing coding conventions.
* Adding a compiler flag or other public facing changes, which should be documented (ultimately) in the rustc book. This is only appropriate for "minor" tweaks, however, and not major things that may impact a lot of users. (Also, public facing changes will require a full FCP before landing on stable, but an MCP can be a good way to propose the idea.)

Note that, in some cases, the change may be deemed **too big** and a full FCP or RFC may be required to move forward. This could occur with significant public facing change or with sufficiently large changes to the architecture. The compiler team leads can make this call.

Note that whether something is a major change proposal is not necessarily related to the number of lines of code that are affected. Renaming a method can affect a large number of lines, and even require edits to the rustc-dev-guide, but it may not be a major change. At the same time, changing names that are very broadly used could constitute a major change (for example, renaming from the `tcx` context in the compiler to something else would be a major change).

[rustc-dev-guide]: https://rustc-dev-guide.rust-lang.org
[rustc book]: https://doc.rust-lang.org/rustc/index.html

## Public-facing changes require rfcbot fcp

The MCP "seconding" process is only meant to be used to get agreement
on the technical architecture we plan to use. It is not sufficient to
stabilize new features or make public-facing changes like adding a -C
flag. For that, an `rfcbot fcp` is required (or perhaps an RFC, if the
change is large enough).

For landing compiler flags in particular, a good approach is to start
with an MCP introducing a `-Z` flag and then "stabilize" the flag by
moving it to `-C` in a PR later (which would require `rfcbot fcp`).

Major change proposals are not sufficient for language changes or
changes that affect cargo.

## Steps to open a MCP

* Open a tracking issue on the [rust-lang/compiler-team] repo using the
  [major change template].
* Create a Zulip topic in the stream `#t-compiler/major changes`:
  * The topic should be named something like "modify the whiz-bang
    component compiler-team#123", which describes the change and links
    to the tracking issue.
  * The stream will be used for people to ask questions or propose changes.

## What kinds of comments should go on the tracking issue in compiler-team repo?

Please direct technical conversation to the Zulip stream.

The compiler-team repo issues are intended to be low traffic and used for procedural purposes. Note that to "second" a design or offer to review, you should be someone who is familiar with the code, typically but not necessarily a compiler team member or contributor.

* Announcing that you "second" or approve of the design.
* Announcing that you would be able to review or mentor the work.
* Noting a concern that you don't want to be overlooked.
* Announcing that the proposal will be entering FCP or is accepted.

## How does one register as reviewer, register approval, or raise an objection?

These types of procedural comments can be left on the issue (it's also good to leave a message in Zulip). See the
previous section. To facilitate a machine parsable scanning of the concerns please use the following syntax to formally
register a concern:
```
@rfcbot concern reason-for-concern

<long description of the concern>
```

And the following syntax to lift a concern when resolved:
```
@rfcbot resolve reason-for-concern
```

## Who decides whether a concern is unresolved?

Usually the experts in the given area will reach a consensus here. But if there is some need for a "tie breaker" vote or judgment call, the compiler-team leads make the final call.

## What are some examples of major changes from the past?

Here are some examples of changes that were made in the past that would warrant the major change process:

* overhauling the way we encode crate metadata
* merging the gcx, tcx arenas
* renaming a widely used, core abstraction, such as the `Ty` type
* introducing cargo pipelining 
* adding a new `-C` flag that exposes some minor variant

## What are some examples of things that are too big for the major change process?

Here are some examples of changes that are too big for the major change process, or which at least would require auxiliary design meetings or a more fleshed out design before they can proceed:

* introducing incremental or the query system
* introducing MIR or some new IR
* introducing parallel execution
* adding ThinLTO support

## What are some examples of things that are too small for the major change process?

Here are some examples of things that don't merit any MCP:

* adding new information into metadata
* fixing an ICE or tweaking diagnostics
* renaming "less widely used" methods

## When should Major Change Proposals be closed?

Major Change Proposals can be closed:

* by the author, if they have lost interest in pursuing it.
* by a team lead or expert, if there are strong objections from key
  members of the team that don't look likely to be overcome.
* by folks doing triage, if there have been three months of
  inactivity. In this case, people should feel free to re-open the
  issue if they would like to "rejuvenate" it.
