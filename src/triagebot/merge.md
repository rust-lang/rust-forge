# Merge 

The `merge` feature enables merging (and delegating the rights) to merge a PR in repositories that use the merge queue.

> Note: it currently does not work for repositories using bors or using direct PR merging, without a merge queue.

## Usage

### Merge

To merge a pull request, any authorized users (see below) may enter the command:

```text
@rustbot merge
```

This will enqueue the PR in the merge queue.

The command is only available for:
 - members with at least write permissions on the repository
 - users who have been delegated the rights to merge

### Delegation

#### Delegate to the author

To delegate the rights to merge the pull request to the PR author, any members with at least write permissions on
the repository may enter the command:

```text
@rustbot delegate+
```

or

```text
@rustbot delegate
```

and will post a message roughtly like this:

> :v: @User, you can now merge this pull request!
>
> If @reviewer told you to merge after making some further change, then please make that change and post `@rustbot merge`.

#### Delegate to a specific user

To delegate to another user, rather than the PR author, it's possible to specify a specific user:

```text
@rustbot delegate=@other_user
```

## Configuration

This feature is enabled on a repository by having a `[merge]` table in `triagebot.toml`:

```toml
[merge]
type = "merge-queue"
```

There are several optional keys that you can include:

- `type` --- what type of merging should be used, supported: `merge-queue`.

## Implementation

See [`src/handlers/merge.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/src/handlers/merge.rs).
