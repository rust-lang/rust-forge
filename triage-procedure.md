---
layout: default
title: Triage Procedure
---

# PR triage

## Status tag meanings:

 - [S-waiting-on-author] - Author needs to make changes to address reviewer comments, or merge
   conflicts/test failures are present. This also covers more obscure cases, like a PR being blocked
   on another, or waiting for a crater run -- it is the author's responsibility to push the PR
   forward.
 - [S-waiting-on-review] - Review is incomplete
 - [S-waiting-on-team] - A T- label is marked, and team has been cc-ed for feedback.
 - [S-waiting-on-bors] - Currently approved, waiting to merge.
 - [S-waiting-on-crater] - Waiting to see what the impact the PR will have on the ecosystem
 - [S-waiting-on-bikeshed] - Waiting on the consensus over a minor detail
 - [S-blocked] - Waiting for another PR to be merged or for discussion to be resolved
 - [S-blocked-closed] - Closed because resolving the block is expected to take a long time
 - [S-inactive-closed] - Closed due to inactivity.

[S-waiting-on-author]: https://github.com/rust-lang/rust/pulls?q=is%3Aopen+is%3Apr+sort%3Aupdated-asc+label%3AS-waiting-on-author
[S-waiting-on-review]: https://github.com/rust-lang/rust/pulls?q=is%3Aopen+is%3Apr+sort%3Aupdated-asc+label%3AS-waiting-on-review
[S-waiting-on-team]: https://github.com/rust-lang/rust/pulls?q=is%3Aopen+is%3Apr+label%3AS-waiting-on-team+sort%3Aupdated-desc
[S-waiting-on-bors]: https://github.com/rust-lang/rust/pulls?q=is%3Aopen+is%3Apr+label%3AS-waiting-on-bors+sort%3Aupdated-asc
[S-waiting-on-crater]: https://github.com/rust-lang/rust/pulls?q=is%3Aopen+is%3Apr+label%3AS-waiting-on-crater+sort%3Aupdated-asc
[S-waiting-on-bikeshed]: https://github.com/rust-lang/rust/pulls?q=is%3Aopen+is%3Apr+label%3AS-waiting-on-bikeshed+sort%3Aupdated-asc
[S-blocked]: https://github.com/rust-lang/rust/pulls?q=is%3Aopen+is%3Apr+label%3AS-blocked+sort%3Aupdated-asc
[S-blocked-closed]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+label%3AS-blocked-closed+sort%3Aupdated-asc
[S-inactive-closed]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+label%3AS-inactive-closed+sort%3Aupdated-asc

## Procedure:

*Note:* When you are pinging people in triage comments, you should mention that you are doing triage in the comment you post. For example, start your comments with something like "Ping from triage ..."."

### [Unlabeled PRs]

All unlabeled PRs should be processed. The steps below are not mutually exclusive, any number of
them may apply.

When there is **no assignee**, because highfive flaked, figure out the responsible team and randomly
choose a reviewer from the team by manually assigning in GitHub.

When **no review has happened**, if the PR is a work in progress (e.g., test failures, merge
conflict) mark S-waiting-on-author. Otherwise, mark S-waiting-on-review. If no human has checked in
yet and you don't recognise the submitter as a regular contributor, leave a comment saying something
like "Thanks for the PR! We’ll periodically check in on it to
make sure that @reviewer or someone else from the team reviews it soon."

At this point, all PRs must have a tag applied.

[Unlabeled PRs]: https://github.com/rust-lang/rust/pulls?utf8=%E2%9C%93&q=is%3Aopen%20is%3Apr%20sort%3Aupdated-asc%20-label%3AS-waiting-on-author%20-label%3AS-waiting-on-team%20-label%3AS-waiting-on-bors%20-label%3AS-waiting-on-crater%20-label%3AS-waiting-on-team%20-label%3AS-waiting-on-review%20-label%3AS-blocked%20-label%3AS-waiting-on-bikeshed

### [S-waiting-on-author PRs][S-waiting-on-author]

PRs with greater than 3 days of inactivity need to be processed. These can be found by looking at
the "updated X days ago" on GitHub's PR list.

First, ensure that the status tag matches the current state of the PR. Change the tag if necessary,
and apply the procedure for the new tag.

Then, if the author hasn't responded for more than 7 days to a request for changes or a status
update, ping the author on GitHub asking for an update. If they've given advance warning that they
won't be able to address comments for a period of time, allow for that.

If the author's been unresponsive for more than 14 days, close the PR due to inactivity and ask the
author to reopen when they have a chance to make the necessary changes. Make sure to thank the
author for the changes. Also tag the PR with S-inactive-closed.

