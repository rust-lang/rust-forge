# Issue Links

## Canonicalise Issue Links

GitHub permits having automatic action like `Fixes #123`, which closes the issue number `123`.
This handler updates the pull-request description with the canonicalized version, `Fixes org/repo#123`.

This is useful when updating subtrees into the upstream repository as it avoids referencing and closing the issue from the upstream repository instead of the one from the subtree.

## Issue Links in Commits

GitHub also permits having having issue links in commits which are then referenced in the issue. While useful, they often more than anything else spam the referenced issue. This handler puts a warning against them.

## Configuration

This feature is enabled on a repository by having a `[issue-links]` table in `triagebot.toml`:

```toml
[issue-links]
```

### Without commits checking

```toml
[issue-links]
check-commits = false # defaults to true
```

## Implementation

See [`src/handlers/issue_links.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/src/handlers/issue_links.rs) and [`src/handlers/check_commits/issue_links.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/src/handlers/check_commits/issue_links.rs).
