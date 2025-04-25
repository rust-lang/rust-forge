# Behind Upstream

This handler checks if a PR is based on an *X* days old commit.

## Context

When a PR's code is based on a very old commit from an upstream branch:
It passes when tested locally, but fails when the PR is submitted for testing through CI.

This is because the CI applies the commit patches to the current upstream branch,
which may have new test cases, so it won't pass. We need to rebase the PR to the nearest upstream branch.

## Configuration

This feature is enabled on a repository by having a `[behind-upstream]` table in `triagebot.toml`:

```toml
[behind-upstream]
```
or, you can set the day threshold,
```toml
[behind-upstream]
days-threshold = 7
```

## Implementation

See [`src/handlers/check_commits/behind_upstream.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/src/handlers/check_commits/behind_upstream.rs).