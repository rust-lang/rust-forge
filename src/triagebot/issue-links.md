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

Some repositories may not need, or may prefer not, to warn about issue links in commit messages. This behavior can be disabled with the `check-commits` configuration option:

```toml
[issue-links]
check-commits = false # defaults to true
```

#### Only check uncanonicalized issue links

For subtrees (or embedded repositories), it is recommended to still check for *uncanonicalized* issue links (e.g. `#132` instead of `rust-lang/cargo#123`). 
This prevents links from resolving to the wrong repository when the subtree is merged upstream while still allowing issue links in commits.

```toml
[issue-links]
check-commits = "uncanonicalized" # for subtrees
```

## Implementation

See [`src/handlers/issue_links.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/src/handlers/issue_links.rs) and [`src/handlers/check_commits/issue_links.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/src/handlers/check_commits/issue_links.rs).
