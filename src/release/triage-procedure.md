# Triage Procedure

## Pull Request Triage

### Status Tags

- [S-waiting-on-author] - Author needs to make changes to address reviewer
  comments, or merge conflicts/test failures are present. This also covers more
  obscure cases, like a PR being blocked on another, or waiting for a [crater]
  run -- it is the author's responsibility to push the PR forward.
- [S-waiting-on-review] - Review is incomplete
- [S-waiting-on-team] - A `T-` label is marked, and team has been CC'd for
  feedback.
- [S-waiting-on-bors] - Currently approved, waiting to merge. Managed by [bors].
- [S-waiting-on-crater] - Waiting to see what the impact the PR will have on the
  ecosystem
- [S-waiting-on-bikeshed] - Waiting on the consensus over a minor detail
- [S-waiting-on-perf] - Waiting on the results of a perf run
- [S-blocked] - Waiting for another PR to be merged or for discussion to be
  resolved
- [S-inactive] - Hasn't had activity in a while
- [S-blocked-closed] - Closed because resolving the block is expected to take a
  long time
- [S-inactive-closed] - Closed due to inactivity
- [S-experimental] - An experimental PR that likely shouldn't be triaged.
  [S-waiting-on-author] used to be used for this, but [S-experimental]
  communicates that the PR is work-in-progress.

Also: [PRs with no status tags][no-status-tags]. This is useful to find PRs
where highfive conked out and didn't assign a reviewer and thus didn't assign
[S-waiting-on-review]. These PRs can get lost otherwise. (Note that you should
likely not triage PRs that have `r? @ghost` since that means the author does not
yet want a review.)

[s-waiting-on-author]: https://github.com/rust-lang/rust/pulls?q=is%3Aopen+draft%3Afalse+is%3Apr+sort%3Aupdated-asc+label%3AS-waiting-on-author+-label%3AI-nominated+-label%3Aneeds-fcp
[s-waiting-on-review]: https://github.com/rust-lang/rust/pulls?q=is%3Aopen+draft%3Afalse+is%3Apr+sort%3Aupdated-asc+label%3AS-waiting-on-review+-label%3AI-nominated+-label%3Aneeds-fcp
[s-waiting-on-team]: https://github.com/rust-lang/rust/pulls?q=is%3Aopen+is%3Apr+label%3AS-waiting-on-team+sort%3Aupdated-asc
[s-waiting-on-bors]: https://github.com/rust-lang/rust/pulls?q=is%3Aopen+is%3Apr+label%3AS-waiting-on-bors+sort%3Aupdated-asc
[s-waiting-on-crater]: https://github.com/rust-lang/rust/pulls?q=is%3Aopen+is%3Apr+label%3AS-waiting-on-crater+sort%3Aupdated-asc
[s-waiting-on-bikeshed]: https://github.com/rust-lang/rust/pulls?q=is%3Aopen+is%3Apr+label%3AS-waiting-on-bikeshed+sort%3Aupdated-asc
[s-waiting-on-perf]: https://github.com/rust-lang/rust/pulls?q=is%3Aopen+is%3Apr+label%3AS-waiting-on-perf+sort%3Aupdated-asc
[s-blocked]: https://github.com/rust-lang/rust/pulls?q=is%3Aopen+is%3Apr+label%3AS-blocked+sort%3Aupdated-asc
[s-inactive]: https://github.com/rust-lang/rust/pulls?q=is%3Aopen+is%3Apr+label%3AS-inactive+sort%3Aupdated-asc
[s-blocked-closed]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+label%3AS-blocked-closed+sort%3Aupdated-asc
[s-inactive-closed]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+label%3AS-inactive-closed+sort%3Aupdated-asc
[s-experimental]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+label%3AS-experimental+sort%3Aupdated-asc
[no-status-tags]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Aopen+-label%3AS-waiting-on-author+-label%3AS-waiting-on-review+-label%3AS-waiting-on-team+-label%3AS-waiting-on-bors+-label%3AS-waiting-on-crater+-label%3AS-waiting-on-bikeshed+-label%3AS-waiting-on-perf+-label%3AS-blocked+-label%3AS-inactive+-label%3AS-blocked-closed+-label%3AS-inactive-closed+-label%3AS-waiting-on-fcp+-label%3AS-experimental
[crater]: https://github.com/rust-lang-nursery/crater
[bors]: https://github.com/rust-lang/homu

