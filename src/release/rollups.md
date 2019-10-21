# Rollup Procedure

## Background

The Rust project has a policy that every pull request must be tested after merge
before it can be pushed to master. As PR volume increases this can scale poorly,
especially given the long (>2hr) current CI duration for Rust.

Enter rollups - low risk changes (often doc fixes or other non-functional
changes) are marked with the `rollup` command to bors (`@bors r+ rollup` to
approve a PR and mark as a rollup, `@bors rollup` to mark a previously approved
PR, `@bors rollup-` to unmark as a rollup).  'Performing a Rollup' then means
collecting these changes into one PR and merging them all at once.

You can see the list of rollup PRs on Rust's [Homu queue], they are
listed at the bottom of the 'approved' queue with a priority of 'rollup' meaning
they will not be merged by themselves until everything in front of them in the
queue has been merged.

## Making a Rollup

1. Using the interface on [Homu queue], select a few pull requests and then use
   "rollup" button to make one.
2. Use the `@bors r+ rollup=never p=<NUMBER_OF_PRS_IN_ROLLUP>` command in the
   pull request thread.
3. Mark the pull request with the label `rollup`.
4. If the rollup fails, use the logs rust-highfive (really it is
   rust-log-analyzer) provides to bisect the failure to a specific PR and do
   `@bors r-`. If the PR is running, you need to do `@bors r-` retry. Otherwise,
   your rollup succeeded. If it did, proceed to the next rollup (every now and then let `rollup=never`
   and toolstate PRs progess).
5. Recreate the rollup without the offending PR starting again from **1.**

## Selecting Pull Requests

This is something you will learn to improve over time. Some basic tips include
(from obvious to less):

1. Avoid `rollup=never` PRs (these are red in the interface).
2. Include all PRs marked with `rollup=always` (these are green). Try to check
   if some of the pull requests in this list shouldn't be rolled up — in the
   interest of time you can do so sometimes after you've made the rollup PR.
3. Avoid pull requests that touch the CI configuration or bootstrap.
4. Avoid having too many large diff pull requests in the same rollup.
5. Avoid having too many submodule updates in the same rollup (especially LLVM).
6. Do not include toolstate PRs like those fixing Miri, Clippy, etc.
7. Do include docs PRs (they should hopefully be marked as green).

## Failed rollups
If the rollup has failed, run the `@bors retry` command figure out if the
failure was spurious. If so, create a new PR, if not, find the possible
candidate PR(s) and unmark it (them) both as rollup and as being reviewed with
`@bors rollup- r-`, also commenting on the PR with the error. Hopefully the
author or a reviewer will give feedback to get the PR fixed or confirm that it's
not at fault.

If a rollup continues to fail you can run the `@bors rollup=never` command to
never rollup the PR in question.

[Homu queue]: https://buildbot2.rust-lang.org/homu/queue/rust
