# Governance

<p style="background: #f8d7da; color: #721c24; padding: 5px">
<strong>IMPORTANT</strong> This document is adapted from
<a href="https://rust-lang.github.io/rfcs/1068-rust-governance.html">RFC 1068</a>
and is currently being actively worked on, however there may be large parts of
Rust's governance that are missing, incomplete, or out of date.
</p>

## Core team

**The core team serves as leadership for the Rust project as a whole**. In
particular, it:

- **Sets the overall direction and vision for the project.** That means setting
  the core values that are used when making decisions about technical
  tradeoffs. It means steering the project toward specific use cases where Rust
  can have a major impact. It means leading the discussion, and writing RFCs
  for, _major_ initiatives in the project.

- **Sets the priorities and release schedule.** Design bandwidth is limited, and
  it's dangerous to try to grow the language too quickly; the core team makes
  some difficult decisions about which areas to prioritize for new design, based
  on the core values and target use cases.

- **Focuses on broad, cross-cutting concerns.** The core team is specifically
  designed to take a _global_ view of the project, to make sure the pieces are
  fitting together in a coherent way.

- **Spins up or shuts down subteams.** Over time, we may want to expand the set
  of subteams, and it may make sense to have temporary "strike teams" that focus
  on a particular, limited task.

- **Decides whether/when to ungate a feature.** While the subteams make
  decisions on RFCs, the core team is responsible for pulling the trigger that
  moves a feature from nightly to stable. This provides an extra check that
  features have adequately addressed cross-cutting concerns, that the
  implementation quality is high enough, and that language/library commitments
  are reasonable.

The core team should include both the subteam leaders, and, over time, a diverse
set of other stakeholders that are both actively involved in the Rust community,
and can speak to the needs of major Rust constituencies, to ensure that the
project is addressing real-world needs.

## Subteams

**The primary roles of each subteam are**:

- Shepherding RFCs for the subteam area. As always, that means (1) ensuring that
  stakeholders are aware of the RFC, (2) working to tease out various design
  tradeoffs and alternatives, and (3) helping build consensus.

- Accepting or rejecting RFCs in the subteam area.

- Setting policy on what changes in the subteam area require RFCs, and reviewing
  direct PRs for changes that do not require an RFC.

- Delegating _reviewer rights_ for the subteam area. The ability to `r+` is not
  limited to team members, and in fact earning `r+` rights is a good stepping
  stone toward team membership. Each team should set reviewing policy, manage
  reviewing rights, and ensure that reviews take place in a timely manner.
  (Thanks to Nick Cameron for this suggestion.)

Subteams make it possible to involve a larger, more diverse group in the
decision-making process. In particular, **they should involve a mix of**:

- Rust project leadership, in the form of at least one core team member (the
  leader of the subteam).

- Area experts: people who have a lot of interest and expertise in the subteam
  area, but who may be far less engaged with other areas of the project.

- Stakeholders: people who are strongly affected by decisions in the
  subteam area, but who may not be experts in the design or
  implementation of that area. _It is crucial that some people heavily
  using Rust for applications/libraries have a seat at the table, to
  make sure we are actually addressing real-world needs._

Members should have demonstrated a good sense for design and dealing with
tradeoffs, an ability to work within a framework of consensus, and of course
sufficient knowledge about or experience with the subteam area. Leaders should
in addition have demonstrated exceptional communication, design, and people
skills. They must be able to work with a diverse group of people and help lead
it toward consensus and execution.

Each subteam is led by a member of the core team. **The leader is responsible for**:

- Setting up the subteam:

  - Deciding on the initial membership of the subteam (in consultation with
    the core team). Once the subteam is up and running.

  - Working with subteam members to determine and publish subteam policies and
    mechanics, including the way that subteam members join or leave the team
    (which should be based on subteam consensus).

- Communicating core team vision downward to the subteam.

- Alerting the core team to subteam RFCs that need global, cross-cutting
  attention, and to RFCs that have entered the "final comment period" (see below).

- Ensuring that RFCs and PRs are progressing at a reasonable rate, re-assigning
  shepherds/reviewers as needed.

