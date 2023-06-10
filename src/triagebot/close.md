# Close

The close command can be used to close a GitHub issue or pull request.

## Usage

To close an issue or pull request, any rust-lang team member may enter the command:

```text
@rustbot close
```

This will immediately close the issue or PR.

## Configuration

This feature is enabled on a repository by having a `[close]` table in `triagebot.toml`:

```toml
[close]
```

## Implementation

See [`src/handlers/close.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/src/handlers/close.rs) and
[`parser/src/command/close.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/parser/src/command/close.rs).
