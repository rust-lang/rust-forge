# Lock

The `lock`/`unlock` commands can be used to lock and unlock a GitHub issue or pull request.

## Usage

### Lock

To lock an issue or pull request, any rust-lang team member may enter the command:

```text
@rustbot lock
```

This will immediately lock the issue or PR.

### Unlock

To unlock an issue or pull request, any rust-lang team member may enter the command:

```text
@rustbot unlock
```

> [!NOTE]
> The triagebot Zulip `unlock` command can be used in case it's not possible
> for the team member to post a command.

## Configuration

This feature is enabled on a repository by having a `[lock]` table in `triagebot.toml`:

```toml
[lock]
```

## Implementation

See [`src/handlers/lock.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/src/handlers/lock.rs) and
[`parser/src/command/lock.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/parser/src/command/lock.rs).
