# Proposals, Approvals and Stabilization
It is very common to need to gather feedback and approval when contributing to the compiler, either
for permission to proceed with an experiment or refactoring, or when stabilizing a feature. This
document aims to summarise the various processes that the compiler team has for making approval
decisions and when each should be used.

## Approvals
There are three mechanisms that the team can use to approve a proposal (not all approval mechanisms
are suitable for each method of making a proposal - see below):

- r+
  - A proposal is r+'d when it is approved to be merged.
  - r+ can only be used to approve a PR.
- Seconding
  - A proposal is second'ed when a team member formally endorses the proposal. It is intended that
    seconding only occur once discussion has concluded and team members have had time to raise
    concerns. Seconding tentatively accepts a proposal subject to a ten-day waiting period for
    other team members to raise any concerns.
  - Seconding can only be used to approve a MCP.
- FCP
  - A Final Comment Period is started by a T-compiler member. it's a tool to get concrete consensus
    from the team. This requires sign-off from the compiler FCP reviewers (the [`compiler-fcp`
    subteam][compiler-fcp]) to approve a proposal and then a ten day waiting period.
  - FCPs can be used to approve any form of proposal.

## Proposals
There are three ways to propose a change to the compiler team. The appropriate choice depends on
the nature of the proposal, described below.

- Request For Comments (RFC)
  - RFCs are pull requests to the [`rust-lang/rfcs`][rfcs] repository and are a heavy-weight
    proposal mechanism, reserved for significant changes.
  - RFC proposals can only be approved by *FCPs*.
- Major Change Proposal (MCP)
  - MCPs are issues in the [`rust-lang/compiler-team`][mcps] repository and are a medium-weight
    proposal mechanism, suitable for most proposals. MCPs are recommended for written proposals
    that are not end-user facing.
  - Introduced in [RFC 2904][rfc_2904].
  - MCP proposals can be approved by *FCPs* or *Seconding*.
- Pull Request (PR)
  - PRs are pull requests to the [`rust-lang/rust`][rust] repository and are a light-weight
    proposal mechanism, suitable for most proposals. PRs are preferred when the proposal is
    accompanied by a small patchset (such as stabilization of a compiler flag or addition of
    a new target).
  - PR proposals can be approved by *FCPs* or *r+*.

[rfc_2904]: https://github.com/rust-lang/rfcs/blob/master/text/2904-compiler-major-change-process.md

### How do I submit an MCP?

* Open a tracking issue on the [rust-lang/compiler-team] repo using the [major change template].
    * A Zulip topic in the stream `#t-compiler/major changes` will automatically be created for you
      by a bot.
    * If concerns are raised, you may want to modify the proposal to address those concerns.
    * Alternatively, you can submit a [design meeting proposal] to have a longer, focused discussion.
* To be accepted, a major change proposal needs three things:
    * A **second**, a member of the compiler team who approves of the idea, but is not the one
      originating the proposal.
    * A **final comment period** (a 10 day wait to give people time to comment).
        * The FCP can be skipped if the change is easily reversed and/or further objections are
          considered unlikely. This often happens if there has been a lot of prior discussion, for
          example.
        * Outstanding concerns block the final comment period from completing.
        * When all outstanding concerns are resolved, the final comment period countdown is
          restarted.
* Once the FCP completes, if there are no outstanding concerns, contributions can begin.
    * An earlier accepted MCP is not a substitute for any later necessary approvals.

[rust-lang/compiler-team]: https://github.com/rust-lang/compiler-team
[design meeting proposal]: ./meetings.md#steeringplanning-meeting
[major change template]: https://github.com/rust-lang/compiler-team/issues/new?template=major_change.md

#### What kinds of comments should go on a MCP in the compiler-team repo?
Please direct technical conversation to the Zulip stream.

The compiler-team repo issues are intended to be low traffic and used for procedural purposes.

It is recommended that any team member who wishes to "second" a proposal be familiar with the
relevant code. Anyone can note concerns that shouldn't be overlooked.

#### How does one second an MCP or raise an objection?
These types of procedural comments can be left on the issue (it's also good to leave a message in
Zulip). See the previous section. To facilitate a machine parsable scanning of the concerns
please use the following syntax to formally register a concern:

```text
@rustbot concern reason-for-concern

<long description of the concern>
```

And the following syntax to lift a concern when resolved:

```text
@rustbot resolve reason-for-concern
```

MCPs can be seconded using:

```text
@rustbot second
```

