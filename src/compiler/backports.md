# Compiler change backports

Sometimes, compiler changes need to be backported to the stable and/or beta channel. Example
motivations may include but are not limited to:

- To fix significant regressions that are present on the beta / stable channels.
- To revert changes that unintentionally cause the compiler to reject / accept new code.
- To revert lints (or changes to their default lint levels) that may be overzealous / too noisy.
- To revert stabilizations due to significant concerns / issues, or accidental stabilizations.

## Nominating for compiler backport review

You can nominate a compiler change for compiler team to consider to backport to **stable** and/or
**beta** channels by applying suitable labels:

- Nominate for **beta** channel backport: [`beta-nominated`]
- Nominate for **stable** channel backport: [`stable-nominated`]

You can combine both labels if the change should be considered to backport to both release channels.

To apply those labels, you can either:

- Directly apply the suitable label(s), or
- Use one of the following [`@rustbot` label invocations](../triagebot/labeling.md):
  - `@rustbot label: +beta-nominated`
  - `@rustbot label: +stable-nominated`
  - `@rustbot label: +beta-nominated +stable-nominated`
- Make sure the PR has a `T-compiler` team label if the backport decision requires compiler review.
  This should typically already be done by [triagebot](../triagebot/autolabels.md) if the PR changes
  compiler code. If this is not the case, you can also use `@rustbot label: +T-compiler` to manually
  apply a compiler team label.

In any case, **please include a nomination message providing context of the change and justifying
the beta and/or stable backport** so that compiler backport reviewers don't have to search for
issues or dig through a bunch of links.

**Backport nominations are not guaranteed be accepted**. Compiler backport reviewers may decide to
reject backport nominations if they determine that:

- The **benefit** of backporting to beta/stable channels **does not sufficiently outweigh the risk
  of introducing *new* regressions** to the beta/stable channels.
- The nominated change is **too complex** (which typically correlates with being too risky) to    
  backport.
- (For beta backport nominations) The nominated change has insufficient time to bake on the beta
  release channel. If the beta compiler candidate is near the end of a release cycle, i.e. meaning
  the beta compiler candidate has very limited (if any) time to bake to give people opportunities to
  test and report new regressions. 
- (For stable backport nominations) The nominated change does not sufficiently justify a stable
  point release on its own, and there are no other accepted stable backport nominations (not
  necessarily compiler stable backport nominations) around the same time.

If a backport nomination unfortunately slides the relevant release channels (e.g. due to
implementation issues), the nomination may be automatically rejected. For beta regressions that slid
to stable, you may need to consider a stable backport nomination instead. This should not generally
happen for critical issues.

## Meaning of the `{beta,stable}-{nominated,accepted}` labels

- [`beta-nominated`], [`stable-nominated`] (only when in conjunction with the [`T-compiler`] team 
  label): a PR is nominated for the beta and/or stable release channel.
- [`beta-accepted`], [`stable-accepted`]: the compiler team decided to accept the beta and/or stable
  backport nomination.

## How are approved backports handled?

Most of the time, [`beta-accepted`] and [`stable-accepted`] labels are applied to PRs that target
the `master` branch. If a `master`-targeting PR is marked `{beta,stable}-accepted`, their backports
will be handled by the [release team](../release/backporting.md).

