# No Mentions (in commits)

GitHub permits having mentions (eg. `@user`) in commits, and while it's sometimes useful it almost always ends-up spamming the mentioned users, in particular as the commits are being rebased, cherry-picked or pushed.

This handler tries to prevent those mentions by adding a comment warning in the PR against them.

## Configuration

This feature is enabled on a repository by having a `[no-mentions]` table in `triagebot.toml`:

```toml
[no-mentions]
```

## Implementation

See [`src/handlers/check_commits/no_mentions.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/src/handlers/check_commits/no_mentions.rs).
