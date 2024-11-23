# Merge Conflicts

The `merge-conflicts` feature detects if a Pull Request has a merge conflict, and will post a comment asking the author to resolve the conflicts.

## Usage

This is triggered automatically when a commit is made to a branch that causes existing, open PRs to have a merge conflict. The bot will post a comment to the PR that roughly looks like this:

> ☔ The latest upstream changes (possibly #152) made this pull request unmergeable. Please [resolve the merge conflicts](https://rustc-dev-guide.rust-lang.org/git.html#rebasing-and-conflicts).

Note that it may take a minute or so for the comments to be posted.

## Configuration

This feature is enabled on a repository by having a `[merge-conflicts]` table in `triagebot.toml`:

```toml
[merge-conflicts]
```

There are several optional keys that you can include:

- `remove` --- A list of labels to remove from the PR when a conflict is detected.
- `add` --- A list of labels to add to the PR when a conflict is detected.
- `unless` --- A list of labels that, if already present on the PR, will prevent triagebot from adding or removing labels.

## Example

```toml
[merge-conflicts]
remove = ['S-waiting-on-bors']
add = ['S-waiting-on-author']
unless = ['S-blocked', 'S-waiting-on-crater', 'S-waiting-on-team', 'S-waiting-on-review']
```

## Implementation

See [`src/handlers/merge_conflicts.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/src/handlers/merge_conflicts.rs).
