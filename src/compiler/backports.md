# Backports

Sometimes compiler fixes need to be backported to the stable and/or beta channel to fix a
significant regression or to revert unintentional changes that weren't noticed when originally
merged. See also [prioritization](./prioritization.md) for how priority of issues and regressions
are determined.

In case a backport is applied to the stable channel, the Rust project will release a patch release
(for example a 1.87.1 point release could follow the initial stable release 1.87.0).

## Nominating for backport review

You can propose a change be backported by applying labels to the GitHub pull request. Add
`beta-nominated` if the patch should be backported to the **beta** channel, and also
`stable-nominated` if it needs backported to the **stable** channel. Make sure the pull request has
a [`T-compiler`] label as well.

In any case, you should **add a comment when you nominate the pull request for backport** providing
context for the compiler team backport reviewers about why it should be backported.

**Backport nominations are not guaranteed be accepted**. Please refer to the [*Should the backport
be approved*](#should-the-backport-be-approved) section below for the criteria on which backport
nominations may be accepted or rejected.

Beta regressions that slid to the stable channel will need a stable backport nomination (and a
subsequent patch release, if approved).

The compiler team tries to make sure that critical issues (labeled with `P-critical`) do not
progress to the stable release.

## Reviewing compiler backport nominations

When one of [`beta-nominated`] or [`stable-nominated`] label is applied, a new thread is
automatically opened on Zulip in the [#t-compiler/backports] channel. Compiler team members can use
these Zulip threads to cast their vote asynchronously in favor of or raise concerns about
backporting. If you are a compiler team member and want to be notified of these threads, you should
subscribe that zulip channel.

During the weekly triage meeting (happening on [#t-compiler/meetings], see [here](./meetings.md)),
the compiler team will finalize the decision and apply the relevant `{beta,stable}-accepted` label.

### Should the backport be approved?

For compiler backport reviewers, here are some **non-exhaustive** considerations that they might
consider in making a backport decision:

- The nominated change is not merged in time for the stable or beta release channel (e.g. due to
  implementation issues).
- Has the backport enough time to be tested?
  - (For beta backport nominations) The nominated change would be applied too close to the next
    stable compiler release. Merging a backport at this time will mean very limited (if any) time
    for testing.
  - A stable point release will be *immediately* available to all users without any time to bake!
- How complex or risky is the nominated compiler change? Is the risk of the backport introducing new
  regressions potentially worse than the regression or issue it addresses?
- How severe is the regression/issue being fixed?
  - For example stable regressions are not all equal: some are not very severe or are already on
    stable channel since multiple releases.

By default, approved stable backports will cause a new point release to be issued by the [release
team](../release/backporting.md#stable-backporting-in-rust-langrust).

However, the compiler team may approve a stable backport, but additionally indicate to the [release
team](../release/backporting.md) that the nomination does not justify a stable point release *on its
own*. In this case, the release team will consider other approved stable backport candidates and how
serious those are in conjunction with this candidate, to finalize a decision on whether to execute a
stable point release.

## How are approved backports handled?

The [release team](../release/README.md) (`T-release`) will handle the backport at the end of the
current development cycle (see [release backporting](../release/backporting.md)). If a beta backport
nomination is approved too late, the release team may be unable to backport the change.

Most of the time, accepted backports target the `master` branch. In rare circumstances, a beta
backport may need to *directly* target the `beta` branch. In this case, coordinate with the release
team before merging, by opening a new thread on the Zulip [#t-release
channel](https://rust-lang.zulipchat.com/#narrow/channel/241545-t-release)).

For complicated backports, the release team may ask the patch author for assistance.


[`beta-nominated`]: https://github.com/rust-lang/rust/labels/beta-nominated
[`beta-accepted`]: https://github.com/rust-lang/rust/labels/beta-accepted
[`stable-nominated`]: https://github.com/rust-lang/rust/labels/stable-nominated
[`stable-accepted`]: https://github.com/rust-lang/rust/labels/stable-accepted
[#t-compiler/backports]:
    https://rust-lang.zulipchat.com/#narrow/channel/474880-t-compiler.2Fbackports
[#t-compiler/meetings]: https://rust-lang.zulipchat.com/#narrow/channel/238009-t-compiler.2Fmeetings
[`T-compiler`]: https://github.com/rust-lang/rust/labels/T-compiler
