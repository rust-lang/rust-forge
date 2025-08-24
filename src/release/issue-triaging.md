# Issue triaging

This page is about the [`rust-lang/rust`] repository. Other repositories under the `rust-lang` GitHub organization likely have different processes.

Tracking issues (issues labeled `C-tracking-issue`) don't fit into this procedure and are treated differently.

## Motivation

The [`rust-lang/rust`] repository has thousands of issues and hundreds of people working on it. It is impossible for all people to check and solve issues. The goals of triaging are to:

- Improve **discoverability**:
    - Categorize which teams an issue may concern.
    - Make sure regressions are properly tagged and tracked.
    - Connect issues to the relevant people (especially relevant teams).
- First-pass **coarse classification** of the issue nature:
    - Is this issue a bug, or is this discussion, or is this a user-side problem?
- Identify if the issue is **sufficiently actionable**:
    - Does the report need further info, like reproduction steps, a reproducer, or build/execution environment info?
    - Does the reproducer need minimization?
- Make it easier for contributors to **filter** issues by applying suitable labels.

In practice, it is unrealistic for all issues to be solved quickly and found by right people. Through applications of labels we make the issue tracker more searchable for future reference, so that people in the future have an easier time finding related issues or issues they are interested in working on.

Triaging can be done by **everyone**, no matter if you have write access to [`rust-lang/rust`] or not. We encourage everyone to help here, as triaging work is highly parallelizable and easy to get started with.

Some actions that require write access to [`rust-lang/rust`] include:

- Using `@rustbot transfer` to transfer the issue to another repository under `rust-lang` GitHub organization.
- Closing the issue.
- Using `@rustbot` to apply certain labels that require write access (see "Applying and removing labels" section below).

If you don't have write access but you assess that these actions should be done, you can just open a new topic in [`t-release/triage`] zulip channel, or add a comment linking to the issue in an existing topic in the [`t-release/triage`] channel.

## Initial triaging

When an issue is opened, it usually receives the `needs-triage` label automatically. This is done so that untriaged issues are easily discovered and are filterable, to make sure that new issues receive an initial triaging pass (describe below). `needs-triage` is an *initial* checkpoint. The effort needed to get an issue past the label should be small.

To do the initial triage and remove the `needs-triage` label, the following conditions should be fulfilled/considered. It's okay if not all of these are always considered. Treat this non-exhaustive list as a guideline, not a hard checklist:

- The issue should make sense, that is, it should present a problem.
    - For example, if an issue is a question about Rust in general, the issue should be closed and the user redirected to [URLO]. You can, of course, answer the question too, but make sure to mention that the user should go to [URLO] next time.
- Check if this issue is a duplicate of earlier-reported issues.
    - If you are certain this is a duplicate, close this issue as a duplicate of the earlier issue. Make sure this is obvious in the backlink of the earlier issue, or explicitly link to the duplicate issue.
    - If you are not sure, you can still leave a comment to indicate the other issue is possibly a duplicate, similar, or related.
