# Priority levels

As the compiler team's resources are limited, the prioritization working group's main goal is to identify the most relevant issues to work on, so that the compiler team can focus on what matters the most.

## Words used in this document:

`issue` refers to bugs and feature requests that are nominated for prioritization, by flagging the `I-Prioritize` label as described below.

This document will define what each label means, and what strategy for each label will be used.

## Labels

Labeling an issue as `I-Prioritize` starts the prioritization process, which will end by removing the `I-Prioritize` label and appending one of the 4 labels we will discuss below:

- P-critical
- P-high
- P-Medium
- P-Low

Each of these labels defines a strategy the team will adopt regarding:

- The amount of focus a given issue will receive
- How members of the community can get involved

## Definitions

### P-Critical

A `P-Critical` issue is a potentially blocker issue.

The Working Group will keep track of these issues and will remind the compiler team on a weekly basis during the triage meeting.

Examples of things we typically judge to be “critical” bugs:

- Regressions where code that used to compile no longer does

  - Mitigating conditions that may lower priority:
    - If the code should never have compiled in the first place (but if the regression affects a large number of crates, this may indicate that we need a warning period)
    - If the code in question is theoretical and considered unlikely to exist in the wild, or if it only exists in small, unmaintained packages that are not widely used
  - If a regression has been in stable for a release or two (either because we are still awaiting a fix, or because the bug had laid dormant i.e. undetected), we typically lower the priority as well, because by that time, if the users have not raised a ruckus about the regression, that is a sign that it is inherently not a critical issue. Eg: [an issue that would have been P-Critical but ended up being P-High](https://rust-lang.zulipchat.com/#narrow/stream/227806-t-compiler.2Fwg-prioritization/topic/pre-meeting.20triage.202020-04-09.20.2354818)

- Regressions where code still compiles but does something different than it used to do (dynamic semantics have changed)
  - Mitigating conditions that may lower priority:
    - If code uses feature that is explicitly not specified (e.g. `std::vec::Vec` docs state order in which it drops its elements is subject to change)
- Feature-gated features accessible without a feature gate
  - Mitigating conditions that may lower priority:
    - If the pattern is VERY unlikely
- Soundness holes where common code that should not compile actually does
  - Mitigating conditions that may lower priority:
    - Soundness holes that are difficult to trigger
    - Soundness holes that have been around for a very long time may be critical, but typically require
- Diagnostic regressions where the diagnostic is very common and the situation very confusing
- ICEs for common scenarios or code patterns
  - Mitigating conditions that may lower priority:
    - If the code that triggers the ICE also triggers compilation errors, and those errors are emitted before the ICE
    - If the code in question makes use of unstable features, particularly if the ICE requires a feature gate

### P-High

`P-High` issues are the ones the compiler team will focus on the most.

They can be `P-Critical` issues that have a mitigating condition as defined above, or important issues that aren't deemed blockers.

The effectiveness of the prioritization Working Group will be a direct consequence of our ability to draw the line between `P-Critical` and `P-High` issues.

### P-Medium

`P-Medium` refer to issues that aren't critical, and that will be resolved in the long run. Eg issues that will be fixed after a specific feature has landed.

### P-Low

`P-Low` bugs are bugs that don't cause soundness or safety issues, and that occur in a "failing path" already. Eg: Code that wouldn't compile and raises an error. It might also refer to features that would be nice to have, but aren't deemed a priority

## Amount of focus a given issue will receive

### P-Critical

A P-critical issue will receive the most attention. It must be assigned one of several people as soon as possible, and the rest of the team should do their best to help them out if/when applicable.

### P-High

P-High issues are issues the teams will mostly work on. We want to make sure they're assigned, and keep an eye on them.

### P-Medium

P-Medium issues won't be our focus. They are issues we would mentor someone interested in fixing.

### P-Low

P-Low issues won't be our focus. They will remain in this state until someone complains, a community member fixes it, or it gets fixed by accident.

---

###### For questions and comments about this document feel free to hit the relevant [zulip topic](https://rust-lang.zulipchat.com/#narrow/stream/227806-t-compiler.2Fwg-prioritization/topic/What.20is.20the.20meaning.20of.20each.20priority.20level.3F) or [open an issue](https://github.com/rust-lang/rust-forge/issues).

######
