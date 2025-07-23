# Automatic nomination of pull requests for backport

When a [regression][regression-beta] for the compiler or the standard library (or any other component) is fixed, we evaluate whether to backport the patch to the release channel where it originated (beta or stable, see [backports]). The relevant team looks at the fix and either accepts or declines the backport.

In order to give more visibility to patches fixing important regressions and facilitate the team taking a decision, a discussion topic on our [Zulip chat][zulip-chat] can be automatically opened ([example][zulip-backport-poll]).

Automatic nomination of pull requests for backport can be configured to scan every new patch in the git repository and look for a text marker in the opening comment (example "Fixes #123", see [GitHub documentation][gh-assoc-issue-patched]). If such text is found and the issue being fixed is classified as `P-high` or `P-critical` and has one of the `required-issue-label` labels (e.g. `regression-from-*`), the handler will apply the configured `add-labels` labels.

[regression-beta]: https://github.com/rust-lang/rust/issues?q=is%3Aissue%20label%3Aregression-from-stable-to-beta
[backports]: ../compiler/backports.md#backports
[zulip-chat]: ../platforms/zulip.md
[zulip-backport-poll]: https://rust-lang.zulipchat.com/#narrow/channel/474880-t-compiler.2Fbackports/topic/.23143509.3A.20beta-nominated/with/527517416
[gh-assoc-issue-patched]: https://docs.github.com/en/issues/tracking-your-work-with-issues/creating-issues/linking-a-pull-request-to-an-issue

## Configuration

The automatic backport labelling is enabled on a repository when at least one `[backport.<foo>]` table is configured in `triagebot.toml`:

```toml
[backport.foo]
required-pr-labels = ["T-compiler"]
required-issue-label = "regression-from-stable-to-beta"
add-labels = ["beta-nominated"]

[backport.bar]
required-pr-labels = ["T-compiler"]
required-issue-label = "regression-from-stable-to-stable"
add-labels = ["beta-nominated", "stable-nominated"]

[backport.baz]
required-pr-labels = ["T-libs", "T-libs-api"]
required-issue-label = "regression-from-stable-to-stable"
add-labels = ["stable-nominated"]
```

`foo`, `bar`, `baz` are examples of unique identifiers to disambiguate them, they can be anything.

Fields explanation:
- `required-pr-labels`: a list of labels that the pull request must have when it is opened (suggested to use a team label `T-*`)
- `required-issue-label`: a label that the regression must have in order to be identified as such. It can be one of: `regression-from-stable-to-nightly`, `regression-from-stable-to-beta` or `regression-from-stable-to-stable`.
- `add-labels`: a list of labels that the pull request will be assigned, if all conditions apply.

## Implementation

See [`src/handlers/backport.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/src/handlers/backport.rs).
