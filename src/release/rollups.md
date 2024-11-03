# Rollup Procedure

## Background

The Rust project has a policy that every pull request must be tested after merge
before it can be pushed to master. As PR volume increases, this can scale poorly,
especially given the long (~3.5hr) current CI duration for Rust.

Enter rollups! Changes that are small, not performance sensitive, or not platform
dependent are marked with the `rollup` command to bors (`@bors r+ rollup` to
approve a PR and mark it as a rollup, `@bors rollup` to mark a previously approved
PR, `@bors rollup-` to un-mark as a rollup).  'Performing a Rollup' then means
collecting these changes into one PR and merging them all at once. The rollup
command accepts four values: `always`, `maybe`, `iffy`, and `never`. See [the
Rollups section] of the review policies for guidance on what these different
statuses mean.

You can see the list of rollup PRs on Rust's [Homu queue], they are
listed at the bottom of the 'approved' queue with a priority of 'rollup' meaning
they will not be merged by themselves until everything in front of them in the
queue has been merged.

## Making a Rollup

1. Using the interface on [Homu queue], select pull requests and then
   use "rollup" button to make a rollup pull request.

   **Important note**:  consider for addition PRs marked as
   `rollup=always`, `rollup=maybe` and `rollup=iffy`, based on the
   review policies of [the Rollups section].  Be extra careful when
   deciding what to include, in particular on `rollup=maybe` and
   `rollup=iffy` PRs. We should try as much as possible to avoid risking
   to hit regressions (bugs or perf).  Also consider that contributors
   often forget to tag things with rollup=never, when they should have
   done so, so when PRs are not explicitly tagged with rollup, be extra
   careful. Also carefully consider the area of the compiler the PRs touch.
   Two diagnostic PRs may actually conflict with each other, as they both 
   change compiler output, which causes failures in each other's tests, 
   when both of them are merged together in a rollup without causing git merge-conflicts.
   In this case the older PR should be given the privilege to merge first 
   and the newer PR should then be rebased as needed.

