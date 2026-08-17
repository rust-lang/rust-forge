# Proposals, Approvals and Stabilization

It is very common to need to gather feedback and approval when contributing to mdBook, either
for permission to proceed with an experiment or refactoring, or when adding a new feature. This
document aims to summarise the various processes that the mdBook team has for making approval
decisions and when each should be used.

## Approvals

There are two mechanisms that the team can use to approve a proposal (not all approval mechanisms
are suitable for each method of making a proposal - see below):

- Add to the merge queue
  - A proposal (an RFC or an FCP) is added to the merge queue when it is approved to be merged.
- FCP
  - A final comment period will require sign-off from a majority (all members minus 2)
    of the mdBook team to approve a proposal and then a ten day waiting period.
  - FCPs can be used to approve any form of proposal.

## Proposals

There are three ways to propose a change to the mdBook team. The appropriate choice depends on
the nature of the proposal, described below.

- Open a discussion on the [mdBook zulip thread].
  - This is the preferred way. It allows to prevent users to lose too much time implementing
    something if in the end, the team will ask major changes or even refuse it. After the
    discussion, if accepted and depending on the change, an RFC or a PR will be the next step.
- Pull Request (PR)
  - Opening a pull request on the [`rust-lang/mdBook`][mdbook] repository is a lightweight
    mechanism suitable for most proposals.
  - PR proposals can be approved by *FCPs* or *by being added to the merge queue*. See
    *When are FCPs required?* section below when *being added to the merge queue* isn't
    sufficient alone.
- Issues
  - Opening an issue on the [`rust-lang/mdBook`][mdbook] repository are also a good starting
    point if you don't know which of the previous ways is the best fit.

[mdBook zulip thread]: https://rust-lang.zulipchat.com/#narrow/channel/507422-t-mdbook

### When are FCPs required?

An FCP will be needed for any stabilization of user-facing changes, like UI/UX changes,
new command-line arguments, new attributes, etc.

When starting an FCP, make sure only the relevant subteam is labeled on the issue/PR, to avoid
pinging people with changes they aren't interested in.

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
  equivalent tooks?
* **Alternatives, concerns, and key decisions:** Were there any alternatives considered? If so, why
  did you pick this design?

[mdbook]: https://github.com/rust-lang/mdBook/