### Procedure

We primarily triage three status labels: S-waiting-on-review,
S-waiting-on-author, and (once in a while) S-blocked. Here is the procedure for
each:

#### S-waiting-on-review

Click [this link][S-waiting-on-review] to see all PRs with the
S-waiting-on-review label. Only triage PRs that were last updated 15 days or
more ago (give or take a day).

For each PR:

1. **If** the PR has new conflicts, CI failed, or a new review has been made
   **then** change the label to S-waiting-on-author and ping the author.

2. Add the PR to your [report].

#### S-waiting-on-author

Click [this link][S-waiting-on-author] to see all PRs with the
S-waiting-on-author label. Only triage PRs that were last updated 15 days or
more ago (give or take a day).

For each PR:

1. **If** the author did what the PR was waiting on them for **then** update the
   label to S-waiting-on-review.

2. **If** the author is a member of a Rust team (not working groups — teams like
   T-compiler, T-lang, T-rustdoc, etc.) **then** don't ping anyone.

   **Otherwise,** ping the author.

3. Add the PR to your [report].

#### S-blocked

You only need to check S-blocked PRs occasionally (e.g., once a month).  Click
[this link][S-blocked] to see all PRs with the S-blocked label.

For each PR:

1. **If** it is still blocked **then** leave it as-is.

   **Otherwise,** if it is no longer blocked, then remove S-blocked (and add a
   status label like S-waiting-on-review if appropriate).

2. Add the PR to your [report].

#### Triage Report
[report]: #triage-report

You should record information about each PR you triage in a report. The report
is just a small document that looks like:

> #### S-waiting-on-review
>
> #12345 20 days - still waiting on review - author: ferris, assignee: bors
>
> \[...\]

However, each person has a different format for their triage reports, so yours
does not need to look like that.

Once you are done triaging PRs, post your report in the topic for the current
week's triage in the `#t-release/triage` Zulip stream.

<div style="margin-bottom: 150px"></div>

-----

> #### Note
>
> The rest of this page is significantly out of date. If you are a member of
> wg-triage, please ask in `#t-release/triage` on Zulip if you have questions
> about procedure.

