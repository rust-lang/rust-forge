# Undraft comment

GitHub doesn’t send an email notification when a pull request is undrafted (marked as ready for review).

This handler posts a comment saying that the PR has been undrafted, forcing GitHub to send an email notification to everyone subscribed to the PR.

## Configuration

This feature is enabled by having an empty `[undraft-comment]` table in `triagebot.toml`:

```toml
[undraft-comment]
```

## Implementation

See [`src/handlers/undraft_comment.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/src/handlers/undraft_comment.rs).
