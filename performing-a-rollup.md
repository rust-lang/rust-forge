---
layout: default
title: Performing a Rollup
---

# Rollups

## Background

The Rust project has a policy that every PR must be tested after merge before it can
be pushed to master. As PR volume increases this can scale poorly, especially given
the long (>2hr) current CI duration for Rust.

Enter rollups - low risk changes (often doc fixes or other non-functional changes)
are marked with the `rollup` command to bors (`@bors r+ rollup` to approve a PR
and mark as a rollup, `@bors rollup` to mark a previously approved PR, `@bors rollup-`
to unmark as a rollup).
'Performing a Rollup' then means collecting these changes into one PR and merging
them all at once.

You can see the list of rollup PRs on https://buildbot2.rust-lang.org/homu/queue/rust -
they are listed at the bottom of the 'approved' queue, with a priority of 'rollup',
meaning they will not be merged by themselves until everything in front of them in the
queue has been merged.

## Procedure

Prerequisites: `r+` permission on the Rust repo, a fork of `rust-lang/rust`
under your GitHub username.

 - Visit https://buildbot2.rust-lang.org/homu/queue/rust.
 - If there are >= 10 approved rollup priority PRs, you should probably create a rollup.
 - Notify the #rust-infra IRC channel.
 - Press the 'Create a rollup' button near the top left of the queue status page.
 - (first time: Authorise the github permissions)
 - Wait for a minute while the rollup is created on the `rollup` branch of your repo
   and the PR is submitted for you.
 - Comment with `@bors r+ p=10`. This will put the PR above any 'normal' (`p=0`) or
   'do this soon' (`p=1`) PRs, but below 'urgent' PRs.
 - If the PR currently being tested is in the rollup, comment on the it with
   `@bors retry`. This will prompt bors to stop the current test, allowing it
   to pick up the new top of the queue - likely the rollup.

## Failed rollup

Close the PR, figure out if the failure was spurious. If so, create a new PR,
if not, find the possible candidate PR(s) and unmark it (them) both as rollup
and as being reviewed with `@bors rollup- r-`, also commenting on the PR with the
error. Hopefully the author or a reviewer will give feedback to get the PR fixed
or confirm that it's not at fault.