- Making final decisions in cases of contentious RFCs that are unable to reach
  consensus otherwise (should be rare).

The way that subteams communicate internally and externally is left to each
subteam to decide, but:

- Technical discussion should take place as much as possible on public forums,
  ideally on RFC/PR threads and tagged discuss posts.

- Each subteam will have a dedicated
  [internals forum](https://internals.rust-lang.org/) tag.

- Subteams should actively seek out discussion and input from stakeholders who
  are not members of the team.

- Subteams should have some kind of regular meeting or other way of making
  decisions. The content of this meeting should be summarized with the rationale
  for each decision -- and, as explained below, decisions should generally be
  about weighting a set of already-known tradeoffs, not discussing or
  discovering new rationale.

- Subteams should regularly publish the status of RFCs, PRs, and other news
  related to their area. Ideally, this would be done in part via a dashboard
  like [the Homu queue](http://buildbot.rust-lang.org/homu/queue/rust).

## Decision-making

### Consensus

Rust has long used a form of [consensus decision-making][consensus]. In a
nutshell the premise is that a successful outcome is not where one side of a
debate has "won", but rather where concerns from _all_ sides have been addressed
in some way. **This emphatically does not entail design by committee, nor
compromised design**. Rather, it's a recognition that

> ... every design or implementation choice carries a trade-off and numerous
> costs. There is seldom a right answer.

Breakthrough designs sometimes end up changing the playing field by eliminating
tradeoffs altogether, but more often difficult decisions have to be made. **The
key is to have a clear vision and set of values and priorities**, which is the
core team's responsibility to set and communicate, and the subteam's
responsibility to act upon.

Whenever possible, we seek to reach consensus through discussion and design
revision. Concretely, the steps are:

- Initial RFC proposed, with initial analysis of tradeoffs.
- Comments reveal additional drawbacks, problems, or tradeoffs.
- RFC revised to address comments, often by improving the design.
- Repeat above until "major objections" are fully addressed, or it's clear that
  there is a fundamental choice to be made.

Consensus is reached when most people are left with only "minor" objections,
i.e., while they might choose the tradeoffs slightly differently they do not
feel a strong need to _actively block_ the RFC from progressing.

One important question is: consensus among which people, exactly? Of course, the
broader the consensus, the better. But at the very least, **consensus within the
members of the subteam should be the norm for most decisions.** If the core team
has done its job of communicating the values and priorities, it should be
possible to fit the debate about the RFC into that framework and reach a fairly
clear outcome.

[consensus]: https://en.wikipedia.org/wiki/Consensus_decision-making

### Lack of consensus

In some cases, though, consensus cannot be reached. These cases tend to split
into two very different camps:

- "Trivial" reasons, e.g., there is not widespread agreement about naming, but
  there is consensus about the substance.

- "Deep" reasons, e.g., the design fundamentally improves one set of concerns at
  the expense of another, and people on both sides feel strongly about it.

In either case, an alternative form of decision-making is needed.

- For the "trivial" case, usually either the RFC shepherd or subteam leader will
  make an executive decision.

- For the "deep" case, the subteam leader is empowered to make a final decision,
  but should consult with the rest of the core team before doing so.

### How and when RFC decisions are made, and the "final comment period"

Each RFC has a shepherd drawn from the relevant subteam. The shepherd is
responsible for driving the consensus process -- working with both the RFC
author and the broader community to dig out problems, alternatives, and improved
design, always working to reach broader consensus.

At some point, the RFC comments will reach a kind of "steady state", where no
new tradeoffs are being discovered, and either objections have been addressed,
or it's clear that the design has fundamental downsides that need to be weighed.

At that point, the shepherd will announce that the RFC is in a "final comment
period" (which lasts for one week). This is a kind of "last call" for strong
objections to the RFC. **The announcement of the final comment period for an RFC
should be very visible**; it should be included in the subteam's periodic
communications.

> Note that the final comment period is in part intended to help keep RFCs
> moving. Historically, RFCs sometimes stall out at a point where discussion has
> died down but a decision isn't needed urgently. In this proposed model, the
> RFC author could ask the shepherd to move to the final comment period (and
> hence toward a decision).

After the final comment period, the subteam can make a decision on the RFC. The
role of the subteam at that point is _not_ to reveal any new technical issues or
arguments; if these come up during discussion, they should be added as comments
to the RFC, and it should undergo another final comment period.

Instead, the subteam decision is based on **weighing the already-revealed
tradeoffs against the project's priorities and values** (which the core team is
responsible for setting, globally). In the end, these decisions are about how to
weight tradeoffs. The decision should be communicated in these terms, pointing
out the tradeoffs that were raised and explaining how they were weighted, and
**never introducing new arguments**.

## Keeping things lightweight

In addition to the "final comment period" proposed above, this RFC proposes some
further adjustments to the RFC process to keep it lightweight.

A key observation is that, thanks to the stability system and nightly/stable
distinction, **it's easy to experiment with features without commitment**.

### Clarifying what needs an RFC

Over time, we've been drifting toward requiring an RFC for essentially any
user-facing change, which sometimes means that very minor changes get stuck
awaiting an RFC decision. While subteams + final comment period should help keep
the pipeline flowing a bit better, it would also be good to allow "minor"
changes to go through without an RFC, provided there is sufficient review in
some other way. (And in the end, the core team ungates features, which ensures
at least a final review.)

This RFC does not attempt to answer the question "What needs an RFC", because
that question will vary for each subteam. However, this RFC stipulates that each
subteam should set an explicit policy about:

1. What requires an RFC for the subteam's area, and
2. What the non-RFC review process is.

These guidelines should try to keep the process lightweight for minor changes.

### Clarifying the "finality" of RFCs

While RFCs are very important, they do not represent the final state of a
design. Often new issues or improvements arise during implementation, or after
gaining some experience with a feature. **The nightly/stable distinction exists
in part to allow for such design iteration.**

Thus RFCs do not need to be "perfect" before acceptance. If consensus is reached
on major points, the minor details can be left to implementation and revision.

Later, if an implementation differs from the RFC in _substantial_ ways, the
subteam should be alerted, and may ask for an explicit amendment RFC. Otherwise,
the changes should just be explained in the commit/PR.

## The teams

With all of that out of the way, what subteams should we start with? This RFC
proposes the following initial set:

- Language design
- Libraries
- Compiler
- Tooling and infrastructure
- Moderation

In the long run, we will likely also want teams for documentation and for
community events, but these can be spun up once there is a more clear need (and
available resources).

### Language design team

Focuses on the _design_ of language-level features; not all team members need to
have extensive implementation experience.

Some example RFCs that fall into this area:

- [Associated types and multidispatch](https://github.com/rust-lang/rfcs/pull/195)
- [DST coercions](https://github.com/rust-lang/rfcs/pull/982)
- [Trait-based exception handling](https://github.com/rust-lang/rfcs/pull/243)
- [Rebalancing coherence](https://github.com/rust-lang/rfcs/pull/1023)
- [Integer overflow](https://github.com/rust-lang/rfcs/pull/560) (this has high
  overlap with the library subteam)
- [Sound generic drop](https://github.com/rust-lang/rfcs/pull/769)

### Library team

Oversees both `std` and, ultimately, other crates in the `rust-lang` github
organization. The focus up to this point has been the standard library, but we
will want "official" libraries that aren't quite `std` territory but are still
vital for Rust. (The precise plan here, as well as the long-term plan for `std`,
is one of the first important areas of debate for the subteam.) Also includes
API conventions.

Some example RFCs that fall into this area:

- [Collections reform](https://github.com/rust-lang/rfcs/pull/235)
- [IO reform](https://github.com/rust-lang/rfcs/pull/517/)
- [Debug improvements](https://github.com/rust-lang/rfcs/pull/640)
- [Simplifying std::hash](https://github.com/rust-lang/rfcs/pull/823)
- [Conventions for ownership variants](https://github.com/rust-lang/rfcs/pull/199)

### Compiler team

Focuses on compiler internals, including implementation of language
features. This broad category includes work in codegen, factoring of compiler
data structures, type inference, borrowck, and so on.

There is a more limited set of example RFCs for this subteam, in part because we
haven't generally required RFCs for this kind of internals work, but here are two:

- [Non-zeroing dynamic drops](https://github.com/rust-lang/rfcs/pull/320) (this
  has high overlap with language design)
- [Incremental compilation](https://github.com/rust-lang/rfcs/pull/594)

### Tooling and infrastructure team

Even more broad is the "tooling" subteam, which at inception is planned to
encompass every "official" (rust-lang managed) non-`rustc` tool:

- rustdoc
- rustfmt
- Cargo
- crates.io
- CI infrastructure
- Debugging tools
- Profiling tools
- Editor/IDE integration
- Refactoring tools

It's not presently clear exactly what tools will end up under this umbrella, nor
which should be prioritized.

### Moderation team

Finally, the moderation team is responsible for dealing with CoC violations.

One key difference from the other subteams is that the moderation team does not
have a leader. Its members are chosen directly by the core team, and should be
community members who have demonstrated the highest standard of discourse and
maturity. To limit conflicts of interest, **the moderation subteam should not
include any core team members**. However, the subteam is free to consult with
the core team as it deems appropriate.

The moderation team will have a public email address that can be used to raise
complaints about CoC violations (forwards to all active moderators).

#### Initial plan for moderation

What follows is an initial proposal for the mechanics of moderation. The
moderation subteam may choose to revise this proposal by drafting an RFC, which
will be approved by the core team.

Moderation begins whenever a moderator becomes aware of a CoC problem, either
through a complaint or by observing it directly. In general, the enforcement
steps are as follows:

> **These steps are adapted from text written by Manish Goregaokar, who helped
> articulate them from experience as a Stack Exchange moderator.**

- Except for extreme cases (see below), try first to address the problem with a
  light public comment on thread, aimed to de-escalate the situation. These
  comments should strive for as much empathy as possible. Moderators should
  emphasize that dissenting opinions are valued, and strive to ensure that the
  technical points are heard even as they work to cool things down.

  When a discussion has just gotten a bit heated, the comment can just be a
  reminder to be respectful and that there is rarely a clear "right" answer. In
  cases that are more clearly over the line into personal attacks, it can
  directly call out a problematic comment.

- If the problem persists on thread, or if a particular person repeatedly comes
  close to or steps over the line of a CoC violation, moderators then email the
  offender privately. The message should include relevant portions of the CoC
  together with the offending comments. Again, the goal is to de-escalate, and
  the email should be written in a dispassionate and empathetic way. However,
  the message should also make clear that continued violations may result in a
  ban.

- If problems still persist, the moderators can ban the offender. Banning should
  occur for progressively longer periods, for example starting at 1 day, then 1
  week, then permanent. The moderation subteam will determine the precise
  guidelines here.

In general, moderators can and should unilaterally take the first step, but
steps beyond that (particularly banning) should be done via consensus with the
other moderators. Permanent bans require core team approval.

Some situations call for more immediate, drastic measures: deeply inappropriate
comments, harassment, or comments that make people feel unsafe. (See the
[code of conduct](http://www.rust-lang.org/conduct.html) for some more details
about this kind of comment). In these cases, an individual moderator is free to
take immediate, unilateral steps including redacting or removing comments, or
instituting a short-term ban until the subteam can convene to deal with the
situation.

The moderation team is responsible for interpreting the CoC. Drastic measures
like bans should only be used in cases of clear, repeated violations.

Moderators themselves are held to a very high standard of behavior, and should
strive for professional and impersonal interactions when dealing with a CoC
violation. They should always push to _de-escalate_. And they should recuse
themselves from moderation in threads where they are actively participating in
the technical debate or otherwise have a conflict of interest. Moderators who
fail to keep up this standard, or who abuse the moderation process, may be
removed by the core team.

Subteam, and especially core team members are _also_ held to a high standard of
behavior. Part of the reason to separate the moderation subteam is to ensure
that CoC violations by Rust's leadership be addressed through the same
independent body of moderators.

Moderation covers all rust-lang venues, which currently include github
repos, IRC channels (#rust, #rust-internals, #rustc, #rust-libs), and
the two discourse forums. (The subreddit already has its own
moderation structure, and isn't directly associated with the rust-lang
organization.)
