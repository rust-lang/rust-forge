# Operations

Here is a list of recurring tasks to support the compiler team and help keep things moving forward. Ideally run through this list every week. If there are blockers or doubts, after having acquired the right context, don't hesitate to ping people around. Keep in mind that contributors are the best resource of the project and we want to be mindful of their time.

## Issues hygiene

- [Issue to be prioritized](https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AI-prioritize): see [prioritization](https://forge.rust-lang.org/compiler/prioritization.html).
- [P-high issues without assignee](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3AT-compiler+label%3AP-high+no%3Aassignee): ideally this category of issues should have an assignee (filter out those without a PR).In rare cases it's fine if they don't.
- [MCP in FCP status](https://github.com/rust-lang/compiler-team/issues?q=is%3Aissue+is%3Aopen+label%3Afinal-comment-period+sort%3Acreated-asc), close seconded since more 10 days, ensure no open concerns
- [Check open MCPs](https://github.com/rust-lang/compiler-team/issues?q=is%3Aissue+is%3Aopen+label%3Amajor-change+-label%3Afinal-comment-period+sort%3Aupdated-asc): [MCP is a protocol](https://forge.rust-lang.org/compiler/proposals-and-stabilization.html) to bring proposals to the compiler team attention. Ensure MCPs are moving towards one of these two outcome, being seconded or being closed for lack of seconding. When it's clear that an MCP won't be seconded or is abandoned, after about two or three months is ok to query its status and evaluate closing it. Otherwise try to get them unstuck.
- [Issues needing a reproducible](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3AE-needs-mcve+label%3AT-compiler+sort%3Acreated-asc)
- [Issues and PRs that are going through FCP](https://github.com/rust-lang/rust/issues?q=sort%3Aupdated-desc+label%3Afinished-final-comment-period): check if the team need to check their box. These issues are in the weekly triage agenda.

## Prioritization for T-compiler

Some useful filters when looking at regressions.

- [Nightly regressions without priority](https://github.com/rust-lang/rust/issues?q=is%3Aissue+label%3AT-compiler+label%3Aregression-from-stable-to-nightly+-label%3AI-prioritize++is%3Aopen)
- [Beta regressions without priority](https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AT-compiler+label%3Aregression-from-stable-to-beta+-label%3AI-prioritize)
- [Stable regressions without priority](https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AT-compiler+label%3Aregression-from-stable-to-stable+-label%3AI-prioritize)
- [Untriaged regressions without a priority](https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AT-compiler+label%3Aregression-untriaged+-label%3AP-critical+-label%3AP-high+-label%3AP-medium+-label%3AP-low+-label%3AI-prioritize)

## PRs hygiene

- Every PR should have a team assigned
  - [PR without a team label](https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Aopen+draft%3Afalse+-label%3AT-libs-api+-label%3AT-libs+-label%3AT-rustdoc+-label%3AT-compiler+-label%3AT-lang+-label%3AT-infra+-label%3AT-release+-label%3AT-types+-label%3AT-style+-label%3AT-bootstrap+-label%3AT-opsem+sort%3Acreated-asc)
  - [Waiting on author](https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Aopen+draft%3Afalse+-label%3AT-libs-api+-label%3AT-libs+-label%3AT-rustdoc+-label%3AT-compiler+-label%3AT-lang+-label%3AT-infra+-label%3AT-release+-label%3AT-types+-label%3AT-style+-label%3AT-bootstrap+label%3AS-waiting-on-author+sort%3Aupdated-asc)
  - [Waiting on a review](https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Aopen+draft%3Afalse+-label%3AT-libs-api+-label%3AT-libs+-label%3AT-rustdoc+-label%3AT-compiler+-label%3AT-lang+-label%3AT-infra+-label%3AT-release+-label%3AT-types+-label%3AT-style+-label%3AT-bootstrap+label%3AS-waiting-on-review+sort%3Aupdated-asc)

## Things to do a week before the release:

- [No regression without priority](https://github.com/rust-lang/compiler-team/issues?q=is%3Aissue+is%3Aopen+label%3Afinal-comment-period+sort%3Acreated-asc): ensure they've been fixed and if not try to get the team attention.
- No [beta regressions](https://github.com/rust-lang/compiler-team/issues?q=is%3Aissue+is%3Aopen+label%3Afinal-comment-period+sort%3Acreated-asc) or [stable regressions](https://github.com/rust-lang/compiler-team/issues?q=is%3Aissue+is%3Aopen+label%3Afinal-comment-period+sort%3Acreated-asc) regressions without an owner, filter out those out without a PR.
- No [beta regressions](https://github.com/rust-lang/rust/issues?q=label%3Aregression-from-stable-to-beta+label%3AT-compiler+is%3Aopen) or [stable regressions](https://github.com/rust-lang/rust/issues?q=label%3Aregression-from-stable-to-stable+label%3AT-compiler+is%3Aopen) regressions work in progress, ideally they should all be merged.
- Ensure breaking changes (i.e. regressions agreed to be acceptable) has a corresponding issue tagged `relnotes-tracking-issue`, see [list of release notes](https://github.com/rust-lang/rust/issues?q=sort%3Aupdated-desc+is%3Aopen+label%3Arelnotes-tracking-issue). T-release will then pick them up and add them to the release notes.

## After the release

- Check which regressions can be closed as "accepted". Add a comment clarifying that the PR causing the regression is accepted as breaking change, example: "Closing since PR #123456 will be mentioned in the release notes". Check carefully, don't be trigger-happy. Discussions and comments about this practice can be directed on [Zulip](https://rust-lang.zulipchat.com/#narrow/channel/242269-t-release.2Ftriage/topic/beta.20regressions.20that.20are.20no.20more/near/456509338).

## Rest of the world

These filters are for checking what's happening in other teams

- [List of open RFCs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3Aproposed-final-comment-period+label%3Adisposition-merge+sort%3Aupdated-asc) (all teams) waiting for the team to discuss or check the proposal, can anything be done to help moving them forward?
