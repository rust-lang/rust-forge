# Fixing inconsistencies in the bors queue

bors queue page: <https://bors.rust-lang.org/queue/rust>.

<div class="warning">

**WARNING**: You should only do this if you have bors `r+` permissions on the
rust-lang/rust repo. Please do not synchronize if you do not have `r+` permissions
even if you have write access to the repo, as you will be unable to perform the
required cleanup steps.

This is a **destructive** operation. If someone syncs, they need to
baby-sit the queue for around 45 minutes: around 30 minutes to wait for PRs to
be recollected, and 15 minutes after that to kick out PRs that should not be in
the tree.

**DO NOT CLICK THIS BUTTON IF YOU ARE NOT ABLE TO HANDLE THE CLEANUP.**

</div>

Sometimes you have to do a bors queue sync for various reasons. This is not
trivial and requires you to be very careful, as otherwise we may accidentally
merge PRs to `master` (or even `beta`) that should not have been merged
otherwise.

## Steps

### Step 0: Announce your intention

Let T-infra (and other reviewers) know that you plan to close the tree. Open a
new [T-infra zulip
thread](https://rust-lang.zulipchat.com/#narrow/channel/242791-t-infra) to let
other contributors know about the bors queue resync.

### Step 1: Close the tree

Find a PR that's currently being tested (or any open PR in the queue really if
the queue is *really* messed up).

Issue `@bors treeclosed=1000` along with some brief explanation for why you are
closing the tree so other reviewers (especially people doing rollups) have some
context.

Example:

```text
Closing the tree due to a resync.
cc <https://rust-lang.zulipchat.com/#narrow/channel/242791-t-infra/topic/try.20jobs.20not.20kicking.20off>.

@bors treeclosed=1000
```

### Step 2: Click "synchronize" button

As a courtesy, you can record which PRs had try jobs starting on them. After a
sync, the distinction between regular jobs and try jobs will be lost, so you'll
have to kick out all "pending" PRs.

Click the "synchronize" button in the [bors queue page][bors-queue]. Then,
immediately start performing the next step.

### Step 3: Kick out all actively tested PRs

Find *all* actively tested PRs. This includes both "auto" builds (or full CI)
which show up as "pending", or "try" builds which show up as "pending (try)"
(but sometimes bors forget the distinction and try jobs can show up as "pending"
too).

On each "pending" or "pending (try)" PR, write:

```text
@bors retry r- (sync)
```

to suspend the current job and take it out of the queue (bors can confuse try
jobs with full CI jobs).

### Step 4: Wait

Wait for around **30-45 minutes** to allow bors to recollect all the PRs. **Do
not** reopen the tree beforehand, as it will cause bors to have an inconsistent
view of the PRs, which will lead to unspecified behavior.

### Step 5: Kick out ineligible PRs

Check "approved" PRs in the queue. Some of them will actually not be eligible
for merge, due to reasons such as:

- Merge conflicts
- Significant changes since last review
- Never has been approved

But sometimes bors forget these distinction. You'll need to manually visit each
of the "approved" PRs and check their eligibility.

For "approved" PRs that are not actually eligible, you should kick them out of
the queue via `@bors r-`. Prefer to be cautious if you are not sure, and
unapprove the PR in case of ambiguity.

```text
@bors r- (sync)
```

### Step 6: Double-check approved PRs

Do another review pass of "approved" PRs in the queue, to make sure all approved
PRs are actually eligible for merge.

### Step 7: Re-open the tree on the same PR where you closed the tree

Reopen the tree on the same PR that you issued the `treeclosed` command with

```text
@bors treeclosed-
```

Closely monitor bors' behavior for around 5 minutes, to ensure that bors is
correctly testing a PR that's eligible for merge. Update the relevant T-infra
zulip thread as suitable.

### Step 8: Re-queue try jobs

In Step 2, if you had to kick out try jobs, you can requeue the try jobs on the
PRs that previously had try jobs started on them.

Use the normal try-job command:

```text
@bors try
```

and not `@bors retry`.

### Step 9: Edit your `treeclose` commands to prevent bors from picking them up

Edit the `@bors treeclosed=xxxx` command and `@bors treeclosed-` command like

```text
~~@/bors treeclosed=xxxx~~

EDIT(ferris): edited to prevent bors from picking up command in a future sync
```

AFTER the tree has been reopened to prevent bors from picking them up in a
future sync.

[bors-queue]: https://bors.rust-lang.org/queue/rust