2. Run the following command in the pull request thread:

    ```
    @bors r+ rollup=never p=5
    ````
   where 5 is the number of PRs contained in your rollup.
3. If the rollup fails, use the logs rust-log-analyzer
   provides to bisect the failure to a specific PR and
   `@bors r-` the PR. If the PR is running, you need to do `@bors r- retry`. Otherwise,
   your rollup succeeded. If it did, proceed to the next rollup (every now and
   then let `rollup=never/iffy` and toolstate PRs progress).
4. Recreate the rollup without the offending PR starting again from **1.**. 
   There's a link in the rollup PR's body to automatically prefill the rollup UI 
   with the existing PRs (minus any PRs that have been `r-`d)
   Try avoiding adding any additional PR to the current "batch" as this
   unnecessarily increases the chance of test failures.
   Rollups should not overlap, if a PR is already contained in a rollup that is not closed,
   it should not be added to another different rollup at the same time.
   If a rollup fails and you are not sure which PR caused the problem, 
   you may bisect the rollup and split it up into two rollups until the offending PR becomes clear.

## Selecting Pull Requests

The queue is sorted by rollup status. In general, a good rollup contains a bunch of `maybe` (unmarked) PRs, and a large pile of `always` PRs. You can include one or two `iffy` PRs if you are confident that they will pass. 
If you have a lot of time, you can also make a rollup just from `iffy` PRs (if there are enough of them) and weed out the failures one by one.
A rollup should never include `rollup=never` PRs.

The actual absolute size of the rollup can depend based on experience and current length of the queue.
People new to making rollups might start with including 1 `iffy`, 2 `maybe`s, and 4 `always`s. Usually 6-8 PRs per rollup is a good compromise.
There is rarely a need to roll up more than 10 PRs at once (unless there are >30 PRs waiting the queue), keep in mind that we also try to minimize regressions per merge.

Don't try to make mega-rollups (15-20 PRs that merge half or more of the entire queue all at once) to keep the number of perf or bug regressions per merge as low as possible and keep potential regressions [bisectable].

Don't hesitate to downgrade the rollup status of a PR! If your intuition tells you that a `rollup=always` PR has some chances for failures, mark it `rollup=maybe` or better `rollup=iffy`. A lot of the unmarked `maybe` PRs are categorized as such because the reviewer may not have considered rollupability, so it's always worth picking them with a critical eye. Similarly, if a PR causes your rollup to fail, it's worth considering changing its rollup status.

Generally, PRs that touch CI configuration or the bootstrapping process are probably `iffy` or `never` and should be handled with care. On the other hand, PRs that just edit docs are usually `rollup=always`.

Avoid having too many PRs with large diffs or subtree changes in the same rollup. Self-contained submodule changes (such as `miri` updates) on the other hand may be fine to be rolled up with other unrelated PRs.
Also avoid having PRs you suspect will have large perf impacts and mark them as `rollup=never`.

If an `iffy` PR keeps failing in a rollups, it should be marked `never` to prevent it from causing further problems inside unrelated rollups. This will also cause it to bump up in front of all `maybe`s in the queue and the author will get feedback quicker in case of subsequent failures.
It should be noted which runner the PR failed on, to run this runner as a `try-job` job and make sure it succeeds there before another merge is attempted (example on syntax [here]).
In general, if possible, try to test a failed PR via a handful of carefully selected try-jobs instead of having to run the full battery of all 60 runners on if it's likely a PR may fail again.

To not have `never` or `iffy` PRs stuck in the queue indefinitely, it is recommended to alternate between rollup and non-rollup prs, so one `never`, one rollup, one `iffy`, one `rollup`, one `never` etc, until most of the `never`s are merged.
If you are the only person making rollups, you can also leave a couple of `never`/`iffy`s for a time where you know nobody will be doing rollups actively, or for weekends which generally see a lower number of PR approvals.

Try to be fair with rollups: Rollups are a way for things to jump the queue. For `rollup=maybe` PRs, try to include the oldest one (at the top of the section) so that newer PRs aren't jumping the queue over older PRs entirely. You don't have to include every PR older than PRs included in your rollup, but try to include the oldest. Similar to the perspective around `iffy`, it's useful to look at a rollup as a way for other PRs to piggyback on the CI cycle of the oldest PR in queue.
Very old (several months) or very large PRs that are extremely prone to merge conflicts may also be given a slight priority bump (`p=1`) to finally get them out of the queue without having to rebase them repeatedly.
Ultimately, we want to keep the number of regressions per merge at a minimum while also minimizing the amount of time between approval and the final merge of a PR, to avoid unnecessary merge conflicts and rebases.


## Failed rollups
If the rollup has failed, run the `@bors retry` command if the
failure was spurious (e.g. due to a network problem or a timeout).
There may be a matching `CI-spurious-fail-.*` label that you can use to tag the PR as such, to help discover common fail patterns.
If it wasn't spurious, find the offending PR and throw it out by copying a link to the rust-logs-analyzer comment,
and leaving a comment like `Failed in <link_to_comment>, @bors r-`.
In case the log-analyzer does not give any meaningful output, you can directly open the ci-logs (the `(web)` link), find the point where the error was thrown
and directly copy the URL to the respective line in the log output.
Hopefully, the author or reviewer will give feedback to get the PR fixed or confirm that it's not
at fault. The failed rollup PR should then be closed.

Once you've removed the offending PR, recreate your rollup without it (see 1.).
Merge one batch of PRs by throwing out the failures one by one instead of adding new PRs to it, as this may introduce additional points of failure.

Sometimes however, it is hard to find the offending PR. If so, use your intuition
to avoid the PRs that you suspect are the problem and recreate the rollup.
Another strategy is to raise the priority of the PRs you suspect,
mark them as `rollup=never` (or `iffy`) and let bors test them standalone to dismiss
or confirm your hypothesis, or split the rollup into 2 smaller ones until are certain of the failure cause. If a PR was found to be the cause and other PRs were "wrongfully" `iffy`'d, they can of course be reprioritised as `maybe` again.

If a PR in a rollup continues to fail you can run the `@bors rollup=never` command to
never rollup the PR in question.

[Homu queue]: https://bors.rust-lang.org/queue/rust
[the Rollups section]: ../compiler/reviews.md#rollups
[here]: https://github.com/rust-lang/rust/pull/132434#issue-2628063878
[bisectable]: https://rust-lang.github.io/cargo-bisect-rustc/