- Add appropriate labels ([Labels](#labels)).
    - Specifically, team labels (`T-*`) and issue category labels (`C-*`) are the most important, because team members will usually rely on those as a baseline to filter what concerns the team (e.g. during triage meetings).
- If the issue contains no reproduction but needs one (when in doubt, it needs one), ask for one and add the `S-needs-repro` label.
- If the issue lacks information (e.g. system information, the exact invocation used), then request that the reporter provide the information that is important to reproduce the problem. Add the `S-needs-info` label.
- The issue tracker is the wrong place for some kinds of feature requests. Redirect the author to proper channels:
    - Standard library API requests should follow [libs-api processes](https://std-dev-guide.rust-lang.org/development/feature-lifecycle.html).
    - Language changes should be redirected to [IRLO] or T-lang zulip channel ([t-lang](https://rust-lang.zulipchat.com/#narrow/stream/213817-t-lang)).
- If the issue could benefit from bisecting the regression (when in doubt, it can), add `E-needs-bisection` (or do the bisection yourself). An useful bisection tool is [`cargo-bisect-rustc`].
    - If there is a bisection and the bisection outcome is convincing, add `S-has-bisection`.
- If the reported example is large and/or complex (e.g. a lot of code, has a lot of external dependencies, has unrelated warnings or errors that obfuscates the actual problem), add `E-needs-mcve` to indicate the need to minimize the example.
    - If there is a Minimal, Complete and Verifiable Example (MCVE), then tag the issue with `S-has-mcve`. Double-check that the minimization is valid, e.g. the MCVE is for the original problem and has not drifted to a different problem.
- Is the issue a regression? If so, apply the `regression-untriaged` label (or figure out what regression it is exactly). Generally speaking, an issue is considered a regression if something used to work, but no longer does.
- If you happen to know people who this issue is relevant to, ping them.
    - For example, write `cc @ThatPerson` if `ThatPerson` has been working a lot on the feature in question recently.
- Does this issue require nightly? Add `requires-nightly`.
- Does this issue require incomplete or internal features? Add `requires-{incomplete,internal}-features`.

### Applying and removing labels

Users without write access to [`rust-lang/rust`] can use **@rustbot** to add or remove [the labels allowed by the `triagebot.toml` configuration](https://github.com/rust-lang/rust/blob/master/triagebot.toml) as a workaround.

Users with write access should change the labels directly to avoid sending a notification to everyone subscribed to the issue unnecessarily.

For example:

```console
@rustbot label +T-compiler +C-bug +A-linkage +O-macos -needs-triage
```

To see a list of all labels, check out the "labels" page next to the search bar in the issue tracker.

Note that some labels may only be applied by users with write access to [`rust-lang/rust`]. Refer to the `allow-unauthenticated` list under `[relabel]` section in [`triagebot.toml`](https://github.com/rust-lang/rust/blob/master/triagebot.toml) to see what labels users without write access may use.

### Relnotes triage

For issues labeled as `relnotes-tracking-issue`, the `needs-triage` tag should not be removed until the release notes text has been cleaned up.

### Labels

There are many different labels that can be applied to issues.

- `needs-triage`: Signals that an issue is new and needs initial triage.
- [`T-*`]: Specifies the team or teams that this issue is relevant to. For example `T-compiler`, `T-types` or `T-libs`.  See [Team Examples](#team-examples) for more details.
- [`WG-*`]: Specifies the working groups that this issue is relevant to, for example `WG-debugging`.
- [`PG-*`]: Specifies the project groups that this issue is relevant to, for example the `PG-exploit-mitigations`.
- [`C-*`]: Specifies the category of the label, for example a bug, tracking issue or discussion.
    - `C-optimization`: For missed compiler optimizations.
    - `C-defective-hardware`: For hardware bugs that are beyond our control.
    - `C-gub`: If the problem is due to user error, e.g. undefined behavior.
    - `C-discussion`: Not a bug, but still worth discussing.
    - `C-external-bug`: For software bugs that affect us but we don't have direct control over, but is worth tracking from our side.
- [`O-*`]: For target-specific issues, specifies the compile target[^1] or compile target family (most notably the platform, i.e., the architecture or operating system). For example `O-macos`, `O-aarch64`, `O-windows`, `O-windows-msvc`.
    - Where possible, specify the most specific target-category. For instance, if an issue affects `*-windows-gnu` but not `*-windows-msvc`, prefer `O-windows-gnu` over the generic `O-windows`.
- [`A-*`]: The areas that the issue is relevant to, for example `A-linkage`, `A-patterns` or `A-diagnostics`. This is particularly helpful for filtering issues.
    - `A-diagnostics`: Issues created from the diagnostics issue template only have `A-diagnostics` and not `C-bug`.
- [`B-*`]: Issues which are blockers.
- [`D-*`]: Labels for diagnostic issues.
    - `D-diagnostic-infra`: This issue is about the diagnostics infrastructure itself.
- [`L-*`]: When the issue concerns a specific lint.
    - `L-false-positive` if the lint fires on a case that it should not have fired on.
    - `L-false-negative` if the lint misses a case where it should have fired.
- [`F-*`]: When the issue concerns a specific (usually unstable, usually language) feature.
- [`-Z*`]: When the issue concerns a specific unstable `-Z` compiler flag.
- `requires-nightly`: This issue only affects the nightly channel compiler, and does not affect the beta or stable channel compiler.
- `requires-{incomplete,internal}-features`: This issue requires an incomplete or internal feature. The latter often means that the issue should be closed in accordance with compiler [MCP 620](https://github.com/rust-lang/compiler-team/issues/620).
- [`regression-*`]: Labels for tracking issues that are regressions.
    - `regression-untriaged`: The nature and kind of regression is unclear; indicates further regression triage is necessary.
    - `regression-from-stable-to-stable`: Something that regressed from an older stable release, which has already reached the latest stable release or even earlier stable releases.
    - `regression-from-stable-to-{beta,nightly}`: Something that regressed from the a stable release to beta or nightly channel.
- [`I-*`]: Different labels about the nature[^2] of an issue.
    - `I-ICE`: Internal compiler error.
    - `I-prioritize`: Indicates that the issue should be additionally triaged by [T-compiler/ops](../compiler/prioritization.md) to assign a priority `P-*` if applicable to indicate the urgency.
    - `I-heavy`: Heavy code (binary size).
    - `I-slow`: Slow run-time performance.
    - `I-hang`: The compilation fails to complete after a long time.
    - `I-crash`: The compiler or generated code crashes but does not manifest as an ICE, e.g. SIGSEGV or access violations.
    - `I-unsound`: Library, compiler, type system or language unsoundness.
    - `I-compilemem`: Excessive memory usage during compilation.
    - `I-compiletime`: Slow compilation time.
    - `I-{team}-nominated`: Issue is nominated for discussion by `{team}`. E.g. `I-compiler-nominated`.
    - `I-lang-easy-decision`: The decision needed by T-lang is conjectured (by the person applying the label) to be easy or perfunctory. Note that this label does not imply `I-lang-nominated`; the nomination label should be applied simultaneously if the person apply the label wants to nominate the issue for T-lang discussion.
- [`P-*`]: Priority labels. Applied using the [compiler prioritization procedure](../compiler/prioritization.md).
- [`E-*`]: Calls for participation[^3], for example to minimize an issue.
    - `E-mentor`: A mentor is available to help with the issue, which makes for a good first issue. Make sure that the intended mentor is actually willing and available to mentor.
    - `E-needs-mcve`: This issue has a reproduction, but it is not minimal. It should be minimized.
    - `E-needs-bisection`: This issue needs a bisection, for example using [`cargo-bisect-rustc`].
    - `E-needs-investigation`: This issue needs further investigation to determine root causes and the nature of the issue.
    - `E-needs-design`: This issue will require some substantial design effort (exploration, prototyping, discussions, etc.).
    - `E-needs-test`: The issue has been fixed, but no test has been added for it. After someone adds a test, it can be closed.
    - `E-{easy,medium,hard}`: Someone has estimated how hard the issue is to fix. This can help with finding good first issues, but is [bound to be inaccurate](https://en.wikipedia.org/wiki/Curse_of_knowledge).
- [`S-*`]: The status of an issue, for example `S-needs-repro` or `S-needs-info`.
    - `S-needs-info`: Needs more information from issue reporter.
    - `S-needs-repro`: Needs reproducer (code sample, repro steps, etc.).
    - `S-bug-has-test`: Issue has a known-bug test in [`rust-lang/rust`] `tests/crashes` test suite or elsewhere.
    - `S-has-bisection`: A bisection has been performed and the bisection result is convincing.
    - `S-waiting-on-LLVM`: Waiting on upstream LLVM PR or fix.
    - `S-tracking-forever`: The tracking issue is never intended to be closed.
- [`beta-nominated`]: Tracks changes nominated for being [backported to beta].
- [`beta-accepted`]: Tracks changes that have been approved for being [backported to beta]. `T-release` will usually handle the backport.
- [`stable-nominated`]: Tracks changes nominated for being [backported to stable], in anticipation of a point release.
- [`stable-accepted`]: Tracks changes that have been approved for being [backported to stable]. `T-release` will usually handle the backport.
- [`relnotes`]: Changes that are proposed to be documented in the release notes of the next release
  - It makes triagebot create a new relnotes issue ([example][relnotes issue example])
  - It also marks relnotes issues, so it can be processed by T-release relnotes tooling
  - An FCP will also cause a relnotes issue to be created, if it's started on an issue.
- [`metabug`]: Tracks other bugs.

#### Team Examples
This section gives a list of examples of kinds of issues that should be assigned to a specific team.

##### T-compiler
Anything related to the compiler implementation, such as diagnostics and ICEs.

##### T-libs
* Changes to implementation details of library functions
* Spelling, grammar, and organizational changes in library docs

##### T-libs-api
* New library apis, such as functions and methods
* Changes to signatures of unstable functions
* Semantic changes to library documentation,
  such as guaranteeing that a function will produce a certain error code in some situation
* Type inference breakage due to library changes, such as new trait implementations

##### T-lang
* New keywords/language features
* Changes to the keyword docs in the standard library

##### T-spec
* Changes to [the reference](https://github.com/rust-lang/reference/)

##### T-opsem
* Changes to [the nomicon](https://doc.rust-lang.org/nomicon/)
* Changes to the semantics of the abstract machine, such as the sematics of atomics.
* Changes to the docs of unsafe pointer functions
* Changes to the docs of `core::ptr`

### Creating labels

<div class="warning">

Triagebot needs to support `@rustbot label: xxx` usages terminated with a period or whitespace (as inline invocation), so the label name must consist of only alphanumeric or hyphen (`-`) or underscore (`_`) characters.

</div>

- Check existing labels to make sure you're not duplicating them.
- Discuss in <https://rust-lang.zulipchat.com/#narrow/channel/242269-t-release.2Ftriage/topic/New.20labels> if the new label may be non-conventional or controversial. Leave a comment about the new label as an FYI for others.

### Relnotes issues

Release note issues will currently come with `needs-triage` by default. The triage for relnotes is usually best done if you have sufficient context. Leave them as-is if you don't.

- Remove unrelated area `A-*` labels that concern the actual PR, but not the actual user-facing relnotes.
- Remove unrelated `T-*` or `WG-*` labels.
- Remove `needs-triage` if the relnotes wording has been confirmed/changed by the PR author, reviewer or a relevant team member.

### Internal Compiler Error (ICE) issue triage

For issues that include an Internal Compiler Error (ICE):

- Make sure the issue is tagged with `I-ICE`, `T-compiler` and `C-bug`.
- Check that the issue is actually an ICE, and not more accurately described with `I-crash` or `I-hang`.
- If it is an older (like latest stable) version of rust, ask:
    - Does the ICE reproduce on beta and latest nightly.
    - If older stable, ask if the ICE reproduces on latest stable.
- Check for duplicates, but don't close as duplicate unless you're sure they represent the same underlying issue. Prefer simply linking to the issue as possibly related/duplicate.
    - A good way is to search the ICE message. Be careful to not close as duplicate when the same ICE message may have different root causes.
- If it does not have a reproduction, comment asking for one and add ` S-needs-repro`.
    - If there isn't one for around a month it should generally be closed.
- If the reproduction is not minimal, add `E-needs-mcve` or create a MCVE yourself. See the "Initial triaging" section for more details on MCVE procedure.
- Add `A-*` labels based on the code that causes the issue (check backtraces!), and the nature of the repro (e.g. if the repro is a weird trait impl or the backtrace points to `rustc_trait_selection`, add `A-traits`).
- Add `T-*`, `WG-*`, `PG-*`, `F-*`, `requires-*`, and `regression-*` labels as appropriate.
    - Usually the ICE should be tagged `T-compiler`. If the issue concerns the type system (e.g. trait solver), also tag the issue with `T-types`.

## Further triaging

For issues that have been through the initial triaging step (that is, don't have the `needs-triage` label anymore), there are usually still things that can be improved. There are often many more labels that could be applied (using rustbot again if you don't have write access to [`rust-lang/rust`]).

Additionally, old (there is no clear definition of old yet, but something on the order of months) `S-needs-repro` issues can be closed if there is no way to make progress without a reproduction. This requires write access to [`rust-lang/rust`], but if you don't have them, you can just link the issue on Zulip (for example in [`t-release/triage`] or `general`) and someone with write access can close it for you.

Another useful thing to do is to go through `E-needs-mcve` and `E-needs-bisection` issues and create minimizations or bisecting the issue (using [`cargo-bisect-rustc`]). When you provide one, you can also remove the label using rustbot (`@rustbot label -E-needs-bisection`).

[`rust-lang/rust`]: https://github.com/rust-lang/rust
[URLO]: https://users.rust-lang.org
[IRLO]: https://internals.rust-lang.org/
[`cargo-bisect-rustc`]: https://github.com/rust-lang/cargo-bisect-rustc
[`t-release/triage`]: https://rust-lang.zulipchat.com/#narrow/stream/242269-t-release.2Ftriage

[`T-*`]: https://github.com/rust-lang/rust/labels?q=T-
[`WG-*`]: https://github.com/rust-lang/rust/labels?q=WG-
[`PG-*`]: https://github.com/rust-lang/rust/labels?q=PG-
[`B-*`]: https://github.com/rust-lang/rust/labels?q=B-
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
[`beta-nominated`]: https://github.com/rust-lang/rust/labels/beta-nominated
[`stable-nominated`]: https://github.com/rust-lang/rust/labels/stable-nominated
[`beta-accepted`]: https://github.com/rust-lang/rust/labels/beta-accepted
[`stable-accepted`]: https://github.com/rust-lang/rust/labels/stable-accepted
[backported to beta]: backporting.md#beta-backporting-in-rust-langrust
[backported to stable]: backporting.md#stable-backporting-in-rust-langrust
[`relnotes`]: https://github.com/rust-lang/rust/labels/relnotes
[`metabug`]: https://github.com/rust-lang/rust/labels/metabug
[relnotes issue example]: https://github.com/rust-lang/rust/issues/137132

[^1]: The `O` in `O-*` labels originally stood for *operating system (OS)*.
[^2]: The `I` in `I-*` labels originally stood for *importance*. This makes the most sense for the `I-*-nominated` labels. For most `I-*` labels however it makes sense to interpret the `I` as *issue (kind)*.
[^3]: The `E` in `E-*` labels stands for *experience*.