##### Who decides whether a concern is unresolved?
Usually the experts in the given area will reach a consensus here, but if there is some
need for a "tie breaker" vote or judgment call, the compiler team leads make the final call.

#### When should MCPs be closed?
MCPs can be closed:

* by the author, if they have lost interest in pursuing it.
* by a team lead or expert, if there are strong objections from key members of the team that
  don't look likely to be overcome.
* by folks doing triage, if there have been three months of inactivity. In this case, people
  should feel free to re-open the issue if they would like to "rejuvenate" it.

### What happens if someone makes a contribution that requires an approval and doesn't have one?
If the approval required for the contribution requires an MCP or an RFC, then the contribution
should be closed or marked as blocked, with a request to create an MCP or RFC first. If approval of
a PR is acceptable for the specific contribution (see below), then the approval process can begin.

### Can I work on code experimentally before a approval is gained?
Of course! You are free to work on PRs or write code. But those PRs should be marked as
experimental and they should not land, nor should anyone be expected to review them (unless
folks want to).

## What makes a good proposal?
A good proposal will address the following:

* **Motivation:** Why is this proposal necessary? What problem does it solve? Why is that problem
  important?
* **Design:** What are you proposing?
* **Implementation notes:** You don't have to talk about the implementation normally, but if there
  are any key things to note (i.e., it was very invasive to implement), you might note them here.
* **Precedent, links, and related material:** Have there been similar proposals on other
  compilers/linkers/tools, like `clang` or `lld`?
* **Alternatives, concerns, and key decisions:** Were there any alernatives considered? If so, why
  did you pick this design?

## What proposal/approval do I need?
This section aims to exhaustively detail which proposal and approval is necessary for any given
circumstance.

### Internal

- Creating [a notification group](./notification-groups.md)
  - **Propose using:** PR
  - **Approve using:** r+
  - If a team member finds the new group reasonable then they can merge the change adding the group.
- Significant internal refactorings/changes
  - **Propose using:** MCP
  - **Approve using:** Seconding
  - Describe your proposed refactorings in detail in an MCP - optionally scheduling a steering
    meeting if more focused discussion is necessary. Once discussion has concluded, a team member
    may second the proposal
- Defining/changing small team policies
  - **Propose using:** MCP
  - **Approve using:** Seconding
  - Examples of smaller policy changes where an MCP would be sufficient include our level of
    support for case-insensitive filesystems or whether the team intend tracking issues to host
    discussion
- Defining/changing large team policies
  - **Propose using:** RFC
  - **Approve using:** FCP
  - Larger policy changes requiring an FCP include proposals to the team's structure and
    membership criteria, etc

### Compiler flags

- Adding a compiler option for internal-use only (e.g. `-Ztreat-bug-as-err`)
  - **Propose using:** PR
  - **Approve using:** r+
  - If a team member finds the new option reasonable then they can merge the change adding the
    option
- Adding a simple compiler option with intent to later stabilize
  - **Propose using:** MCP
  - **Approve using:** Seconding
  - Simple options, such as exposing an uncontroversial option from LLVM, can be implemented and
    merged with a seconded MCP and r+ approval from a reviewer. It will need a full FCP when it is
    later stabilized
- Adding a complex compiler option with intent to later stabilize
  - **Propose using:** RFC
  - **Approve using:** FCP
  - If the option is complicated and requires design considerations, then write and submit
    a `t-compiler` RFC
- Removing internal-use only flags
  - **Propose using:** MCP
  - **Approve using:** Seconding
  - Describe the rationale for removing the unstable implementation. Once discussion has concluded,
    a team member may second the proposal
- Removing flags which were intended for eventual stabilization
  - **Propose using:** MCP
  - **Approve using:** Seconding
  - Describe the rationale for removing the unstable implementation. Once discussion has concluded,
    a team member may second the proposal
- Stabilizing a compiler option
  - **Propose using:** PR
  - **Approve using:** FCP
  - Open a PR and follow the [stabilization guide][stabilization_guide]. The assigned reviewer will
    check that the stabilization guide has been followed, review the code and start an FCP
- Reverting stabilization of a compiler option
  - **Propose using:** PR
  - **Approve using:** FCP
  - Open a PR and follow the [stabilization guide][stabilization_guide]. The assigned reviewer will
    check that the stabilization guide has been followed, review the code and start an FCP
- Extending the behavior of a stable flag
  - **Propose using:** MCP
  - **Approve using:** Seconding
  - Describe the rationale for extending the behavior of the flag. Once discussion has concluded,
    a team member may second the proposal

### Attributes