If there has been no meaningful updates after two triage updates, with no meaningful being defined
as no commits or no status updates that show progress (i.e. "Soon TM"). The PR should be closed
due to prolonged inactivity and ask the author to reopen when they have a chance to make
the necessary changes. Make sure to thank the author for the changes, and warn them not to push to
the PR while it is closed as GitHub will prevent the PR from being reopened. Make sure to also tag
the PR with S-inactive-closed.

*TIP*: if an author is on holiday and you know they won't have a chance to come to a PR for a while,
you can 'bump' the PR by removing and readding the tag (note that removing/readding requires
clicking off the tag selection dropdown between the two actions).

If the PR is blocked on another PR, issue, or some kind of discussion, add a comment clearly identifying what is blocking the PR (something like "This PR appears to be blocked on
#12345") and change the state to S-blocked. Follow the instruction for S-blocked to determine whether you should also close the PR.

### [S-waiting-on-review PRs][S-waiting-on-review]

PRs with greater than 3 days of inactivity need to be processed. These can be found by looking at
the "updated X days ago" on GitHub's PR list.

First, ensure that the status tag matches the current state of the PR. Change the tag if necessary,
and apply the procedure for the new tag.

If there are **no comments from the reviewer**:

 - If the PR is more than 3 days old, ping the reviewer on IRC and on GitHub, noting that you've
   pinged on IRC. If the reviewer is not on IRC, then note in the GitHub comment that IRC ping
   wasn't possible.
 - If the PR is more than 6 days old, in addition to the above steps, ping the subteam on GitHub and
   ask for a new reviewer.
 - If the PR is more than 9 days old, in addition to the above steps, change the tag to S-waiting-on-team

If the **review is incomplete**:

 - If more than 3 days since the last reviewer comment, ping the reviewer on IRC and on GitHub.
 - If more than 6 days since the last reviewer comment, also ping the subteam on GitHub.
 - If more than 9 days since the last reviewer comment, also change the tag to S-waiting-on-team

If the PR is blocked on another PR, add a comment clearly identifying
the blocking PR (something like "This PR appears to be blocked on
#12345") and change the state to S-blocked.

### [S-waiting-on-team PRs][S-waiting-on-team]

PRs active within the last 4 days or inactive for greater than 2 weeks need to be processed.
These can be found by looking at the "updated X days ago" on GitHub's PR list.

First, ensure that the status tag matches the current state of the PR. Change the tag if necessary,
and apply the procedure for the new tag now. Verify that there is a T- tag
for all PRs that remain in this category.

If the PR has been inactive for greater than 2 weeks, add the `I-nominated` label and ping the team,
requesting a new assignee or other appropriate action.

If there has been recent activity, the team might have taken some action meaning the state has
changed but the label has not yet been updated. Therefore, we also check the most recent ones.

### [S-waiting-on-bors PRs][S-waiting-on-bors]

All PRs should be processed. First, ensure that the status tag matches the current state of the PR.
Change the tag if necessary, and apply the procedure for the new tag now.

### [S-waiting-on-crater PRs][S-waiting-on-crater]

All PRs should be processed.

If the PR has been active in the last three days, make sure it's present on the crater
[spreadsheet](https://docs.google.com/spreadsheets/d/1VPi_7ErvvX76fa3VqvQ3YnQmDk3bS7fYnkzvApIWkKo/edit#gid=0).
Fill in the link to the PR and set status as "Pending".

If crater has been run and results include failures, change the tag to S-waiting-on-review for
the reviewer to be responsible for deciding what should be done with the information provided by
the failures.

If crater has been run and the results do not include failures, change the tag to
S-waiting-on-review for the reviewer to take one last look and approve.

If crater has not been run and it has been more than 3 days since a crater run was requested, ping
the last three distinct listed people on the spreadsheet in the infra irc channel and request a crater run.

If crater has been started (the person starting should leave a comment) and it has
been more than 5 days since an update, ping the person starting the run on IRC and GitHub.

### [S-waiting-on-bikeshed][S-waiting-on-bikeshed]

PRs inactive for greater than 7 days need to be processed. These can
be found by looking at the "updated X days ago" on GitHub's PR list.

Find the source of the discussion and see if it has been resolved.

If it has been resolved, move it back to S-waiting-on-author or
S-waiting-on-review as appropriate. Add a comment notifying the author
or reviewer that the PR is now unblocked.

If it has not been resolved, remove and re-add the S-waiting-on-bikeshed tag.
This resets the update time so the PR won't be reviewed for another week.

### [S-blocked PRs][S-blocked]

Blocked PRs can remain blocked for a long time. To avoid needlessly re-triaging them, they should be closed if the blocking issue is unlikely to be resolved soon. If you close a blocked PR, apply the S-blocked-closed label and invite the author to re-open the PR once the issue has been resolved. If you feel uncomfortable just closing the PR, feel free to link to this document. As a rule of thumb, consider these guidelines:

* PRs blocked on discussion (such as RFCs or WG decisions) should be closed immediately, since those discussions generally take a long time.
* PRs blocked on other PRs can be left open, unless the blocking PR doesn't look like it's going to be merged soon.
* PRs which have already been blocked for two weeks should generally be closed, unless there is a clear indication that they will be unblocked soon.

Blocked PRs which have not been closed should be triaged as follows:

PRs inactive for greater than 7 days need to be processed. These can
be found by looking at the "updated X days ago" on GitHub's PR list.

Find the blocking issue from the triage comment and see if it has been
resolved.

If it has been resolved, move it back to S-waiting-on-author or
S-waiting-on-review as appropriate. Add a comment notifying the author
or reviewer that the PR is now unblocked.

If it has not been resolved, remove and re-add the S-blocked tag. This
resets the update time so the PR won't be reviewed for another week.

### [S-blocked-closed PRs][S-blocked-closed]

These never need to be looked at, although if you want you can go through the PRs and see if any have been unblocked. This label is for PRs which are blocked and have been closed because it is unlikely that the blocking issue will be resolved soon.

### [S-inactive-closed PRs][S-inactive-closed]

These never need to be looked at. PRs which have been closed due inactivity. This is a terminal
state for the time being, primarily oriented towards easing future work.

# Issue triage

Issue triage is mostly much simpler. After finishing PR triage, go to the [list of untagged issues]
and add tags as you see fit. The following categories should, ideally, be assigned to each issue:

 - At least one A- tag. This represents the area of the issue, so an issue relating to benchmarks or
   testing would get A-libtest. If you can't find an appropriate tag for the issue, it's possible
   that creating one is the right thing to do. Try to pick just one tag to give, unless you're
   giving the A-diagnostics tag, in which case one more tag is a good idea.
 - One, and only one, C- tag. This represents that category of the issue.
    - C-bug: Bugs. These are things like ICEs or other failures of the compiler to do what it's
      supposed to in a way that breaks correct user code. It's not always easy to tell if code is
      correct, and the compiler broken, though, but tend towards assuming it's the compiler's fault:
      at least, we should give a better diagnostic. Note that as of now, I-slow, and
      I-compile{time,mem} are not considered bugs, rather, they are enhancements, since they do not
      break user code.
    - C-cleanup: Refactoring and cleanup work within the compiler.
    - C-enhancement: Diagnostic improvements, primarily, or other nice to haves, but not critical
      issues. Somewhat implies that this is a minor addition.
    - C-feature-request: An addition of an impl is the primary thing here. Sometimes minor lang
      features also qualify, though in general it's likely those should be closed in favor of RFCs.
      It's recommended that triagers should close issues in favor of the author opening a thread on
      internals or rust-lang/rfcs for language changes that are more significant than adding an
      impl.
    - C-feature-accepted: Feature-requests that a relevant team would like to see an implementation
      for before final judgement is rendered. It's likely that such an implementation would be
      merged, unless breakage (e.g., inference-related) occurs.
    - C-future-compatibility: Used for tracking issues for future compatibility lints.
    - C-tracking-issue: This is used for both feature tracking issues (feature gates) and issues
      which track some question or concern of a team. These are maintained on GitHub over internals
      because GH is a more stable location and easier to refer to in the long run.
 - At least one T- tag. Assign the appropriate team to the issue; sometimes this is multiple teams,
   but usually falls into either dev-tools, compiler, or libs. Sometimes the lang team needs to make
   a decision.
 - If necessary, add I- tags as you see fit. Particularly, I-ICE is the dominant tag to be added.
 - If applicable, add platform tags (O-). It's fine to add more than one.

If an issue has been tagged with an `E-` category tag, such as `E-help-wanted`
and has been taken up by someone, but there has been no activity for 7 days,
ask if they require assistance, and inform them that after 14 days this issue
will be made available to anyone. After 14 days re-add the help tag and
deassign them if necessary.

[list of untagged issues]: https://github.com/rust-lang/rust/issues?utf8=%E2%9C%93&q=is%3Aissue%20is%3Aopen%20sort%3Acreated-asc%20-label%3AC-feature-request%20-label%3AC-enhancement%20-label%3AC-cleanup%20-label%3AC-bug%20-label%3AC-tracking-issue%20-label%3AC-future-compatibility%20-label%3AC-question%20-label%3AC-feature-accepted
