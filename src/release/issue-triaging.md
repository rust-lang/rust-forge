# Issue triaging

This page is about the `rust-lang/rust` repository. Other repositories may have different processes.

Tracking issues (label `C-tracking-issue`) don't fit into this procedure and are treated differently.

## Motivation

The `rust-lang/rust` repository has thousands of issues and hundreds of people working on it.
It is impossible for all people to check and solve issues. The goals of triaging are connecting
issues to the relevant people, and helping them be more effective at fixing the issue.

In practice, it is unrealistic for all issues to be solved quickly and found by right people.
Through applications of labels we make the issue tracker more searchable for future reference,
so that people in the future have an easier time finding related issues or issues they are interested
in working on.

Triaging can be done by **everyone**, no matter your permissions. We encourage everyone to help here,
as triaging work is highly parallelizable and easy to get started with.

## Initial triaging

When an issue is opened, it gets the `needs-triage` label. This ensures that every issue gets an initial
look and that no issue is ignored, or that when it is ignored, it is at least visibly ignored by still having the label.

`needs-triage` is an initial checkpoint. The effort needed to get an issue past the label should be minimal.

To do the initial triage and remove the `needs-triage` label, the following conditions should be fulfilled/considered.
It's okay if not all of these are always considered; treat it as a guideline, not a hard checklist. It is also not exhaustive.

- The issue should make sense, that is, it should present a problem.
    - For example, if an issue is a question about Rust in general, the issue should be closed and the user redirected to URLO/Discord.
      You can of course answer the question too :) (but make sure to mention that the user should go to URLO/Discord next time).
- Add appropriate labels ([Labels](#labels))
    - Specifically, `T-*` and `C-*` are the most relevant
- If the issue contains no reproduction but needs one, ask for one and add the `S-needs-repro` label
- The issue is the wrong place for some kinds of feature requests. Tell the author about it.
    - Library API requests should follow [its processes](https://std-dev-guide.rust-lang.org/development/feature-lifecycle.html).
    - Language changes should be redirected to [IRLO](https://internals.rust-lang.org/) or Zulip (t-lang).
- If the issue could benefit from bisecting the regression, add `E-needs-bisection` (or do the bisection yourself)
- Does this issue require nightly? Add `requires-nightly`.
- Is the issue a regression? Apply the `regression-untriaged` label (or figure out what regression it is exactly)
- If you happen to know people who this issue is relevant to, ping them.
    - For example, write `cc @ThatPerson` if `ThatPerson` has been working a lot on the problematic feature recently
- Does this issue require incomplete or internal features? Add `requires-{incomplete,internal}-features`.

For applying and removing labels, unprivileged users can use **@rustbot** to add or remove
[the labels allowed by the `triagebot.toml` configuration](https://github.com/rust-lang/rust/blob/master/triagebot.toml).
For example, `@rustbot label +T-compiler +C-bug +A-linkage +O-macos -needs-triage`.

To see a list of all labels, check out the "labels" page next to the search bar in the issue tracker.

## Further triaging

For issues that have been through the initial triaging step (that is, don't have the `needs-triage` label anymore), there are usually
still things that can be improved. There are often many more labels that could be applied (using rustbot again if you don't have privileges).

Additionally, old (there is no clear definition of old yet, but something on the order of months) `S-needs-repro` issues can be closed
if there is no way to make progress without a reproduction. This requires privileges, but if you don't have them, you can just link the issue
on Zulip (for example in `t-release/triage` or `general`) and someone with privileges can close it for you.

Another useful thing to do is go through `E-needs-mcve` and `E-needs-bisection` issues and creating minimizations or bisecting the issue
(using [cargo-bisect-rustc](`https://github.com/rust-lang/cargo-bisect-rustc`)). When you provide one, you can also remove the label
using rustbot (`@rustbot label -E-needs-bisection`).

At the time of writing, there is also the `needs-triage-legacy` label, for older issues that are suspected to not have been triaged.
Triaging them the same way as `needs-triage` is also useful.

## Labels

There are many different labels that can be applied to issues.

- `needs-triage`: signals that an issue is new and needs initial triage
- `T-*`: Specifies the team or teams that this issue is relevant to, for example compiler, types or libs
- `WG-*`: Specifies the working groups that this issue is relevant to, for example WG-debugging.
- `C-*`: Specifies the category of the label, for example a bug, tracking issue or discussion
    - `A-diagnostics` issues usually don't have any C label.
    - Also note `C-optimization` for missed compiler optimizations.
- `O-*`: For platform-specific issues, specifies the platform (architecture or operating system). For example macos, aarch64, windows
- `A-*`: The areas that the issue is relevant to, for example linkage, patterns, diagnostics
- `F-*`: When the issue concerns a specific (usually unstable) feature
- `requires-nightly`: This issue is not relevant to the stable compiler
- `requires-{incomplete,internal}-features`: This issue requires an incomplete or internal feature. The latter often means that the issue
    should be closed in accordance with compiler [MCP 620](https://github.com/rust-lang/compiler-team/issues/620).
- `regression-*`: Labels for tracking issues that are regressions.
- `D-*`: Labels for diagnostics issue.
- `I-*`: Different labels about the nature (originally, importance) of a bug. For example ICE, slow code, heavy code (binary size), crashes, unsoundness.
  There are also some other `I-*` labels that don't really fit into this. For triaging, focus on `I-slow`, `I-heavy`, `I-ICE`, `I-crash`, `I-unsound`.
- `P-*`: Priority labels. Applied using the [Compiler Prioritization procedure](../compiler/prioritization.md)
- `S-*`: The status of an issue, for example S-needs-repro.
- `E-*`: Calls for participation, for example to minimize an issue 
    - `E-mentor`: A mentor is available to help with the issue, which makes for good first issues.
    - `E-needs-mcve`: This issue has a reproduction, but it is not minimal, it should be minimized.
    - `E-needs-test`: The issue has been fixed, but no test has been added for it. After someone adds a test, it can be closed.
    - `E-{easy,medium,hard}`: Someone has estimated how hard the issue is to fix. This can help with finding good first issues, but is [bound to be inaccurate](https://en.wikipedia.org/wiki/Curse_of_knowledge).
