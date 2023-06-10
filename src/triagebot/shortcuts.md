# Shortcuts

Shortcuts are simple commands for performing common tasks.

## Usage

Shortcut commands can be issued by writing a GitHub comment as indicated below.

### ready

`@rustbot ready`

This indicates that a PR is ready for review.
This assigns the `S-waiting-on-review` label on the pull request and removes both `S-waiting-on-author` and `S-blocked` if present.

`@rustbot review` or `@rustbot reviewer` are aliases for `ready`.

### author

`@rustbot author`

This indicates that a PR is waiting on the author.
This assigns the `S-waiting-on-author` label on the pull request and removes both `S-waiting-on-review` and `S-blocked` if present.

### blocked

`@rustbot blocked`

This indicates that a PR is blocked on something.
This assigns the `S-blocked` label on the pull request and removes both `S-waiting-on-author` and `S-waiting-on-review` if present.

## Configuration

This feature is enabled on a repository by having a `[shortcut]` table in `triagebot.toml`:

```toml
[shortcut]
```

## Implementation

See [`parser/src/command/shortcut.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/parser/src/command/shortcut.rs) and
[`src/handlers/shortcut.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/src/handlers/shortcut.rs).
