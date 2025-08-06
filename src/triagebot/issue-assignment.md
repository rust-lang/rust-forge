# Issue Assignment

The issue assignment commands allows any user to assign themselves to a GitHub issue.

## Usage

Issue assignment is done by entering one of these commands in a GitHub comment:

* `@rustbot claim` --- Assigns the issue to yourself.
* `@rustbot release-assignment` or `@rustbot unclaim` --- Removes the current assignee.
  Only the current assignee or a team member can release an assignment.
* `@rustbot assign @user` --- Assigns a specific user.
  Only team members can assign other users.

Due to GitHub restrictions, not all users can be directly assigned to an issue.
Only users with write permission to the repo, or rust-lang organization members can be directly assigned.
If triagebot is unable to directly assign the user, it will instead assign `@rustbot` and edit the top-level comment with a message that the issue has been claimed.

## Configuration

Issue assignment is enabled on a repository by the existence of the `[assign]` table in `triagebot.toml`:

```toml
[assign]
```

## Implementation

See [`parser/src/command/assign.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/parser/src/command/assign.rs) and [`src/handlers/assign.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/src/handlers/assign.rs).