- Adding a attribute for internal-use only (e.g. `rustc_attrs`)
  - **Propose using:** PR
  - **Approve using:** r+
  - If a team member finds the new attribute reasonable then they can merge the change adding the
    attribute
- Adding a attribute with intent to later stabilize
  - Follow the language team's process and have the implementation PR reviewed by a member of the
    compiler team
- Removing internal-use only attributes
  - **Propose using:** MCP
  - **Approve using:** Seconding
  - Describe the rationale for removing the unstable implementation. Once discussion has concluded,
    a team member may second the proposal
- Removing attribute which were intended for eventual stabilization
  - Follow the language team's process and have the removal PR reviewed by a member of the compiler
    team
- Stabilizing an attribute
  - Follow the language team's process and have the stabiization PR reviewed by a member of the
    compiler team
- Reverting stabilization of an attribute
  - Follow the language team's process and have the revert PR reviewed by a member of the
    compiler team

### Features

- Adding experimental implementations of not-yet-proposed language features
  - **Propose using:** MCP
  - **Approve using:** Seconding
  - With the approval of the language team (that they think the feature is worth experimentation),
    then submit an RFC and if, after discussion has concluded, a compiler team member agrees that
    the implementation is feasible and will not put undue burden on the maintainers of the compiler,
    then they can second the MCP and implementation can proceed.
  - This isn't necessary if the owner of the implementation is a member of the compiler team
- Stabilizing a language feature
  - Follow the language team's process and have the stabiization PR reviewed by a member of the
    compiler team
- Reverting stabilization of a language feature
  - Follow the language team's process and have the revert PR reviewed by a member of the
    compiler team

### Targets

See the [Target Tier Policy][tier_policy] for detailed requirements for different Target Tiers.

#### Promoting targets

Quick overview for target promotions:

| Current target tier          | Goal target tier | Proposal | Required approvals  |
|------------------------------|------------------|----------|---------------------|
| N/A (proposing a new target) | 3                | PR       | r+ (compiler leads) |
| 3                            | 2                | MCP      | FCP                 |
| 2                            | 1                | RFC      | FCP                 |

- Proposing a new target
  - **Propose using:** PR
  - **Approve using:** r+ (compiler leads)
  - You can `r? compiler_leads` on the PR to roll one of the compiler leads as the reviewer.
  - Open a PR with the new target (w/ relevant documentation updates) and document adherence to the
    [target tier policy][tier_policy] in the description.
  - **New targets must start as Tier 3**.
  - New targets should be assigned to the compiler team co-leads to check for any licensing concerns
    and to ensure that any demands on the project infrastructure are considered and checked with
    relevant teams.
- Promoting a target from Tier 3 to Tier 2
  - **Propose using:** MCP
  - **Approve using:** FCP
  - Open a MCP with the target and document adherence to the [Tier 2 target policy][tier_policy] in
    the description.
- Promoting a target from Tier 2 to Tier 1
  - **Propose using:** RFC
  - **Approve using:** FCP
  - Open a RFC with the target and document adherence to the [Tier 1 target policy][tier_policy] in
    the RFC text.

#### Demoting or removing targets

Quick overview for target demotions and removals:

| Current target tier | Goal target tier               | Proposal | Required approvals  |
|---------------------|--------------------------------|----------|---------------------|
| 1                   | 2 (or removal)                 | RFC      | FCP                 |
| 2                   | 3 (or removal)                 | MCP      | FCP                 |
| 3                   | N/A (removing a tier 3 target) | PR       | r+ (compiler leads) |

