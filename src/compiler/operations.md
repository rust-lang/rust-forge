# Operations

"Operations" is a part of the Compiler Team that takes care of organizational and maintenance work and in general help things moving forward. T-compiler ops lives on Zulip under [#t-compiler/ops][t-compiler-ops].

Here is a list of recurring tasks. Ideally run through this list every week. If there are blockers or doubts, after having acquired the right context, don't hesitate to ping people around. Contributors are the best resource of the project (and we want to be mindful of their time) and are always helpful.

You can trigger a discussion about a specific topic, issue or pull request by opening a new topic on Zulip in [#t-compiler][t-compiler] or, if it needs consensus and more focus from the team, it can can be labeled `I-compiler-nominated` and will be discussed in a meeting (see section [Meetings][meetings]).

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

Some useful filters when looking at regressions.

- [Nightly regressions without priority](https://github.com/rust-lang/rust/issues?q=is%3Aissue+label%3AT-compiler+label%3Aregression-from-stable-to-nightly+-label%3AI-prioritize++is%3Aopen)
- [Beta regressions without priority](https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AT-compiler+label%3Aregression-from-stable-to-beta+-label%3AI-prioritize)
- [Stable regressions without priority](https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AT-compiler+label%3Aregression-from-stable-to-stable+-label%3AI-prioritize)
- [Untriaged regressions without a priority](https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AT-compiler+label%3Aregression-untriaged+-label%3AP-critical+-label%3AP-high+-label%3AP-medium+-label%3AP-low+-label%3AI-prioritize)

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

T-compiler has two kinds of meetings: triage and design meetings. Triage meetings happen weekly, there is a [tool](https://github.com/rust-lang/triagebot/blob/master/src/bin/prioritization-agenda.rs) to generate 80% of the meeting's agenda. Design meetings proposals are advanced on the [T-compiler repository](https://github.com/rust-lang/compiler-team/issues?q=sort%3Aupdated-desc%20is%3Aissue%20is%3Aopen%20label%3Ameeting-proposal) and scheduled during recurrent *steering* meetings (where the next *design* meetings are scheduled). Design meetings also need an agenda and a bit of work to summarize the topic and bring together documentation, invite relevant people and so on.

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
