# Nominate pull requests for backport

When a [regression][regression-beta] for the compiler or the standard library (or any other component) is fixed, we evaluate whether to backport the patch to the release channel where it originated (beta or stable, see [backports][backports]). The decision to backport a patch is quickly skimmed by the relevant team and either accepted or declined.

In order to give more visibility to patches fixing important regressions and facilitate the team taking a decision, a discussion topic on our [Zulip chat][zulip-chat] can be automatically opened ([example][zulip-backport-poll]).

When configured in the `triagebot.toml`, the triagebot will scan every new patch in the git repository looking for a text marker in the opening comment (example "Fixes #123", see [GitHub documentation][gh-assoc-issue-patched]). If such text is found and the issue being fixed is classified as `P-high` or `P-critical` and has a `regression-from-*` label, the triagebot will open a new Zulip discussion topic.

[regression-beta]: https://github.com/rust-lang/rust/issues?q=is%3Aissue%20label%3Aregression-from-stable-to-beta
[backports]: https://forge.rust-lang.org/compiler/backports.html#backports
[zulip-chat]: https://forge.rust-lang.org/platforms/zulip.html
[zulip-backport-poll]: https://rust-lang.zulipchat.com/#narrow/channel/474880-t-compiler.2Fbackports/topic/.23143509.3A.20beta-nominated/with/527517416
[gh-assoc-issue-patched]: https://docs.github.com/en/issues/tracking-your-work-with-issues/creating-issues/linking-a-pull-request-to-an-issue

## Configuration

The automatic backport scanning is enabled on a repository with at least one `[backport.<foo>]` table in `triagebot.toml`. There can be multiple occurrence of this configuration to handle different cases:

```toml
[backport.foo]
required-pr-labels = ["T-compiler"]
required-issue-label = "regression-from-stable-to-beta"
add-labels= ["beta-nominated"]

[backport.bar]
required-pr-labels = ["T-compiler"]
required-issue-label = "regression-from-stable-to-stable"
add-labels= ["beta-nominated", "stable-nominated"]

[backport.baz]
required-pr-labels = ["T-libs", "T-libs-api"]
required-issue-label = "regression-from-stable-to-stable"
add-labels= ["stable-nominated"]
```

`foo`, `bar`, `baz` are just examples of unique identifiers to disambiguate them.

Explaination:
- `required-pr-labels`: a list of labels that the pull request must have when it is opened (suggested to use a team label `T-*`)
- `required-issue-label`: a label that the regression must have in order to be identified as such. It can be one of: `regression-from-stable-to-nightly`, `regression-from-stable-to-beta` or `regression-from-stable-to-stable`.
- `add-labels`: a list of labels that the pull request will be assigned, if all conditions apply.

## Implementation

See [`src/handlers/backport.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/src/handlers/backport.rs).
