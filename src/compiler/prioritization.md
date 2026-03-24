# Prioritization
It is important that the compiler team can quickly identify priority issues, hence the establishment
of a prioritization process, described below.

## General Process
1. Ascertain the current status of the issue
1. Try progressing the issue if possible (e.g. request updates from the issue author/reviewer)
1. Is there an MCVE for the issue already?
1. Check if it's a regression and label it accordingly (`regression-*` labels)
1. Figure out the area the issue belongs and label it accordingly (`A-*` labels)
1. [Ping notify groups](https://rustc-dev-guide.rust-lang.org/notification-groups/about.html) or
   relevant teams
1. Assign if possible

## Request prioritization for an issue
We generally prioritize regressions and unsound/miscompile issues.

Anyone can request an issue to be prioritized by issuing the following command in a GitHub comment:
```
@rustbot prioritize
```
or if you are a team member, just add the `I-prioritize` label.

Read more [here][requesting-prio] about how to enable issue prioritization in a repository.

[requesting-prio]: ../triagebot/requesting-prioritization.html

## Assigning priority to an issue
To assign a priority, replace the `I-prioritize` label with one of `P-critical`, `P-high`,
`P-medium` or `P-low` and add a succinct comment about the reasoning or link the Zulip discussion
where the issue prioritization occurred, example:

> Assigning priority (discussion on [Zulip](#)).
>
> @rustbot label -I-prioritize +P-XXX

Tip: you can use [Github Saved Replies](https://docs.github.com/get-started/writing-on-github/working-with-saved-replies) to create a template comment.

Priority can also be assigned from Zulip:
```
@**triagebot** assign-prio <issue #> [ critical | high | medium | low | <empty>]
```

Examples:
- Assign high priority to issue 123456:
  ```
  @**triagebot** assign-prio 123456 high
  ```
- Remove priority from issue 123456:
  ```
  @**triagebot** assign-prio 123456
  ```

# Priority Levels
As the compiler team's resources are limited, the primary goal of prioritization is to identify the
most relevant issues to work on, so that the compiler team can focus on what matters the most.

## Labels
Labeling an issue as `I-prioritize` starts the prioritization process, which will end by
removing the `I-prioritize` label and appending one of the 4 labels we will discuss below:

- `P-critical`
- `P-high`
- `P-medium`
- `P-low`

Each of these labels defines a strategy the team will adopt regarding:

- The amount of focus a given issue will receive
- How members of the community can get involved

## P-critical
A `P-critical` is an issue potentially blocking a compiler release (i.e. highly recommended to be
solved before a new compiler release). These issues will be raised at the compiler team's triage
meeting on a weekly basis.

Examples of things we typically judge to be “critical” bugs:

- Regressions where code that used to compile no longer does
  - Mitigating conditions that may lower priority:
    - If the code should never have compiled in the first place (but if the regression affects a
      large number of crates, this may indicate that we need a warning period)
    - If the code in question is theoretical and considered unlikely to exist in the wild, or if
      it only exists in small, unmaintained packages that are not widely used
  - If a regression has been in stable for a release or two (either because we are still awaiting a
    fix, or because the bug had laid dormant i.e. undetected), we typically lower the priority as
    well, because by that time, if the users have not raised a ruckus about the regression, that
    is a sign that it is inherently not a critical issue
- Regressions where code still compiles but does something different than it used to do (dynamic
  semantics have changed)
  - Mitigating conditions that may lower priority:
    - If code uses feature that is explicitly not specified (e.g. `std::vec::Vec` docs state order
      in which it drops its elements is subject to change)
- Feature-gated features accessible without a feature gate
  - Mitigating conditions that may lower priority:
    - If the pattern is *very* unlikely
- Soundness holes with real-world implications
  - Mitigating conditions that may lower priority:
    - Soundness holes that are difficult to trigger
    - Soundness holes that will not affect stable, e.g. if the hole makes use of a gated unstable
      feature.
- Diagnostic regressions where the diagnostic is very common and the situation very confusing
- ICEs for common scenarios or code patterns
  - Mitigating conditions that may lower priority:
    - If the code that triggers the ICE also triggers compilation errors, and those errors are
      emitted before the ICE
    - If the code in question makes use of unstable features, particularly if the ICE requires a
      feature gate

A `P-critical` issue will receive the most attention. It must be assigned one or several people as
soon as possible, and the rest of the team should do their best to help them out if/when applicable.

## P-high
`P-high` issues are issues that need attention from the compiler team, but not to the point that
they need to be discussed at every meeting. They can be `P-critical` issues that have a mitigating
condition as defined above, or important issues that aren't deemed blockers.

Because there are too many `P-high` issues to fit in every compiler meeting, they should rather be
handled asynchronously by the team's prioritization, in order to help them move forward. They can
still occasionally be brought up at meetings when it is deemed necessary.

The effectiveness of the team's prioritization will be a direct consequence of the ability to draw
the line between `P-critical` and `P-high` issues. There shouldn't be too many `P-critical` issues
that compiler meetings become unmanageable, but critical issues shouldn't get lost in the list of
`P-high` issues.

`P-high` issues are issues the teams will mostly work on. We want to make sure they're assigned,
and keep an eye on them. They are routinely reviewed in batches by the compiler team, deciding a
possible priority downgrade.

## P-medium and P-low
`P-medium` refer to issues that aren't a priority for the team, and that will be resolved in the
long run. For example, issues that will be fixed after a specific feature has landed. They are
issues that the team could mentor someone interested in fixing. They will remain in this state
until someone complains, a community member fixes it, or it gets fixed by accident.

`P-low` refer to issues issue that the compiler team doesn't plan to resolve, but are still worth
fixing. Nominate the issue if it's unclear and needs to be discussed.

# Compiler triage
Team Compiler meets every Thursday on [Zulip][zulip-compiler-meetings] to do triaging and talk about
other topics. Feel free to participate, it's open to everyone.

The triage meeting agenda is generated using the prioritization efforts as input, read [here][meeting-agenda] how.

[zulip-compiler-meetings]: https://rust-lang.zulipchat.com/#narrow/channel/238009-t-compiler.2Fmeetings/topic/.5Bweekly.5D.202026-03-05/with/577530236

[meeting-agenda]: ./operations.html#triage-meetings
