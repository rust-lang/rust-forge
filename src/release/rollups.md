# Rollup Procedure

## Background

The Rust project has a policy that every pull request must be tested after merge
before it can be pushed to master. As PR volume increases this can scale poorly,
especially given the long (~3.5hr) current CI duration for Rust.

Enter rollups! Changes that are small, not performance sensitive, or not platform
dependent are marked with the `rollup` command to bors (`@bors r+ rollup` to
approve a PR and mark as a rollup, `@bors rollup` to mark a previously approved
PR, `@bors rollup-` to un-mark as a rollup).  'Performing a Rollup' then means
collecting these changes into one PR and merging them all at once. The rollup
command accepts four values `always`, `maybe`, `iffy`, and `never`. See [the
Rollups section] of the review policies for guidance on what these different
statuses mean.

You can see the list of rollup PRs on Rust's [Homu queue], they are
listed at the bottom of the 'approved' queue with a priority of 'rollup' meaning
they will not be merged by themselves until everything in front of them in the
queue has been merged.

## Making a Rollup

1. Using the interface on [Homu queue], select pull requests and then
   use "rollup" button to make a rollup pull request. (The text about
   fairness can be ignored.)
   **Important note**:  consider for addition PRs marked as
   `rollup=always`, `rollup=maybe` and `rollup=iffy`, based on the
   review policies of [the Rollups section].  Be extra careful when
   deciding what to include, in particular on `rollup=maybe` and
   `rollup=iffy` PRs. We should try as much as possible to avoid risking
   and hit regressions (bugs or perf).  Also consider that contributors
   often forget to tag things with rollup=never, when they should have
   done so, so when PRs are not explicitly tagged with rollup, be extra
   careful.

2. Run the following command in the pull request thread:

    ```
    @bors r+ rollup=never p=<NUMBER_OF_PRS_IN_ROLLUP>
    ````

3. If the rollup fails, use the logs rust-highfive (really it is
   rust-log-analyzer) provides to bisect the failure to a specific PR and do
   `@bors r-`. If the PR is running, you need to do `@bors r- retry`. Otherwise,
   your rollup succeeded. If it did, proceed to the next rollup (every now and
   then let `rollup=never` and toolstate PRs progress).
4. Recreate the rollup without the offending PR starting again from **1.**

## Selecting Pull Requests

This is something you will learn to improve over time. Some basic tips include
(from obvious to less):

1. Avoid `rollup=never` PRs (these are red in the interface).
2. Include all PRs marked with `rollup=always` (these are green). Try to check
   if some of the pull requests in this list shouldn't be rolled up — in the
   interest of time you can do so sometimes after you've made the rollup PR.
3. Avoid pull requests that touch the CI configuration or bootstrap.
    (Unless the CI PRs have been marked as `rollup`. -- see 2.)
4. Avoid having too many large diff pull requests in the same rollup.
5. Avoid having too many submodule updates in the same rollup (especially LLVM).
    (Updating LLVM frequently forces most devs to rebuild LLVM which is not fun.)
6. Do not include toolstate PRs like those fixing Miri, Clippy, etc.
7. Do include docs PRs (they should hopefully be marked as green).

## Failed rollups
If the rollup has failed, run the `@bors retry` command if the
failure was spurious (e.g. due to a network problem or a timeout). If it wasn't spurious,
find the offending PR and throw it out by copying a link to the rust-highfive comment,
and writing `Failed in <link_to_comment>, @bors r-`. Hopefully,
the author or reviewer will give feedback to get the PR fixed or confirm that it's not
at fault. The failed rollup PR can be closed.

Once you've removed the offending PR, re-create your rollup without it (see 1.).
Sometimes however, it is hard to find the offending PR. If so, use your intuition
to avoid the PRs that you suspect are the problem and recreate the rollup.
Another strategy is to raise the priority of the PRs you suspect,
mark them as `rollup=never` and let bors test them standalone to dismiss
or confirm your hypothesis.

If a rollup continues to fail you can run the `@bors rollup=never` command to
never rollup the PR in question.

[Homu queue]: https://bors.rust-lang.org/queue/rust
[the Rollups section]: ../compiler/reviews.md#rollups
