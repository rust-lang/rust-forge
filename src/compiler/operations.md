# Operations

"Operations" is a part of the Compiler Team that takes care of organizational and maintenance work and in general help things moving forward. T-compiler ops lives on Zulip under [#t-compiler/ops][t-compiler-ops].

Here is a list of recurring tasks. Ideally run through this list every week. If there are blockers or doubts, after having acquired the right context, don't hesitate to ping people around. Contributors are the best resource of the project (and we want to be mindful of their time) and are always helpful.

You can trigger a discussion about a specific topic, issue or pull request by opening a new topic on Zulip in [#t-compiler][t-compiler] or, if it needs consensus and more focus from the team, it can be labeled `I-compiler-nominated` and will be discussed in a meeting (see section [Meetings][meetings]).

[meetings]: #meetings
[t-compiler]: https://rust-lang.zulipchat.com/#narrow/channel/131828-t-compiler
[t-compiler-ops]: #

## Issues hygiene

- [Issue to be prioritized](https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AI-prioritize): see [prioritization](https://forge.rust-lang.org/compiler/prioritization.html).
- [P-high issues without assignee](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3AT-compiler+label%3AP-high+no%3Aassignee): ideally this category of issues should have an assignee (filter out those without a PR). In rare cases it's fine if they don't.
- [MCP in FCP status](https://github.com/rust-lang/compiler-team/issues?q=is%3Aissue+is%3Aopen+label%3Afinal-comment-period+sort%3Acreated-asc), close seconded since more 10 days, ensure no open concerns
- [Check open MCPs](https://github.com/rust-lang/compiler-team/issues?q=is%3Aissue+is%3Aopen+label%3Amajor-change+-label%3Afinal-comment-period+sort%3Aupdated-asc): [MCP is a protocol](https://forge.rust-lang.org/compiler/proposals-and-stabilization.html) to bring proposals to the compiler team attention. Ensure MCPs are moving towards one of these two outcome, being seconded or being closed for lack of seconding. When it's clear that an MCP won't be seconded or is abandoned, after about two or three months is ok to query its status and evaluate closing it. Otherwise try to get them unstuck.
- [Issues needing a MCVE](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3AE-needs-mcve+label%3AT-compiler+sort%3Acreated-asc)
- [Issues and PRs that are going through FCP](https://github.com/rust-lang/rust/issues?q=sort%3Aupdated-desc+label%3Afinished-final-comment-period): check if the team need to check their box. These issues are in the weekly triage meeting agenda.

## Prioritization for T-compiler

See here how to [prioritization][prioritization] works.

Some useful filters when looking at regressions.

- [Nightly regressions without priority](https://github.com/rust-lang/rust/issues?q=is%3Aissue+label%3AT-compiler+label%3Aregression-from-stable-to-nightly+-label%3AI-prioritize++is%3Aopen)
- [Beta regressions without priority](https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AT-compiler+label%3Aregression-from-stable-to-beta+-label%3AI-prioritize)
- [Stable regressions without priority](https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AT-compiler+label%3Aregression-from-stable-to-stable+-label%3AI-prioritize)
- [Untriaged regressions without a priority](https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AT-compiler+label%3Aregression-untriaged+-label%3AP-critical+-label%3AP-high+-label%3AP-medium+-label%3AP-low+-label%3AI-prioritize)

[prioritization]: ./prioritization.md

## PRs hygiene

- Every PR should have a team assigned
  - [PR without a team label](https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Aopen+draft%3Afalse+-label%3AT-libs-api+-label%3AT-libs+-label%3AT-rustdoc+-label%3AT-rustdoc-frontend+-label%3AT-compiler+-label%3AT-lang+-label%3AT-infra+-label%3AT-release+-label%3AT-types+-label%3AT-style+-label%3AT-bootstrap+-label%3AT-opsem+sort%3Acreated-asc)
  - [Waiting on author](https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Aopen+draft%3Afalse+-label%3AT-libs-api+-label%3AT-libs+-label%3AT-rustdoc+-label%3AT-rustdoc-frontend+-label%3AT-compiler+-label%3AT-lang+-label%3AT-infra+-label%3AT-release+-label%3AT-types+-label%3AT-style+-label%3AT-bootstrap+label%3AS-waiting-on-author+sort%3Aupdated-asc)
  - [Waiting on a review](https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Aopen+draft%3Afalse+-label%3AT-libs-api+-label%3AT-libs+-label%3AT-rustdoc+-label%3AT-rustdoc-frontend+-label%3AT-compiler+-label%3AT-lang+-label%3AT-infra+-label%3AT-release+-label%3AT-types+-label%3AT-style+-label%3AT-bootstrap+label%3AS-waiting-on-review+sort%3Aupdated-asc)

## Things to do a week before the release:

- [No regression without priority](https://github.com/rust-lang/rust/issues?q=label%3AT-compiler%20is%3Aopen%20%20label%3AI-prioritize): ensure they've been fixed and if not try to get the team attention.
- No [beta regressions](https://github.com/rust-lang/rust/issues?q=label%3Aregression-from-stable-to-beta%20label%3AT-compiler%20is%3Aopen%20no%3Aassignee) or [stable regressions](https://github.com/rust-lang/rust/issues?q=label%3Aregression-from-stable-to-stable%20label%3AT-compiler%20is%3Aopen%20no%3Aassignee) regressions without an owner, filter out those out without a PR.
- No [beta regressions](https://github.com/rust-lang/rust/issues?q=label%3Aregression-from-stable-to-beta+label%3AT-compiler+is%3Aopen) or [stable regressions](https://github.com/rust-lang/rust/issues?q=label%3Aregression-from-stable-to-stable+label%3AT-compiler+is%3Aopen) regressions work in progress, ideally they should all be merged.
- Ensure breaking changes (i.e. regressions agreed to be acceptable) have a corresponding issue tagged `relnotes-tracking-issue`, see [list of release notes](https://github.com/rust-lang/rust/issues?q=sort%3Aupdated-desc+is%3Aopen+label%3Arelnotes-tracking-issue). T-release will then pick them up and add them to the release notes.

## After the release

- Check carefully which regressions can be closed as "accepted". Add a comment clarifying that the PR causing the regression is accepted as breaking change, example: "Closing since PR #123456 will be mentioned in the release notes". Discussions and comments about this practice can be directed on [Zulip](https://rust-lang.zulipchat.com/#narrow/channel/242269-t-release.2Ftriage/topic/beta.20regressions.20that.20are.20no.20more/near/456509338).

## Meetings

T-compiler has two kinds of meetings: triage and design meetings. Triage meetings happen weekly on Thursdays (you can subscribe to the Team Compiler calendar [from this repository][compiler-calendar]
), there is a [tool](https://github.com/rust-lang/triagebot/blob/master/src/bin/prioritization-agenda.rs) to generate 80% of the meeting's agenda (see [Triage meetings](#triage-meetings) for details). Design meetings proposals are advanced on the [T-compiler repository](https://github.com/rust-lang/compiler-team/issues?q=sort%3Aupdated-desc%20is%3Aissue%20is%3Aopen%20label%3Ameeting-proposal) and scheduled during recurrent *steering* meetings (where the next *design* meetings are scheduled). Design meetings also need an agenda and a bit of work to summarize the topic and bring together documentation, invite relevant people and so on.

[compiler-calendar]: https://github.com/rust-lang/calendar

### Triage meetings

First, ensure that relevant issues are labelled as `T-compiler`:

- [Issues labeled with `I-prioritize`](https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+-label%3AP-critical+-label%3AP-high+-label%3AP-medium+-label%3AP-low+label%3AI-prioritize+-label%3AT-compiler+-label%3AT-cargo+-label%3AT-core+-label%3AT-doc+-label%3AT-infra+-label%3AT-lang+-label%3AT-libs+-label%3AT-libs-api+-label%3AT-release+-label%3AT-rustdoc+-label%3AA-rustdoc+-label%3AA-rustdoc-ui)
- [Pull requests nominated for the stable release channel backport](https://github.com/rust-lang/rust/issues?q=is%3Aall+label%3Astable-nominated+-label%3Astable-accepted+-label%3AT-compiler+-label%3AT-cargo+-label%3AT-core+-label%3AT-doc+-label%3AT-infra+-label%3AT-lang+-label%3AT-libs+-label%3AT-libs-api+-label%3AT-release+-label%3AT-rustdoc+-label%3AA-rustdoc+-label%3AA-rustdoc-ui)
- [Pull requests nominated for the beta release channel backport](https://github.com/rust-lang/rust/issues?q=is%3Aall+label%3Abeta-nominated+-label%3Abeta-accepted+-label%3AT-compiler+-label%3AT-cargo+-label%3AT-core+-label%3AT-doc+-label%3AT-infra+-label%3AT-lang+-label%3AT-libs+-label%3AT-libs-api+-label%3AT-release+-label%3AT-rustdoc+-label%3AA-rustdoc+-label%3AA-rustdoc-ui)
- [Issues labeled `I-compiler-nominated`](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3AI-nominated+-label%3AT-compiler+-label%3AT-cargo+-label%3AT-core+-label%3AT-doc+-label%3AT-infra+-label%3AT-lang+-label%3AT-libs+-label%3AT-libs-api+-label%3AT-release+-label%3AT-rustdoc+-label%3AA-rustdoc+-label%3AA-rustdoc-ui) (i.e. needing a T-compiler discussion)
- [Pull requests waiting on a team's feedback](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3AS-waiting-on-team+-label%3AT-compiler+-label%3AT-cargo+-label%3AT-core+-label%3AT-doc+-label%3AT-infra+-label%3AT-lang+-label%3AT-libs+-label%3AT-libs-api+-label%3AT-release+-label%3AT-rustdoc+-label%3AA-rustdoc+-label%3AA-rustdoc-ui)
- [Issues classified with priority `P-high`](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3AP-high+-label%3AT-compiler+-label%3AT-cargo+-label%3AT-core+-label%3AT-doc+-label%3AT-infra+-label%3AT-lang+-label%3AT-libs+-label%3AT-libs-api+-label%3AT-release+-label%3AT-rustdoc+-label%3AA-rustdoc+-label%3AA-rustdoc-ui)
- [Issues classified with priority `P-critical`](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3AP-critical+-label%3AT-compiler+-label%3AT-cargo+-label%3AT-core+-label%3AT-doc+-label%3AT-infra+-label%3AT-lang+-label%3AT-libs+-label%3AT-libs-api+-label%3AT-release+-label%3AT-rustdoc+-label%3AA-rustdoc+-label%3AA-rustdoc-ui)

..and that prioritization has been completed. Regressions labeled with `I-prioritize` are signaling
that a priority assessment is waiting. When this label is added to an issue, the `triagebot` sends a
notification to the [`#t-compiler/prioritization/alerts`][prio_channel] Zulip channel.

[prio_channel]: https://rust-lang.zulipchat.com/#narrow/channel/245100-t-compiler.2Fprioritization.2Falerts

Ideally, all [`T-compiler` issues with a `I-prioritize` label][issues_needing_prio] should have a
priority assigned, or strive to reach this goal: sometimes different factors are blocking issues
from being assigned a priority label, either because the report or the context is unclear or because
cannot be reproduced and an MCVE would help. Don't hesitate to ask for clarifications to the issue
reporter or to other contributors.

Review [stable][stable_regressions], [beta][beta_regressions] and [nightly][nightly_regressions] and
try to ensure they are assigned when possible.

[issues_needing_prio]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AT-compiler+-label%3AP-critical+-label%3AP-high+-label%3AP-medium+-label%3AP-low+label%3AI-prioritize
[stable_regressions]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Aregression-from-stable-to-stable+-label%3AP-critical+-label%3AP-high+-label%3AP-medium+-label%3AP-low+-label%3AT-infra+-label%3AT-libs+-label%3AT-libs-api+-label%3AT-release+-label%3AT-rustdoc
[beta_regressions]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Aregression-from-stable-to-beta+-label%3AP-critical+-label%3AP-high+-label%3AP-medium+-label%3AP-low+-label%3AT-infra+-label%3AT-libs+-label%3AT-libs-api+-label%3AT-release+-label%3AT-rustdoc
[nightly_regressions]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Aregression-from-stable-to-nightly+-label%3AP-critical+-label%3AP-high+-label%3AP-medium+-label%3AP-low+-label%3AT-infra+-label%3AT-libs+-label%3AT-libs-api+-label%3AT-release+-label%3AT-rustdoc

The final step prior to generating the agenda is to accept Major Change Proposals (MCP), this is
usually automated. MCPs that have been in the Final Comment Period (FCP) phase (identified by having
the [`final-comment-period` label][mcp_fcp]) for more than ten days can be accepted. If an MCP has
no unresolved concerns (look for the `has-concerns` label), you can remove the
`final-comment-period` label, add the `major-change-accepted` label and close the issue.

[mcp_fcp]: https://github.com/rust-lang/compiler-team/issues?q=is%3Aissue+is%3Aopen+label%3Amajor-change+label%3Afinal-comment-period

Finally, the meeting agenda can be generated. Clone and build [`triagebot`][triagebot] and run:

[triagebot]: https://github.com/rust-lang/triagebot

```console
$ cargo run --bin prioritization-agenda
```

Copy the content into a new HackMD in the "Rust Lang Compiler Team" space. The tool will also
download the latest weekly compiler triage logs. In case it didn't work out, manually copy the most
recent [performance triage logs][perf_triage_log] (doing a bit of cleanup, removing anything that
won't display well in Zulip)

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

## Rest of the world

These filters are for checking what's happening in other teams

- [List of open RFCs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3Aproposed-final-comment-period+label%3Adisposition-merge+sort%3Aupdated-asc) (all teams) waiting for the team to discuss or check the proposal, can anything be done to help moving them forward?

## Useful tips

### Github Issues Dashboard

You can utilize the [GitHub Issues Dashboard](https://github.com/issues/) to create custom filters. The filters allow you to aggregate both issues and PRs from *multiple* repositories, and allows applying [advanced filters][adv-filters]. See <https://github.blog/changelog/2025-04-02-github-issues-dashboard-updates/>.

[adv-filters]: https://docs.github.com/en/issues/tracking-your-work-with-issues/using-issues/filtering-and-searching-issues-and-pull-requests#building-advanced-filters-for-issues

#### Example custom filters

Mostly intended for [rust-lang/rust]

| Filter                                                                     | Description                                  |
|----------------------------------------------------------------------------|----------------------------------------------|
| `repo:rust-lang/rust label:P-critical is:open`                             | Open P-critical issues                       |
| `repo:rust-lang/rust label:T-compiler label:P-high is:open`                | Open P-high T-compiler issues                |
| `repo:rust-lang/rust label:needs-triage -label:relnotes`                   | Untriaged issues                             |
| `repo:rust-lang/rust label:regression-untriaged`                           | Untriaged regressions                        |
| `repo:rust-lang/rust label:proposed-final-comment-period`                  | Issues/PRs with on-going FCP                 |
| `repo:rust-lang/rust label:proposed-final-comment-period label:T-compiler` | Issues/PRs with on-going T-compiler FCP      |
| `repo:rust-lang/rust label:I-prioritize`                                   | Unprioritized issues                         |
| `repo:rust-lang/rust label:needs-triage label:relnotes-tracking-issue`     | Untriaged/unedited relnotes issues           |
| `repo:rust-lang/rfcs label:T-compiler is:pr is:open`                       | RFCs concerning T-compiler                   |
| `repo:rust-lang/rfcs label:T-compiler label:proposed-final-comment-period` | RFCs concerning T-compiler with on-going FCP |


[rust-lang/rust]: https://github.com/rust-lang/rust
