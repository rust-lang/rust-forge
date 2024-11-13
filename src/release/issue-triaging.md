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

`needs-triage` is an initial checkpoint. The effort needed to get an issue past the label should be small.

To do the initial triage and remove the `needs-triage` label, the following conditions should be fulfilled/considered.
It's okay if not all of these are always considered; treat it as a guideline, not a hard checklist. It is also not exhaustive.

- The issue should make sense, that is, it should present a problem.
    - For example, if an issue is a question about Rust in general, the issue should be closed and the user redirected to URLO/Discord.
      You can of course answer the question too :) (but make sure to mention that the user should go to [URLO]/[Discord] next time).
- Add appropriate labels ([Labels](#labels))
    - Specifically, `T-*` and `C-*` are the most relevant
- If the issue contains no reproduction but needs one (when in doubt, it needs one), ask for one and add the `S-needs-repro` label
- The issue tracker is the wrong place for some kinds of feature requests. Suggest the author where they can get support.
    - Standard library API requests should follow [libs-api processes](https://std-dev-guide.rust-lang.org/development/feature-lifecycle.html).
    - Language changes should be redirected to [IRLO] or Zulip ([t-lang](https://rust-lang.zulipchat.com/#narrow/stream/213817-t-lang)).
- If the issue could benefit from bisecting the regression (when in doubt, it can), add `E-needs-bisection` (or do the bisection yourself)
- Does this issue require nightly? Add `requires-nightly`.
- Is the issue a regression? Apply the `regression-untriaged` label (or figure out what regression it is exactly)
- If you happen to know people who this issue is relevant to, ping them.
    - For example, write `cc @ThatPerson` if `ThatPerson` has been working a lot on the feature in question recently.
- Does this issue require incomplete or internal features? Add `requires-{incomplete,internal}-features`.

For applying and removing labels, unprivileged users can use **@rustbot** to add or remove
[the labels allowed by the `triagebot.toml` configuration](https://github.com/rust-lang/rust/blob/master/triagebot.toml).
For example, `@rustbot label +T-compiler +C-bug +A-linkage +O-macos -needs-triage`.

To see a list of all labels, check out the "labels" page next to the search bar in the issue tracker.

[URLO]: https://users.rust-lang.org
[IRLO]: https://internals.rust-lang.org/
[Discord]: https://discord.gg/rust-lang

### ICE Triage
For [Issues that have both I-ICE and needs-triage](https://github.com/rust-lang/rust/issues?q=is%3Aissue%20state%3Aopen%20label%3AI-ICE%20label%3Aneeds-triage)
* Check that the issue is actually an ICE, and not more accuratly described with `I-crash` or `I-hang`.
* If it is an older (like latest stable) version of rust, ask for or check the latest nightly.
* Check for duplicates, but don't close as duplicate unless you're sure they represent the same underlying issue.
  Prefer simply linking to the issue as possibly related/duplicate.
* If it does not have a reproduction, comment asking for one and add S-needs-repro. if there isn't one for around a month it should generally be closed.
* If the reproduction is not minimal, add `E-needs-mcve` or create a Minimal Complete and Verifible Example yourself.
* Add `A-*` labels based on the code that causes the issue (check backtraces!),
  and the nature of the repro (eg. if the repro is a weird trait impl or the backtrace points to `rustc_trait_selection`, add `A-traits`)
* Add `T-*`, `WG-*`, `PG-*`, `F-*`, `requires-*`, and `regression-*` labels as appropriate.


## Further triaging

For issues that have been through the initial triaging step (that is, don't have the `needs-triage` label anymore), there are usually
still things that can be improved. There are often many more labels that could be applied (using rustbot again if you don't have privileges).

Additionally, old (there is no clear definition of old yet, but something on the order of months) `S-needs-repro` issues can be closed
if there is no way to make progress without a reproduction. This requires privileges, but if you don't have them, you can just link the issue
on Zulip (for example in `t-release/triage` or `general`) and someone with privileges can close it for you.

Another useful thing to do is go through `E-needs-mcve` and `E-needs-bisection` issues and creating minimizations or bisecting the issue
(using [cargo-bisect-rustc](`https://github.com/rust-lang/cargo-bisect-rustc`)). When you provide one, you can also remove the label
using rustbot (`@rustbot label -E-needs-bisection`).

## Labels

There are many different labels that can be applied to issues.

- `needs-triage`: Signals that an issue is new and needs initial triage
- [`T-*`]: Specifies the team or teams that this issue is relevant to. For example `T-compiler`, `T-types` or `T-libs`.
- [`WG-*`]: Specifies the working groups that this issue is relevant to, for example `WG-debugging`.
- [`PG-*`]: Specifies the project groups that this issue is relevant to, for example the `PG-exploit-mitigations`.
- [`C-*`]: Specifies the category of the label, for example a bug, tracking issue or discussion
    - `A-diagnostics` issues usually don't have any `C-*` label.
    - `C-optimization` for missed compiler optimizations.
    - `C-defective-hardware` for hardware bugs that are beyond our control.
    - `C-external-bug` for software bugs that affect us but we don't have direct control over, but is worth tracking from our side.
- [`O-*`]: For target-specific issues, specifies the compile target[^1] or compile target family (most notably the platform, i.e., the architecture or operating system). For example `O-macos`, `O-aarch64`, `O-windows`, `O-windows-msvc`.
- [`A-*`]: The areas that the issue is relevant to, for example `A-linkage`, `A-patterns`, `A-diagnostics`.
- [`L-*`]: When the issue concerns a specific lint.
    - `L-false-positive` if the lint fires on a case that it should not have fired on.
    - `L-false-negative` if the lint misses a case where it should have fired on.
- [`F-*`]: When the issue concerns a specific (usually unstable, usually language) feature.
- [`-Z*`]: When the issue concerns a specific unstable `-Z` compiler flag.
- `requires-nightly`: This issue is not relevant to the stable compiler
- `requires-{incomplete,internal}-features`: This issue requires an incomplete or internal feature. The latter often means that the issue
    should be closed in accordance with compiler [MCP 620](https://github.com/rust-lang/compiler-team/issues/620).
- [`regression-*`]: Labels for tracking issues that are regressions.
- [`D-*`]: Labels for diagnostic issues.
    - `D-diagnostic-infra`: This issue is about the diagnostics infrastructure itself.
- [`I-*`]: Different labels about the nature[^2] of a bug. For example ICE, slow code, heavy code (binary size), crashes, unsoundness.
  There are also some other `I-*` labels that don't really fit into this. For triaging, focus on `I-ICE`, `I-crash`, `I-hang`, `I-slow`, `I-heavy`, `I-compiletime` and `I-unsound`.
- [`P-*`]: Priority labels. Applied using the [compiler prioritization procedure](../compiler/prioritization.md).
- [`S-*`]: The status of an issue, for example `S-needs-repro`.
- [`E-*`]: Calls for participation[^3], for example to minimize an issue.
    - `E-mentor`: A mentor is available to help with the issue, which makes for good first issues.
    - `E-needs-mcve`: This issue has a reproduction, but it is not minimal, it should be minimized.
    - `E-needs-bisection`: This issue needs a bisection, for example using [cargo-bisect-rustc](https://github.com/rust-lang/cargo-bisect-rustc).
    - `E-needs-investigation`: This issue needs further investigation to determine root causes and the nature of the issue.
    - `E-needs-design`: This issue will require some substantial design effort (exploration, prototyping, discussions, etc.).
    - `E-needs-test`: The issue has been fixed, but no test has been added for it. After someone adds a test, it can be closed.
    - `E-{easy,medium,hard}`: Someone has estimated how hard the issue is to fix. This can help with finding good first issues, but is [bound to be inaccurate](https://en.wikipedia.org/wiki/Curse_of_knowledge).

See also section [Issue Triage](https://rustc-dev-guide.rust-lang.org/contributing.html#issue-triage) in the Rust Compiler Development Guide.

[`T-*`]: https://github.com/rust-lang/rust/labels?q=T-
[`WG-*`]: https://github.com/rust-lang/rust/labels?q=WG-
[`PG-*`]: https://github.com/rust-lang/rust/labels?q=PG-
[`C-*`]: https://github.com/rust-lang/rust/labels?q=C-
[`O-*`]: https://github.com/rust-lang/rust/labels?q=O-
[`A-*`]: https://github.com/rust-lang/rust/labels?q=A-
[`L-*`]: https://github.com/rust-lang/rust/labels?q=L-
[`F-*`]: https://github.com/rust-lang/rust/labels?q=F-
[`-Z*`]: https://github.com/rust-lang/rust/labels?q=-Z
[`regression-*`]: https://github.com/rust-lang/rust/labels?q=regression-
[`D-*`]: https://github.com/rust-lang/rust/labels?q=D-
[`I-*`]: https://github.com/rust-lang/rust/labels?q=I-
[`P-*`]: https://github.com/rust-lang/rust/labels?q=P-
[`S-*`]: https://github.com/rust-lang/rust/labels?q=S-
[`E-*`]: https://github.com/rust-lang/rust/labels?q=E-
[^1]: The `O` in `O-*` labels originally stood for *operating system (OS)*.
[^2]: The `I` in `I-*` labels originally stood for *importance*. This makes the most sense for the `I-*-nominated` labels. For most `I-*` labels however it makes sense to interpret the `I` as *issue (kind)*.
[^3]: The `E` in `E-*` labels stands for *experience*.
