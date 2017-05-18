---
layout: default
title: PR Triage Procedure
---

Status tag meanings:

 - [S-waiting-on-review]: Review is incomplete
 - [S-waiting-on-author]: Author needs to make changes to address reviewer comments, or merge
   conflicts/test failures are present. This also covers more obscure cases, like a PR being blocked
   on another, or waiting for a crater run -- it is the author's responsibility to push the PR
   forward.
 - [S-waiting-on-team]: A T- label is marked, and team has been cc-ed for feedback.
 - [S-waiting-on-bors]: Currently approved, waiting to merge.

[S-waiting-on-review]: https://github.com/rust-lang/rust/pulls?q=is%3Aopen+is%3Apr+sort%3Aupdated-asc+label%3AS-waiting-on-review
[S-waiting-on-author]: https://github.com/rust-lang/rust/pulls?q=is%3Aopen+is%3Apr+sort%3Aupdated-asc+label%3AS-waiting-on-author
[S-waiting-on-team]: https://github.com/rust-lang/rust/pulls?q=is%3Aopen+is%3Apr+label%3AS-waiting-on-team+sort%3Aupdated-asc
[S-waiting-on-bors]: https://github.com/rust-lang/rust/pulls?q=is%3Aopen+is%3Apr+label%3AS-waiting-on-bors+sort%3Aupdated-asc

# Procedure:

## Unlabeled PRs

All [unlabeled PRs] should be processed. The steps below are not mutually exclusive, any number of
them may apply.

When there is **no assignee**, because highfive flaked, figure out the responsible team and randomly
choose a reviewer from the team by manually assigning in GitHub.

When **no review has happened**, if the PR is a work in progress (e.g., test failures, merge
conflict) mark S-waiting-on-author. Otherwise, mark S-waiting-on-review. If no human has checked in
yet and you don't recognise the submitter as a regular contributor, leave a comment saying something
like "Thanks for the PR! We’ll periodically check in on it to
make sure that @reviewer or someone else from the team reviews it soon."

At this point, all PRs must have a tag applied.

[unlabeled PRs]: https://github.com/rust-lang/rust/pulls?utf8=%E2%9C%93&q=is%3Aopen%20is%3Apr%20sort%3Aupdated-asc%20-label%3AS-waiting-on-author%20-label%3AS-waiting-on-team%20-label%3AS-waiting-on-bors%20-label%3AS-waiting-on-team%20-label%3AS-waiting-on-review%20

## [S-waiting-on-author PRs]

PRs with greater than 3 days of inactivity need to be processed. These can be found by looking at
the "updated X days ago" on GitHub's PR list.

First, ensure that the status tag matches the current state of the PR. Change the tag if necessary,
and apply the procedure for the new tag.

Then, if the author hasn't responded for more than 7 days to a request for changes or a status
update, ping the author on GitHub asking for an update.

If the author's been unresponsive for more than 14 days, close the PR due to inactivity and ask the
author to reopen when they have a chance to make the necessary changes. Make sure to thank the
author for the changes.

[S-waiting-on-author PRs]: https://github.com/rust-lang/rust/pulls?q=is%3Aopen+is%3Apr+sort%3Aupdated-asc+label%3AS-waiting-on-author


## [S-waiting-on-review PRs]

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

If the **review is incomplete**:
 - If more than 3 days since last reviewer comment, ping the reviewer on IRC and on GitHub.
 - If more than 6 days since the last reviewer comment, also ping the subteam on GitHub.

[S-waiting-on-review PRs]: https://github.com/rust-lang/rust/pulls?q=is%3Aopen+is%3Apr+sort%3Aupdated-asc+label%3AS-waiting-on-review


## [S-waiting-on-team PRs]

All PRs should be processed. First, ensure that the status tag matches the current state of the PR.
Change the tag if necessary, and apply the procedure for the new tag now. Verify that there is a T- tag
for all PRs that remain in this category.

[S-waiting-on-team PRs]: https://github.com/rust-lang/rust/pulls?q=is%3Aopen+is%3Apr+label%3AS-waiting-on-team+sort%3Aupdated-asc

## [S-waiting-on-bors PRs]

All PRs should be processed. First, ensure that the status tag matches the current state of the PR.
Change the tag if necessary, and apply the procedure for the new tag now.

[S-waiting-on-bors PRs]: https://github.com/rust-lang/rust/pulls?q=is%3Aopen+is%3Apr+label%3AS-waiting-on-bors+sort%3Aupdated-asc


## Updating the Spreadsheet

Finally, once all steps are complete, go back through and get counts for each PR status tag. Log
these onto [the spreadsheet]. Verify that the total reported in the spreadsheet corresponds to the
total number of PRs currently open.

[the spreadsheet]: https://docs.google.com/spreadsheets/d/1aBfKT9j4lwpDQePRggRCy7zqhv46hCtRvTGGC9bPSn4/edit
