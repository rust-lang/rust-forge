# GitHub Actions created PR open/closer

This automation triggers an automatic close & reopen on PRs opened by the
`github-actions` user, i.e., from a GitHub actions job. This enables CI to run
on those PRs without needing a manual poke from some human.

## Configuration

This feature is enabled on a repository by having a `[bot-pull-requests]` table in `triagebot.toml`:

```toml
[bot-pull-requests]
```

## Implementation

See [`src/handlers/bot_pull_requests.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/src/handlers/bot_pull_requests.rs).
