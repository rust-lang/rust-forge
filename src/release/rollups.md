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
    @bors r+ rollup=never p=5
    ````

3. If the rollup fails, use the logs rust-log-analyzer
   provides to bisect the failure to a specific PR and do
   `@bors r-`. If the PR is running, you need to do `@bors r- retry`. Otherwise,
   your rollup succeeded. If it did, proceed to the next rollup (every now and
   then let `rollup=never` and toolstate PRs progress).
4. Recreate the rollup without the offending PR starting again from **1.**. There's a link in the rollup PR's body to automatically prefill the rollup UI with the existing PRs (minus any PRs that have been `r-`d)

## Selecting Pull Requests

The queue is sorted by rollup status. In general, a good rollup includes one or two `iffy` PRs (if available), a bunch of `maybe` (unmarked) PRs, and a large pile of `always` PRs. A rollup should never include `rollup=never` PRs.

The actual absolute size of the rollup can depend based on experience, people new to making rollups might start with including 1 `iffy`, 4 `maybe`s, and 5 `always`s, but more experienced people might even make a rollup of 1-2 `iffy`s, 8 `maybe`s, and 10 `always`s! Massive rollups are rarely needed, but as your intuition grows you'll get better at judging risk when including PRs in a rollup.

Don't hesitate to downgrade the rollup status of a PR! If your intuition tells you that a `rollup=always` PR has some chances for failures, mark it `rollup=maybe` or `rollup=iffy`. A lot of the unmarked `maybe` PRs are categorized as such because the reviewer may not have considered rollupability, so it's always worth picking them with a critical eye. Similarly, if a PR causes your rollup to fail, it's worth considering changing its rollup status

Generally, PRs, that touch CI configuration or the bootstrapping process are probably `iffy` and should be handled with care. On the other hand, PRs that just edit docs are usually `rollup=always`.

Avoid having too many PRs with large diffs or submodule changes in the same rollup. Also avoid having PRs you suspect will have large perf impacts, and mark them as `rollup=never`.

It's tempting to avoid including `iffy` PRs at all since ideally you want your rollup to succeed. However, it's worth remembering that the job of the PR queue is to _test_ PRs, not to land them. As such, a rollup that fails because of an `iffy` PR is a good thing, since that PR would have to be tested at _some point_ anyway and it would have taken up the same amount of time to test if it never got included in a rollup. One way to look at rollups when it comes to `iffy` PRs is that a rollup is a way for a bunch of other PRs to piggyback on the CI cycle that the `iffy` PR needs anyway. If rollups avoid `iffy` PRs entirely what ends up happening is that these PRs tend to languish in the queue for a long time, which isn't good.

Similarly, make sure to leave some spare CI cycles so that `never` PRs also get a chance! If you're the only person making rollups it's worth letting them run during times you're not paying attention to the queue, but these days there are rollup authors in multiple time zones, so it's often best to just keep an eye on the relative size of the queue and put aside a couple CI cycles for `never` PRs, especially if they pile up.

Try to be fair with rollups: Rollups are a way for things to jump the queue. For `rollup=maybe` PRs, try to include the oldest one (at the top of the section) so that newer PRs aren't jumping the queue over older PRs entirely. You don't have to include every PR older than PRs included in your rollup, but try to include the oldest. Similar to the perspective around `iffy`, it's useful to look at a rollup as a way for other PRs to piggyback on the CI cycle of the oldest PR in queue.


## Failed rollups
If the rollup has failed, run the `@bors retry` command if the
failure was spurious (e.g. due to a network problem or a timeout). If it wasn't spurious,
find the offending PR and throw it out by copying a link to the rust-logs-analyzer comment,
and writing `Failed in <link_to_comment>, @bors r-`. Hopefully,
the author or reviewer will give feedback to get the PR fixed or confirm that it's not
at fault. The failed rollup PR can be closed.

Once you've removed the offending PR, re-create your rollup without it (see 1.).
Sometimes however, it is hard to find the offending PR. If so, use your intuition
to avoid the PRs that you suspect are the problem and recreate the rollup.
Another strategy is to raise the priority of the PRs you suspect,
mark them as `rollup=never` (or `iffy`) and let bors test them standalone to dismiss
or confirm your hypothesis.

If a rollup continues to fail you can run the `@bors rollup=never` command to
never rollup the PR in question.

[Homu queue]: https://bors.rust-lang.org/queue/rust
[the Rollups section]: ../compiler/reviews.md#rollups
