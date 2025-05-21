# Proposals, Approvals and Stabilization
It is very common to need to gather feedback and approval when contributing to rustdoc, either
for permission to proceed with an experiment or refactoring, or when stabilizing a feature. This
document aims to summarise the various processes that the rustdoc team has for making approval
decisions and when each should be used.

## Approvals
There are two mechanisms that the team can use to approve a proposal (not all approval mechanisms
are suitable for each method of making a proposal - see below):

- r+
  - A proposal is r+'d when it is approved to be merged.
  - r+ can only be used to approve a PR.
- FCP
  - A final comment period will require sign-off from a majority of the rustdoc team to approve
    a proposal and then a ten day waiting period.
  - FCPs can be used to approve any form of proposal.

## Proposals
There are three ways to propose a change to the rustdoc team. The appropriate choice depends on
the nature of the proposal, described below.

- Open a discussion on the [rustdoc zulip thread].
  - This is the preferred way. It allows to prevent users to lose too much time implementing
    something if in the end, the team will ask major changes or even refuse it. After the
    discussion, if accepted and depending on the change, an RFC or a PR will be the next step.
- Request For Comments (RFC)
  - RFCs are pull requests to the [`rust-lang/rfcs`][rfcs] repository and are a heavy-weight
    proposal mechanism, reserved for significant changes.
  - RFC proposals can only be approved by *FCPs*.
- Pull Request (PR)
  - PRs are pull requests to the [`rust-lang/rust`][rust] repository and are a light-weight
    proposal mechanism, suitable for most proposals. PRs are preferred when the proposal is
    accompanied by a small patchset (such as stabilization of a compiler flag or addition of
    a new target).
  - PR proposals can be approved by *FCPs* or *r+*.

[rustdoc zulip thread]: https://rust-lang.zulipchat.com/#narrow/channel/266220-t-rustdoc

### When are FCPs/RFCs required?

An FCP will be needed for any stabilization of small user-facing changes, like UI/UX changes, new
command-line arguments, new attributes, etc. However, if the change is considered too big/important,
an RFC will need to be written and approved before the change will be accepted.

When opening an FCP, make sure only the relevant subteam is labeled on the issue, to avoid pinging
people with changes they aren't interested in.

### What happens if someone makes a contribution that requires an approval and doesn't have one?
If the approval required for the contribution requires an RFC, then the contribution
should be closed or marked as blocked, with a request to create an RFC first. If approval of
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
  documentation websites, like [Haddock], [Wikipedia], [Racket]?
* **Alternatives, concerns, and key decisions:** Were there any alternatives considered? If so, why
  did you pick this design?

## What proposal/approval do I need?
This section aims to exhaustively detail which proposal and approval is necessary for any given
circumstance.

[rfcs]: https://github.com/rust-lang/rfcs
[Haddock]: https://haskell-haddock.readthedocs.io/latest/
[Wikipedia]: https://www.wikipedia.org/
[Racket]: https://docs.racket-lang.org/
