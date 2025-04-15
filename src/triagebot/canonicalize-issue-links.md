# Canonicalize Issue Links

GitHub permits having automatic action like `Fixes #123`, which closes the issue number `123`.
This handler updates the pull-request description with the canonicalized version, `Fixes org/repo#123`.

This is useful when updating subtrees into the upstream repository as it avoids referencing and closing the issue from the upstream repository instead of the one from the subtree.

## Configuration

This feature is enabled on a repository by having a `[canonicalize-issue-links]` table in `triagebot.toml`:

```toml
[canonicalize-issue-links]
```

## Implementation

See [`src/handlers/canonicalize_issue_links.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/src/handlers/canonicalize_issue_links.rs).