In rare circumstances, a beta backport PR may need to *directly* target the `beta` branch. In this
case, coordinate with the [release team](../release/README.md) before merging (e.g. by pinging the
release team or opening a [#t-release zulip
thread](https://rust-lang.zulipchat.com/#narrow/channel/241545-t-release)).

## Reviewing compiler backport nominations

> **Note on experimental nature of async backport nominations**
>
> The compiler team is currently experimenting with *asynchronous* backport decisions, but we will
> still track backport nominations in the weekly [compiler triage meeting](./meetings.md).

When a PR has `{beta,stable}-nominated` and [`T-compiler`] labels applied on [rust-lang/rust], a new
backport nomination zulip thread will automatically be created by
[triagebot](../triagebot/zulip-notifications.md) under [#t-compiler/backports] and ping compiler
team members who are subscribed to the backport channel.

Compiler team members will review the beta/stable backport nominations through the zulip threads,
and discuss and vote on the polls created by [triagebot](../triagebot/zulip-notifications.md). If
there are no timely consensus on decision from the asynchronous backport nomination zulip threads,
the [compiler-ops team](../compiler/operations.md) will raise those on the upcoming [compiler triage
meeting](./meetings.md) to make a synchronous decision.

### Considerations for backport decisions

For compiler backport reviewers, here are some **non-exhaustive** considerations that they might
consider in making a backport decision:

- How long does a beta backport have to bake? Consider remaining time available for users to test
  the beta compiler candidate before the next release cycle (note that the actual stable compiler
  artifact will necessarily be built sooner than the actual release!).
- How complex is the nominated compiler change?
- How risky is the nominated compiler change? Can it introduce new regressions to the stable/beta
  channel? Can the "cure" (nominated change) be potentially worse than the "poison" (the stable/beta
  regression)?
- How severe is the regression/issue being fixed? E.g. critical regressions vs stable-to-stable
  regressions that are not very severe, and is already on stable channel for multiple release
  cycles.

At the end of then day, it's a risk vs benefit tradeoff decision. The general guidance is to only
accept beta backports if it's fix for a (beta/stable) regression if the benefit sufficiently
overweighs the risk. For stable backports, this bar should be even higher as we should not and do
not want to encourage frequent stable point releases.

## When and how is a backport decision reached? How long should a backport decision take?

> **Overall idea**
>
> The general idea behind the current async backport review and approval process experiment is that
>
> > async backport will never take longer than waiting for the next meeting.
>
> See [discussions on async backport reviews](https://rust-lang.zulipchat.com/#narrow/channel/131828-t-compiler/topic/Do.20backport.20nominations.20async.3F/near/518255539).

A *asynchronous* beta/stable backport nomination (i.e. the aforementioned async backport nomination
zulip threads under [#t-compiler/backports]) **may** be considered accepted/rejected **if and only
if**:

- There are at least **3** compiler team members who vote for approve/reject (i.e. a "**strong
  decision outcome**"), and there are no outstanding votes in the opposite direction, then the
  backport nomination can be considered to be approved/accepted **by the next compiler triage
  meeting**.
  - Example: if **3** compiler team members vote **approve**, **1** member vote **don't know**, then
    the backport nomination is considered accepted by the next compiler triage meeting.

A *synchronous* beta/stable backport nomination **may** be considered accepted/rejected **if and
only if** (during the weekly triage meeting):

- There are sufficient engagement during the backport decision discussion (**at least 2** compiler
  team members are present, including **at least 1** compiler team lead), and no significant
  disagreements.
- The compiler lead will make the call to accept/reject/postpone the backport nomination.
- A [compiler-ops member](./operations.md) or any compiler team member should leave a comment
  announcing the backport decision and backlink to the triage meeting decision message / async
  backport decision thread and adjust `{beta,stable}-{nominated,accepted}` labels as suitable.

A backport nomination decision for a *merged* (reviewed and approved) compiler change (to `master`
branch) should *generally* be expected to take no longer than until the next weekly synchronous
compiler triage meeting. For *unmerged* (outstanding review concerns / unreviewed) PRs, compiler
backport reviewers *may* decide to postpone to next weekly compiler triage meeting.

In time sensitive cases, compiler leads **may** choose to accepted/reject backport nominations if
there are insufficient engagement on either async/sync avenues.


[rust-lang/rust]: https://github.com/rust-lang/rust
[`beta-nominated`]: https://github.com/rust-lang/rust/labels/beta-nominated
[`beta-accepted`]: https://github.com/rust-lang/rust/labels/beta-accepted
[`stable-nominated`]: https://github.com/rust-lang/rust/labels/stable-nominated
[`stable-accepted`]: https://github.com/rust-lang/rust/labels/stable-accepted
[#t-compiler/backports]: https://rust-lang.zulipchat.com/#narrow/channel/474880-t-compiler.2Fbackports
[`T-compiler`]: https://github.com/rust-lang/rust/labels/T-compiler