**IMPORTANT:** Whenever you do PR triage, please fill out the following form:
[goo.gl/forms/YKYVFYjBq28Hm3qQ2](https://goo.gl/forms/YKYVFYjBq28Hm3qQ2). If you
want to create a bookmark for yourself, you can adapt
[this link](https://docs.google.com/forms/d/e/1FAIpQLSfdcbG9tSX49LuY9h1NCLKb6oU4tGalB3PrkN6yIHHWaptUbw/viewform?entry.1711461865=YOUR_GITHUB_USERNAME)
to prefill your GitHub username.

_Note:_ When you are pinging people in triage comments, you should mention that
you are doing triage in the comment you post. For example, start your comments
with something like "Ping from triage ...".

First ensure that the status tag matches the current state of the PR. Change the
tag if necessary, and apply the procedure for the new tag.

#### [Unassigned PRs]

All PRs that have no assignee (except rollups) should be assigned to a random
member of the responsible team.

[unassigned prs]: https://github.com/rust-lang/rust/pulls?q=is%3Aopen+is%3Apr+no%3Aassignee

#### [Unlabeled PRs]

All unlabeled PRs should be processed. The steps below are not mutually
exclusive, any number of them may apply.

When **no review has happened**, if the PR is a work in progress (e.g., test
failures, merge conflict) mark `S-waiting-on-author`. Otherwise, mark
`S-waiting-on-review`. If no human has checked in yet and you don't recognise
the submitter as a regular contributor, leave a comment saying something like
"Thanks for the PR! We’ll periodically check in on it to make sure that
@reviewer or someone else from the team reviews it soon."

At this point, all PRs must have a tag applied.

[unlabeled prs]: https://github.com/rust-lang/rust/pulls?utf8=%E2%9C%93&q=is%3Aopen%20is%3Apr%20sort%3Aupdated-asc%20-label%3AS-inactive%20-label%3AS-waiting-on-author%20-label%3AS-waiting-on-team%20-label%3AS-waiting-on-bors%20-label%3AS-waiting-on-crater%20-label%3AS-waiting-on-team%20-label%3AS-waiting-on-review%20-label%3AS-blocked%20-label%3AS-waiting-on-bikeshed%20-label%3AS-waiting-on-fcp%20-label%3AS-waiting-on-perf

#### [S-waiting-on-author PRs][s-waiting-on-author]

PRs with, roughly, more than a week of inactivity need to be processed. These
can be found by looking at the "updated X days ago" on GitHub's PR list.

If the author hasn't responded for more than a week to a request for changes or
a status update, ping the author on GitHub asking for them to check in on the
PR's state. If they've given advance warning that they are unavailable for a
period of time and therefore won't be able to address comments, do not ping
until after that time. It is a good idea to start the message with "Ping from
Triage..." so that the concerned parties know it is coming from the triage team.

If the author has not responded to a previous ping, meaning more than 2 weeks
have passed with no activity, the PR should be closed with a message thanking
the author for their work, asking the them to reopen when they have a chance to
make the necessary changes, and warning them not to push to the PR while it is
closed as that prevents it from being reopened. Tag the PR with
`S-inactive-closed`.

_TIP_: if an author is unavailable and you know they won't have a chance to come
to a PR for a while, you can 'bump' the PR by removing and readding the tag
(note that removing/readding requires clicking off the tag selection dropdown
between the two actions).

If the PR is blocked on another PR, issue, or some kind of discussion, add a
comment clearly identifying what is blocking the PR (something like "This PR
appears to be blocked on #12345") and change the state to `S-blocked`. Follow
the instruction for `S-blocked` to determine whether you should also close the
PR.

#### [S-waiting-on-review PRs][s-waiting-on-review]

PRs with, roughly, more than a week of inactivity need to be processed. These
can be found by looking at the "updated X days ago" on GitHub's PR list.

If the review is complete the label should be changed from `S-waiting-on-review`
to `S-waiting-on-author`.

Otherwise, the reviewer should be pinged. It is a good idea to start the message
with "Ping from Triage..." so that the concerned parties know it is coming from
the triage team, and the message should be asking the reviewer to either review
or update a review of the PR. If the reviewer has already been pinged, meaning
more than 2 weeks have passed with no activity, another reviewer on their team
should be pinged. Note that if the reviewer has expressed that they are busy, do
not ping them until they are available again. If the PR is not already labeled
with a team (`T-`), find the team assigned to the PR's issue which should have a
`T-` label.

The `r?` command is needed to override a reviewer, however not all triagers will
have sufficient permissions. In this case sending a message to the `#triage-wg`
Discord or pinging @Dylan-DPC will be necessary.

If the PR is blocked on another PR, add a comment clearly identifying the
blocking PR (something like "This PR appears to be blocked on #12345") and
change the state to `S-blocked`.

If the pr is tagged with `final-comment-period` it does not need to be triaged unless the process has stalled for a reasonable period of time. These PRs have a form from RFCbot that looks like:

> Team member **@Donald** has proposed to merge this. The next step is review by the rest of the tagged team members:
> * [ ] @Huey
> * [x] @Dewey
> * [ ] @Louie

At this point, ping the appropriate people to check their boxes to sign off on the PR.

If *this* stalls nominate the PR for prioritizing at the next team triage meeting by marking it with `I-nominated`.

PRs tagged with `finshed-final-comment-period` are eligible for triage.

#### [S-waiting-on-team PRs][s-waiting-on-team]

PRs active within the last 4 days or inactive for greater than 2 weeks need to
be processed. These can be found by looking at the "updated X days ago" on
GitHub's PR list.

First, ensure that the status tag matches the current state of the PR. Change
the tag if necessary, and apply the procedure for the new tag now. Verify that
there is a T- tag for all PRs that remain in this category.

If the PR has been inactive for greater than 2 weeks, add the `I-nominated`
label and ping the team, requesting a new assignee or other appropriate action.

If there has been recent activity, the team might have taken some action meaning
the state has changed but the label has not yet been updated. Therefore, we also
check the most recent ones.

#### [S-waiting-on-bors PRs][s-waiting-on-bors]

[Bors] automatically manages this label but human intervention may be required
if there is an issue.

#### [S-waiting-on-crater PRs][s-waiting-on-crater]

All PRs should be processed.

If the PR has been active in the last three days, make sure it's present on the
crater
[spreadsheet](https://docs.google.com/spreadsheets/d/1VPi_7ErvvX76fa3VqvQ3YnQmDk3bS7fYnkzvApIWkKo/edit#gid=0).
Fill in the link to the PR and set status as "Pending".

If crater has been run and results include failures, change the tag to
`S-waiting-on-review` for the reviewer to be responsible for deciding what
should be done with the information provided by the failures.

If crater has been run and the results do not include failures, change the tag
to `S-waiting-on-review` for the reviewer to take one last look and approve.

If crater has not been run and it has been more than 3 days since a crater run
was requested, ping the last three distinct listed people on the spreadsheet in
the infra irc channel and request a crater run.

If crater has been started (the person starting should leave a comment) and it
has been more than 5 days since an update, ping the person starting the run on
IRC and GitHub.

#### [S-waiting-on-bikeshed][s-waiting-on-bikeshed]

PRs inactive for greater than 7 days need to be processed. These can be found by
looking at the "updated X days ago" on GitHub's PR list.

Find the source of the discussion and see if it has been resolved.

If it has been resolved, move it back to `S-waiting-on-author` or
`S-waiting-on-review` as appropriate. Add a comment notifying the author or
reviewer that the PR is now unblocked.

If it has not been resolved, remove and re-add the `S-waiting-on-bikeshed` tag.
This resets the update time so the PR won't be reviewed for another week.

#### [S-blocked PRs][s-blocked]

Blocked PRs can remain blocked for a long time. To avoid needlessly re-triaging
them, they should be closed if the blocking issue is unlikely to be resolved
soon. If you close a blocked PR, apply the S-blocked-closed label and invite the
author to re-open the PR once the issue has been resolved. If you feel
uncomfortable just closing the PR, feel free to link to this document. As a rule
of thumb, consider these guidelines:

- PRs blocked on discussion (such as RFCs or WG decisions) should be closed
  immediately, since those discussions generally take a long time.
- PRs blocked on other PRs should be closed, unless the blocking PR looks
  like it's going to be merged soon.
- PRs which have already been blocked for two weeks should generally be closed,
  unless there is a clear indication that they will be unblocked soon.

Blocked PRs which have not been closed should be triaged as follows:

PRs inactive for greater than 7 days need to be processed. These can be found by
looking at the "updated X days ago" on GitHub's PR list.

Find the blocking issue from the triage comment and see if it has been resolved.

If it has been resolved, move it back to `S-waiting-on-author` or
`S-waiting-on-review` as appropriate. Add a comment notifying the author or
reviewer that the PR is now unblocked.

If it has not been resolved, remove and re-add the `S-blocked` tag. This resets
the update time so the PR won't be reviewed for another week.

#### [S-blocked-closed PRs][s-blocked-closed]

These never need to be looked at, although if you want you can go through the
PRs and see if any have been unblocked. This label is for PRs which are blocked
and have been closed because it is unlikely that the blocking issue will be
resolved soon.

#### [S-inactive-closed PRs][s-inactive-closed]

These never need to be looked at. PRs which have been closed due inactivity.
This is a terminal state for the time being, primarily oriented towards easing
future work.

## Issue triage

Issue triage is mostly much simpler. After finishing PR triage, go to the [list
of untagged issues] and add tags as you see fit. The following categories
should, ideally, be assigned to each issue:

- At least one `A-` tag. This represents the area of the issue, so an issue
  relating to benchmarks or testing would get A-libtest. If you can't find an
  appropriate tag for the issue, it's possible that creating one is the right
  thing to do. Try to pick just one tag to give, unless you're giving the
  A-diagnostics tag, in which case one more tag is a good idea.
- One, and only one, `C-` tag. This represents that category of the issue.
  - `C-bug`: Bugs. These are things like ICEs or other failures of the compiler
    to do what it's supposed to in a way that breaks correct user code. It's not
    always easy to tell if code is correct, and the compiler broken, though, but
    tend towards assuming it's the compiler's fault: at least, we should give a
    better diagnostic. Note that as of now, I-slow, and I-compile{time,mem} are
    not considered bugs, rather, they are enhancements, since they do not break
    user code.
  - `C-cleanup`: Refactoring and cleanup work within the compiler.
  - `C-enhancement`: Diagnostic improvements, primarily, or other nice to haves,
    but not critical issues. Somewhat implies that this is a minor addition.
  - `C-feature-request`: An addition of an impl is the primary thing here.
    Sometimes minor lang features also qualify, though in general it's likely
    those should be closed in favor of RFCs. It's recommended that triagers
    should close issues in favor of the author opening a thread on internals or
    rust-lang/rfcs for language changes that are more significant than adding an
    impl.
  - `C-feature-accepted`: Feature-requests that a relevant team would like to
    see an implementation for before final judgement is rendered. It's likely
    that such an implementation would be merged, unless breakage (e.g.,
    inference-related) occurs.
  - `C-future-compatibility`: Used for tracking issues for future compatibility
    lints.
  - `C-tracking-issue`: This is used for both feature tracking issues (feature
    gates) and issues which track some question or concern of a team. These are
    maintained on GitHub over internals because GH is a more stable location and
    easier to refer to in the long run.
- At least one `T-` tag. Assign the appropriate team to the issue; sometimes
  this is multiple teams, but usually falls into either dev-tools, compiler, or
  libs. Sometimes the lang team needs to make a decision.
- If necessary, add `I-` tags as you see fit. Particularly, `I-ICE` is the
  dominant tag to be added.
- If applicable, add platform tags (`O-`). It's fine to add more than one.

If an issue has been tagged with an `E-` category tag, such as `E-help-wanted`
and has been taken up by someone, but there has been no activity for 7 days, ask
if they require assistance, and inform them that after 14 days this issue will
be made available to anyone. After 14 days re-add the help tag and deassign them
if necessary.

[list of untagged issues]: https://github.com/rust-lang/rust/issues?utf8=%E2%9C%93&q=is%3Aissue%20is%3Aopen%20sort%3Acreated-asc%20-label%3AC-feature-request%20-label%3AC-enhancement%20-label%3AC-cleanup%20-label%3AC-bug%20-label%3AC-tracking-issue%20-label%3AC-future-compatibility%20-label%3AC-question%20-label%3AC-feature-accepted

## State Of Rust Triage

1. Visit the [State Of Rust] project page. Each card has three pieces of
   information.
   - “Feature” — The name of the feature with a link to the tracking issue.
   - “What’s next?” — What we are waiting on to implement and stabilise the RFC.
   - “Last Update” — The last time this card has been triaged.
2. For each card that you choose to triage:
3. Visit the respective tracking issue, and any related issues that the tracking
   issue is recently mentioned in.
4. If the “What’s next?” on the card does not match what you think the current
   state is, update it with the new information.
5. If the implementation of an RFC has changed since the last update, move it to
   the relevant column.
   - If there are PRs merged that implement the RFC the card would move to
     “Implemented”.
   - If there are only open PRs or the PRs don’t implement the full RFC the card
     would be moved to “Implementation in progress”.
   - If there has been a decision to deprecate the RFC, move that to the
     “Deprecated” column.
6. If there have been no meaningful changes to the RFC within 21 days, ping
   someone for an update on the status of the feature.
   - If there have been PRs implementing the RFC, ping the author(s).
   - If author has not responded within a week, or there are no relevant PRs,
     ping the relevant team.
   - If there is no clear choice for the team that should be doing the
     implementation, please add this to release team meeting notes (which can be
     found in the [#release Discord channel]).
7. Update the date on the “Last update” and move that to the bottom of the
   column.

[state of rust]: https://github.com/rust-lang/rust/projects/8
[#release Discord channel]: https://discord.gg/rust-lang

