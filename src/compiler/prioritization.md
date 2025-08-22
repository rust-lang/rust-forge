# Prioritization
It is important that the compiler team can quickly identify priority issues, hence the establishment
of a prioritization process, described below.

## General Process
1. Ascertain the current status of the issue
1. Try progressing the issue if possible (e.g. request updates from the issue author/reviewer)
1. Is there an MCVE for the issue already?
1. Check if it's a regression and label it accordingly (`regression-*` labels)
1. Figure out the area the issue belongs and label it accordingly (`A-*` labels)
1. [Ping notify groups](https://rustc-dev-guide.rust-lang.org/notification-groups/about.html) or
   relevant teams
1. Assign if possible

# Priority Levels
As the compiler team's resources are limited, the primary goal of prioritization is to identify the
most relevant issues to work on, so that the compiler team can focus on what matters the most.

Issues relevant to prioritization are bugs and feature requests that are nominated for
prioritization, by adding the `I-prioritize` label as described below.

## Labels
Labeling an issue as `I-prioritize` starts the prioritization process, which will end by
removing the `I-prioritize` label and appending one of the 4 labels we will discuss below:

- `P-critical`
- `P-high`
- `P-medium`
- `P-low`

Each of these labels defines a strategy the team will adopt regarding:

- The amount of focus a given issue will receive
- How members of the community can get involved

## P-critical
A `P-critical` is an issue potentially blocking a compiler release (i.e. highly recommended to be
solved before a new compiler release). These issues will be raised at the compiler team's triage
meeting on a weekly basis.

Examples of things we typically judge to be “critical” bugs:

- Regressions where code that used to compile no longer does
  - Mitigating conditions that may lower priority:
    - If the code should never have compiled in the first place (but if the regression affects a
      large number of crates, this may indicate that we need a warning period)
    - If the code in question is theoretical and considered unlikely to exist in the wild, or if
      it only exists in small, unmaintained packages that are not widely used
  - If a regression has been in stable for a release or two (either because we are still awaiting a
    fix, or because the bug had laid dormant i.e. undetected), we typically lower the priority as
    well, because by that time, if the users have not raised a ruckus about the regression, that
    is a sign that it is inherently not a critical issue
- Regressions where code still compiles but does something different than it used to do (dynamic
  semantics have changed)
  - Mitigating conditions that may lower priority:
    - If code uses feature that is explicitly not specified (e.g. `std::vec::Vec` docs state order
      in which it drops its elements is subject to change)
- Feature-gated features accessible without a feature gate
  - Mitigating conditions that may lower priority:
    - If the pattern is *very* unlikely
- Soundness holes with real-world implications
  - Mitigating conditions that may lower priority:
    - Soundness holes that are difficult to trigger
    - Soundness holes that will not affect stable, e.g. if the hole makes use of a gated unstable
      feature.
- Diagnostic regressions where the diagnostic is very common and the situation very confusing
- ICEs for common scenarios or code patterns
  - Mitigating conditions that may lower priority:
    - If the code that triggers the ICE also triggers compilation errors, and those errors are
      emitted before the ICE
    - If the code in question makes use of unstable features, particularly if the ICE requires a
      feature gate

A `P-critical` issue will receive the most attention. It must be assigned one or several people as
soon as possible, and the rest of the team should do their best to help them out if/when applicable.

## P-high
`P-high` issues are issues that need attention from the compiler team, but not to the point that
they need to be discussed at every meeting. They can be `P-critical` issues that have a mitigating
condition as defined above, or important issues that aren't deemed blockers.

Because there are too many `P-high` issues to fit in every compiler meeting, they should rather be
handled asynchronously by the team's prioritization, in order to help them move forward. They can
still occasionally be brought up at meetings when it is deemed necessary.

The effectiveness of the team's prioritization will be a direct consequence of the ability to draw
the line between `P-critical` and `P-high` issues. There shouldn't be too many `P-critical` issues
that compiler meetings become unmanageable, but critical issues shouldn't get lost in the list of
`P-high` issues.

`P-high` issues are issues the teams will mostly work on. We want to make sure they're assigned,
and keep an eye on them. They are routinely reviewed in batches by the compiler team, deciding a
possible priority downgrade.

## P-medium and P-low
`P-medium` refer to issues that aren't a priority for the team, and that will be resolved in the
long run. For example, issues that will be fixed after a specific feature has landed. They are
issues that the team could mentor someone interested in fixing. They will remain in this state
until someone complains, a community member fixes it, or it gets fixed by accident.

`P-low` refer to issues issue that the compiler team doesn't plan to resolve, but are still worth
fixing. Nominate the issue if it's unclear and needs to be discussed.

# Generating the triage meeting agenda
The triage meeting agenda is generated automatically using the prioritization efforts as input.
It is generated from a template available on [HackMD][template_hackmd] or [GitHub][template_github].

[template_hackmd]: https://hackmd.io/WQW0yzDDS16YvtBNurmj6A
[template_github]: https://github.com/rust-lang/compiler-team/blob/master/templates/T-compiler%20Meeting%20Agenda%20YYYY-MM-DD.md

First, ensure that relevant issues are labelled as `T-compiler`..

- [Issues labeled with `I-prioritize`](https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+-label%3AP-critical+-label%3AP-high+-label%3AP-medium+-label%3AP-low+label%3AI-prioritize+-label%3AT-compiler+-label%3AT-cargo+-label%3AT-core+-label%3AT-doc+-label%3AT-infra+-label%3AT-lang+-label%3AT-libs+-label%3AT-libs-api+-label%3AT-release+-label%3AT-rustdoc+-label%3AA-rustdoc+-label%3AA-rustdoc-ui)
- [Pull requests nominated for the stable release channel backport](https://github.com/rust-lang/rust/issues?q=is%3Aall+label%3Astable-nominated+-label%3Astable-accepted+-label%3AT-compiler+-label%3AT-cargo+-label%3AT-core+-label%3AT-doc+-label%3AT-infra+-label%3AT-lang+-label%3AT-libs+-label%3AT-libs-api+-label%3AT-release+-label%3AT-rustdoc+-label%3AA-rustdoc+-label%3AA-rustdoc-ui)
- [Pull requests nominated for the beta release channel backport](https://github.com/rust-lang/rust/issues?q=is%3Aall+label%3Abeta-nominated+-label%3Abeta-accepted+-label%3AT-compiler+-label%3AT-cargo+-label%3AT-core+-label%3AT-doc+-label%3AT-infra+-label%3AT-lang+-label%3AT-libs+-label%3AT-libs-api+-label%3AT-release+-label%3AT-rustdoc+-label%3AA-rustdoc+-label%3AA-rustdoc-ui)
- [Issues labeled `I-compiler-nominated`](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3AI-nominated+-label%3AT-compiler+-label%3AT-cargo+-label%3AT-core+-label%3AT-doc+-label%3AT-infra+-label%3AT-lang+-label%3AT-libs+-label%3AT-libs-api+-label%3AT-release+-label%3AT-rustdoc+-label%3AA-rustdoc+-label%3AA-rustdoc-ui) (i.e. needing a T-compiler discussion)
- [Pull requests waiting on a team's feedback](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3AS-waiting-on-team+-label%3AT-compiler+-label%3AT-cargo+-label%3AT-core+-label%3AT-doc+-label%3AT-infra+-label%3AT-lang+-label%3AT-libs+-label%3AT-libs-api+-label%3AT-release+-label%3AT-rustdoc+-label%3AA-rustdoc+-label%3AA-rustdoc-ui)
- [Issues classified with priority `P-high`](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3AP-high+-label%3AT-compiler+-label%3AT-cargo+-label%3AT-core+-label%3AT-doc+-label%3AT-infra+-label%3AT-lang+-label%3AT-libs+-label%3AT-libs-api+-label%3AT-release+-label%3AT-rustdoc+-label%3AA-rustdoc+-label%3AA-rustdoc-ui)

..and that prioritization has been completed. Regressions labeled with `I-prioritize` are signaling
that a priority assessment is waiting. When this label is added to an issue, the `triagebot` sends a
notification to the [`#t-compiler/prioritization/alerts`][prio_channel] Zulip channel.

[prio_channel]: https://rust-lang.zulipchat.com/#narrow/channel/245100-t-compiler.2Fprioritization.2Falerts

To assign a priority, replace the `I-prioritize` label with one of `P-critical`, `P-high`,
`P-medium` or `P-low` and add a succinct comment to link the Zulip discussion where the issue
prioritization occurred, example of a template for the comment:

> Assigning priority (discussion on [Zulip](#)).
>
> @rustbot label -I-prioritize +P-XXX

Tip: use [Github Saved Replies](https://docs.github.com/get-started/writing-on-github/working-with-saved-replies) to create a template comment.

Ideally, all [`T-compiler` issues with a `I-prioritize` label][issues_needing_prio] should have a
priority assigned, or strive to reach this goal: sometimes different factors are blocking issues
from being assigned a priority label, either because the report or the context is unclear or because
cannot be reproduced and an MCVE would help. Don't hesitate to ask for clarifications to the issue
reporter or to other contributors.

Review [stable][stable_regressions], [beta][beta_regressions] and [nightly][nightly_regressions] and
try to ensure they are assigned when possikle.

[issues_needing_prio]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AT-compiler+-label%3AP-critical+-label%3AP-high+-label%3AP-medium+-label%3AP-low+label%3AI-prioritize
[stable_regressions]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Aregression-from-stable-to-stable+-label%3AP-critical+-label%3AP-high+-label%3AP-medium+-label%3AP-low+-label%3AT-infra+-label%3AT-libs+-label%3AT-libs-api+-label%3AT-release+-label%3AT-rustdoc
[beta_regressions]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Aregression-from-stable-to-beta+-label%3AP-critical+-label%3AP-high+-label%3AP-medium+-label%3AP-low+-label%3AT-infra+-label%3AT-libs+-label%3AT-libs-api+-label%3AT-release+-label%3AT-rustdoc
[nightly_regressions]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Aregression-from-stable-to-nightly+-label%3AP-critical+-label%3AP-high+-label%3AP-medium+-label%3AP-low+-label%3AT-infra+-label%3AT-libs+-label%3AT-libs-api+-label%3AT-release+-label%3AT-rustdoc

The final step prior to generating the agenda is to accept any MCPs. Any MCPs that have had [the `final-comment-period` label][mcp_fcp]
for more than ten days can be accepted. Remove the `final-comment-period` label and add the `major-change-accepted` label and then
close the issue.

[mcp_fcp]: https://github.com/rust-lang/compiler-team/issues?q=is%3Aissue+is%3Aopen+label%3Amajor-change+label%3Afinal-comment-period

Finally, the meeting agenda can be generated. Clone and build [`triagebot`][triagebot] and run:

[triagebot]: https://github.com/rust-lang/triagebot

```console
$ cargo run --bin prioritization-agenda
```

Copy the content into a new HackMD in the "Rust Lang Compiler Team" space. Copy the most recent
[performance triage logs][perf_triage_log] (doing a bit of cleanup, removing anything that won't display well in Zulip)

[perf_triage_log]: https://github.com/rust-lang/rustc-perf/tree/master/triage#triage-logs

Add additional manual details to the agenda:

- Add summaries of stable/beta nominations (e.g. who nominated the backport and why)
- Add summaries of PRs waiting on the team (i.e. why are they waiting)
- Add initial impressions of `P-critical`/`P-high` bugs
- Add summaries of nominated issues (e.g. who the assignee is, why it was nominated, etc)
- Populate the oldest PRs waiting on review
  - Use judgement to determine whether a ping is appropriate (e.g. if the pull request is an
    experiment, it may not need a review; how long has it been since review activity; what do
    recent comments say?)

About two hours prior to the meeting, announce and share the completed agenda in the Zulip thread for the
upcoming meeting (creating it if it does not already exist):

```text
Hello @*T-compiler/meeting*, triage meeting in about 2h.
Pre-triage done in #**t-compiler/prioritization/alerts**.
Meeting agenda [on HackMD](https://hackmd.io/aaabbbccc123456)
```

It is always recommended to re-run the generator and copy any new details over to the agenda as
issue statuses on GitHub may have changed.

After the meeting, there are a few closing tasks:

- Lock the agenda on HackMD assigning write permissions to `Owners`.
- Remove the `to-announce` label from [MCPs], unless this label was added exactly during
  the meeting (and therefore will be seen during the following meeting).
- Remove `to-announce` FCPs from [`rust-lang/rust`][rust_announce], [`compiler-team`][team_announce]
  and the [forge][forge_announce]. Same disclaimer as before regarding changes during the meeting.
- Accept or decline [`beta nominated`][beta_nominated] and [`stable nominated`][stable_nominated]
  backports that have been accepted during the meeting. For more info check [`t-release` backporting
  docs][release_backports]
  - To accept a backport, add a `{beta,stable}-accepted` label and keep the `{beta,stable}-nominated`
    label. Other automated procedures will process these pull requests, it's important to leave both
  labels. Add a comment on Github linking the Zulip discussion.
  - To decline a backport, simply remove `{beta,stable}-nominated` label. Add a comment on Github
  explaining why the backport was declined and link the Zulip discussion.
- Remove [`I-compiler-nominated`][compiler_nominated] label from issues that were discussed.
  Sometimes not all nominated issues are discussed (because of time constraints) and can slip to the
  next meeting.

[beta_nominated]: https://github.com/rust-lang/rust/issues?q=is%3Apr+label%3Abeta-nominated+-label%3Abeta-accepted
[stable_nominated]: https://github.com/rust-lang/rust/issues?q=is%3Apr+label%3Astable-nominated+-label%3Astable-accepted
[rust_announce]: https://github.com/rust-lang/rust/issues?q=label%3Afinished-final-comment-period%20label%3Ato-announce%20is%3Aissue
[team_announce]: https://github.com/rust-lang/compiler-team/issues?q=label%3Afinished-final-comment-period%20label%3Ato-announce%20is%3Aissue
[forge_announce]: https://github.com/rust-lang/rust-forge/issues?q=label%3Afinished-final-comment-period%20label%3Ato-announce%20is%3Aissue
[fcps]: https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change+label%3Ato-announce
[mcps]: https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Ato-announce%20is%3Aissue
[release_backports]: ../release/backporting.md
[compiler_nominated]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3AI-compiler-nominated+label%3AT-compiler
