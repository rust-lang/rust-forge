# Git rebase (range) diff

GitHub native compare feature shows lots of unrelated changes when a force push changes the base commit of a PR.
This handler posts a comment after such a scenario, which links to triagebot `range-diff` feature.

## Configuration

> [!NOTE]
> This feature is [enabled by default](https://github.com/rust-lang/triagebot/blob/master/rust-lang.triagebot.toml) in the rust-lang organization.

This feature is enabled on a repository by having a empty `[range-diff]` table in `triagebot.toml`:

```toml
[range-diff]
```

There are optional keys that you can include:

- `consider-push-from-bots` (default: `false`) --- Should push from bots be consider for a range-diff comment.

## Implementation

See [`src/handlers/check_commits/range_diff.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/src/handlers/check_commits/force_push_range_diff.rs).