- Demoting a target from Tier 1 to Tier 2, or removing a Tier 1 target
  - **Propose using:** RFC
  - **Approve using:** FCP
  - Open a RFC with the target and rationale.
  - Example: [RFC: Demote i686-pc-windows-gnu to Tier 2
    #3771](https://github.com/rust-lang/rfcs/pull/3771)
- Demoting a target from Tier 2 to Tier 3, or removing a Tier 2 target
  - **Propose using:** MCP
  - **Approve using:** FCP
  - Open a MCP with the target and rationale.
- Removing a Tier 3 target
  - **Propose using:** PR
  - **Approve using:** r+ (compiler leads)
  - Open a PR with the target and rationale for removing the Tier 3 target.

#### Other kind of target changes

- Renaming a target or making a breaking change to a tier 3 target
  - **Propose using:** PR
  - **Approve using:** r+
  - Open an PR with the proposed rename and describe the motivation for the change and obtain a r+
    from the reviewer.
- Renaming a target or making a breaking change to a tier 2 target
  - **Propose using:** MCP
  - **Approve using:** FCP
  - Open an MCP describing the motivation for the change and start an FCP to approve, start an FCP.
  - If approved, the change should be accompanied by a blog post announcing the change with a
    notice period of at least one release before the change applies.
- Renaming a target or making a breaking change to a tier 1 target
  - **Propose using:** RFC
  - **Approve using:** FCP
  - Open an RFC describing the motivation for the change and start an FCP to approve, start an FCP.
  - If approved, the change should be accompanied by a blog post announcing the change with a
    notice period of at least one release before the change applies.
- Changing target baseline (e.g. minimum Darwin or Windows version bump)
  - **Propose using:** MCP
  - **Approve using:** FCP
  - Write an MCP describing why the target should have a change of baseline and once discussion has
    concluded, an FCP can be started to approve the change of baseline.
- Adding/removing target maintainers
  - **Propose using:** PR
  - **Approve using:** r+
  - Open a PR with the changes to the target documentation and obtain an r+ from the reviewer.
- Adding a target feature
  - **Propose using:** PR
  - **Approve using:** r+
  - Open a PR adding the target feature and obtain an r+ from the reviewer.
- Stabilizing a target feature
  - **Propose using:** PR
  - **Approve using:** FCP
  - Open a PR stabilizing the target feature and once the reviewer is happy with the changes,
    an FCP can be started

### Lints, errors and warnings

- Adding a new warning/error
  - **Propose using:** PR
  - **Approve using:** r+
  - Open a PR with the implementation and obtain an r+ from the reviewer
- Adding a new lint group
  - Follow the language team's process and have the implementation PR reviewed by a member of the
    compiler team
- Adding a new lint related to compiler features
  - **Propose using:** MCP
  - **Approve using:** FCP
  - A lint concerning a detail that is otherwise the responsibility of the compiler team (such as
    compiler flags) is the responsibility of the compiler to approve, rather than the language team.
  - Write an MCP describing the lint and its justification and once discussion has concluded, an
    FCP can be started to approve the new lint
- Adding a new future compatibility warning (FCW) related to compiler features
  - **Propose using:** MCP
  - **Approve using:** FCP
  - A FCW concerning a detail that is otherwise the responsibility of the compiler team (such as
    compiler flags) is the responsibility of the compiler to approve, rather than the language team.
  - Write an MCP describing the FCW and its justification and once discussion has concluded, an
    FCP can be started to approve the new FCW
- Changing default lint level of a lint related to compiler features
  - **Propose using:** MCP
  - **Approve using:** FCP
  - A lint concerning a detail that is otherwise the responsibility of the compiler team (such as
    compiler flags) is the responsibility of the compiler to approve, rather than the language team.
  - Write an MCP describing the rationale for changing the default lint level and once discussion
    has concluded, an FCP can be started to approve the new lint
- Adding a new lint related to language features
  - Follow the language team's process and have the implementation PR reviewed by a member of the
    compiler team
- Adding a new future compatibility warning (FCW) related to language features
  - Follow the language team's process and have the implementation PR reviewed by a member of the
    compiler team
- Changing default lint level of a lint related to language features
  - Follow the language team's process and have the implementation PR reviewed by a member of the
    compiler team

### Licensing

- Introducing a new dependency/license change/dependency bump
  - **Propose using:** PR
  - **Approve using:** r+ (compiler leads)
  - Open a PR with the change affecting licensing and assign it to the team leads for review

### Stable/beta channel backport nominations of compiler changes

See [*Backports*](./backports.md).

### Adding ecosystem/integration test jobs/components to rust-lang/rust CI

See [*Adding ecosystem/integration test jobs/components to rust-lang/rust CI*](./proposals-and-stabilization/ecosystem-integration-tests.md).


[stabilization_guide]: https://rustc-dev-guide.rust-lang.org/stabilization_guide.html
[tier_policy]: https://doc.rust-lang.org/rustc/target-tier-policy.html
[mcps]: https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change
[rfcs]: https://github.com/rust-lang/rfcs
[rust]: https://github.com/rust-lang/rust
[compiler_lint_eg]: https://doc.rust-lang.org/rustc/lints/listing/deny-by-default.html#explicit-builtin-cfgs-in-flags
[ecosystem_testing]: https://rustc-dev-guide.rust-lang.org/tests/ecosystem.html
[compiler-fcp]: https://github.com/rust-lang/team/blob/master/teams/compiler-fcp.toml
[tier-1-target-policy]: https://doc.rust-lang.org/rustc/target-tier-policy.html#tier-1-target-policy
