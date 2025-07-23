# Rendered link

Rendered links are simple hyperlinks that are automatically added (and updated) to a PR description by triagebot.

## Configuration

This feature is enabled on a repository by having a `[rendered-link]` table in `triagebot.toml`:

```toml
[rendered-link]
trigger-files = ["posts/"]
exclude-files = ["posts/SUMMARY.md"]
```

The `trigger-files` key configures which directories are watched for modification, with the `exclude-files` key to exclude files or directories.

The "Rendered link" will point to the matching file with the most modifications.

## Implementation

See [`src/handlers/rendered_link.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/src/handlers/rendered_link.rs).
