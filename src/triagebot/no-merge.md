# No Merge Policy

The [no-merge policy] informs users if they have merge commits in their pull request.
Some repositories prefer to only use a rebase-oriented workflow.

[no-merge policy]: https://rustc-dev-guide.rust-lang.org/git.html#keeping-things-up-to-date

## Usage

This is triggered automatically if a PR has merge commits.
Triagebot will post a comment on the PR if it detects merge commits.
The comment will explain the no-merge policy, and how the user can avoid merge commits.

## Configuration

This feature is enabled on a repository by having a `[no-merges]` table in `triagebot.toml`:

```toml
[no-merges]
```

## Implementation

See [`src/handlers/no_merges.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/src/handlers/no_merges.rs).
