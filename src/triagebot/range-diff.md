# Git rebase (range) diff

GitHub native compare feature shows lots of unrelated changes when a force push changes the base commit of a PR.
This handler posts a comment after such a scenario, which links to triagebot `range-diff` feature.

## Configuration

This feature is enabled on a repository by having a empty `[range-diff]` table in `triagebot.toml`:

```toml
[range-diff]
```

## Implementation

See [`src/handlers/check_commits/range_diff.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/src/handlers/check_commits/force_push_range_diff.rs).
