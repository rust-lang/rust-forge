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

There are three optional values that can be specified in the table:

* `exclude_labels` --- A list of strings of label names to exclude.
  PRs with these labels set will not be checked for merge commits.

* `labels` --- A list of strings of label names to add.
  These labels will be set on the PR when merge commits are detected.

* `message` --- Override the default message posted for merge commits.
  The message will always be followed up with "The following commits are merge commits:" and then a list of the merge commits.

#### Default message

> There are merge commits (commits with multiple parents) in your changes. We have a [no merge policy](https://rustc-dev-guide.rust-lang.org/git.html#no-merge-policy) so these commits will need to be removed for this pull request to be merged.
> 
> You can start a rebase with the following commands:
> 
> ```shell-session
> $ # rebase
> $ git rebase -i master
> $ # delete any merge commits in the editor that appears
> $ git push --force-with-lease
> ```

## Example

```toml
[no-merges]
# PRs with the following labels will be skipped 
exclude_labels = ["rollup", "sync"]
# Add the following labels to PRs with merge commits
labels = ["has-merge-commits", "S-waiting-on-author"]
# Post the following warning message as a comment on PRs with merge commits
message = """
This repository does not allow merge commits.
Your PR cannot be merged until it is rebased.
"""
```

## Implementation

See [`src/handlers/no_merges.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/src/handlers/no_merges.rs).
