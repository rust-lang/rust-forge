# Requesting Prioritization

Users can request an issue to be prioritized. Prioritization means that the relevant team (T-compiler, T-rustdoc, T-libs) will have a look at the issue and assess its priority.

This procedure is usually reserved to *regressions*, the Rust project takes regressions seriously and these have usually priority above all the rest. When a regression is filed using the dedicated [GitHub issue template][gh-regression-tpl], a prioritization of that issue is automatically requested.

To learn more about this procedure, please visit the [prioritization] documentation.

[gh-regression-tpl]: https://github.com/rust-lang/rust/blob/master/.github/ISSUE_TEMPLATE/regression.md
[prioritization]: ../compiler/prioritization.md

## Usage

On repositories configured for prioritization, any user can post a comment with:

```text
@rustbot prioritize
```

which will add the `I-prioritize` label to the issue and send a notification on [Zulip][prio-alerts].

[prio-alerts]: https://rust-lang.zulipchat.com/#narrow/channel/245100-t-compiler.2Fprioritization.2Falerts/topic/.23147831.20Redundant.20bounds.20check.20when.20indexing.20array.20with.20enu.E2.80.A6/with/545925945

## Configuration

This feature is enabled on a repository by the `[prioritize]` table in `triagebot.toml`:

```toml
[prioritize]
# Name of the label used for requesting prioritization on issues
label = "I-prioritize"
```

## Implementation

See [`parser/src/command/prioritize.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/parser/src/command/prioritize.rs) and
[`src/handlers/prioritize.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/src/handlers/prioritize.rs).
