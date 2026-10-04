# Trusted Contributors marker team

The project [established][council-proposal] a **`trusted-contributors`** marker team to grant `try` and `perf` permissions ([try builds][try-builds] and [perf runs][perf]) to trusted regular contributors who:

* are not yet part of project teams or don't want to be part of project teams whose membership automatically grants `try` and `perf` permissions, and
* where having access to {`try`, `perf`} permissions allows them to contribute more effectively.

This is intended to be a convenience afforded for known and trusted regular contributors so they don't have to constantly ask for a project member with those permissions to delegate via `@bors`.

## Permissions granted by the `trusted-contributors` marker team

* [`@bors try`][try-builds], and
* [`@rust-timer`][perf] `perf` permissions

## How to apply

Membership addition is based on a **vouch** system, similar to [Dev Desktops][dev-desktop].

* A project member with {`try`, `perf`} permissions may **vouch** for a regular trusted contributor to be added to the `trusted-contributors` marker team.
* The vouching project member may then forward their vouch to the infra team, or include their vouch in a `team` PR proposing to add the vouched contributor to the `trusted-contributors` marker team.
  * [Example PR](https://github.com/rust-lang/team/pull/2800)
* The infra team will then review the addition, which may include a quick review of their interaction history with the project, before deciding to approve or reject the addition.
* If the infra team has reasons to be concerned about the proposed addition, the infra team may close the `team` PR (if it was opened) and communicate the concern to the project member privately.

## Responsible usage

Members are expected to use the project resources and {`try`, `perf`} permissions responsibly.

The project and the infra team reserve the right to remove members from this marker team without prior notice if improper usage of {`try`, `perf`} permissions is observed.

## Auto-alumni policy

Since {`try`, `perf`} permissions are sensitive, we want to minimize the risk from stale membership where inactive members may have their accounts compromised. Therefore, `trusted-contributors` has a stricter [auto-alumni policy][auto-alumni] than regular project teams.

`trusted-contributors` members will be subject to an **auto-alumni policy** where they will be removed from the marker team if they are **inactive for 3 months or more** ("inactive" here refers to interactions with the project, including discussions). The contributor will be notified on the `team` PR removing them from `trusted-contributors` but won't have a 10 day waiting period.

* The contributor can be re-added if the project member (or another project member) vouching for them is happy to continue to vouch for them.

[council-proposal]: https://github.com/rust-lang/leadership-council/issues/325
[try-builds]: https://rustc-dev-guide.rust-lang.org/tests/ci.html#try-builds
[perf]: https://rustc-dev-guide.rust-lang.org/tests/perf.html
[dev-desktop]: ./dev-desktop.md
[auto-alumni]: https://github.com/rust-lang/leadership-council/blob/main/policies/membership/auto-alumni.md#identification
